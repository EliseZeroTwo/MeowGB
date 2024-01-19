pub mod cpu;
pub mod dma;
pub mod interrupts;
pub mod joypad;
pub mod mapper;
pub mod memory;
pub mod ppu;
pub mod serial;
pub mod sound;
pub mod timer;

use interrupts::Interrupts;
use joypad::Joypad;
use mapper::Mapper;
use memory::Memory;
use ppu::Ppu;
use timer::Timer;

use self::{
	cpu::Registers,
	dma::DmaState,
	mapper::{mbc1::MBC1, NoMBC},
	serial::{Serial, SerialWriter},
	sound::Sound,
};
#[cfg(feature = "instr-dbg")]
use crate::ringbuffer::RingBuffer;

pub type GenericCartridge = dyn Mapper + Send + Sync;

pub struct Gameboy<S: SerialWriter> {
	pub ppu: Ppu,
	pub memory: Memory,
	pub cartridge: Option<Box<GenericCartridge>>,
	pub interrupts: Interrupts,
	pub timer: Timer,
	pub registers: Registers,
	pub joypad: Joypad,
	pub serial: Serial<S>,
	pub dma: DmaState,
	pub sound: Sound,

	pub halt: bool,
	pub halt_bug: bool,
	pub used_halt_bug: bool,
	pub stop: bool,

	pub tick_count: u8,

	pub last_read: Option<(u16, u8)>,
	pub last_write: Option<(u16, u8)>,

	#[cfg(feature = "instr-dbg")]
	pub pc_history: RingBuffer<u16, 0x1000>,
}

impl<S: SerialWriter> Gameboy<S> {
	pub fn new(serial_writer: S, rom: Option<Vec<u8>>) -> Self {
		Self::new_with_cartridge(serial_writer, rom.map(Self::parse_rom))
	}

	pub fn new_with_cartridge(serial_writer: S, cartridge: Option<Box<GenericCartridge>>) -> Self {
		let mut out = Self {
			memory: Memory::new(),
			cartridge,
			interrupts: Interrupts::new(),
			timer: Timer::new(),
			joypad: Joypad::new(),
			serial: Serial::new(serial_writer),
			dma: DmaState::new(),
			ppu: Ppu::new(),
			registers: Registers::default(),
			sound: Sound::new(),
			halt: false,
			halt_bug: false,
			used_halt_bug: false,
			stop: false,
			tick_count: 0,
			last_read: None,
			last_write: None,
			#[cfg(feature = "instr-dbg")]
			pc_history: RingBuffer::new(),
		};

		out.run_bootrom();
		out.registers.set_post_rom();

		out
	}

	pub fn run_bootrom(&mut self) {
		macro_rules! push8 {
			($byte:expr) => {
				self.registers.sp = self.registers.sp.overflowing_sub(1).0;
				self.debug_write_u8(self.registers.sp, $byte);
			};
		}

		macro_rules! push16 {
			($byte:expr) => {
				push8!(($byte >> 8) as u8);
				push8!($byte as u8);
			};
		}

		macro_rules! pop8 {
			() => {
				{
					let res = self.debug_read_u8(self.registers.sp);
					self.registers.sp = self.registers.sp.overflowing_add(1).0;
					res
				}
			};
		}

		macro_rules! pop16 {
			() => {
				(pop8!() as u16) | ((pop8!() as u16) << 8)
			};
		}

		macro_rules! rl {
			($reg:ident) => {
				{
					let new_carry = self.registers.$reg >> 7 == 1;
					self.registers.$reg <<= 1;

					if self.registers.get_carry() {
						self.registers.$reg |= 1;
					}

					self.registers.set_carry(new_carry);
				}
			};
		}

		self.registers.sp = 0xFFFE;
		
		// Clear VRAM
		self.registers.a = 0;
		let mut address = 0x9FFF;
		while address >= 0x8000 {
			self.debug_write_u8(address, 0);
			address -= 1;
		}

		// // Configure Audio
		// self.registers.set_hl(0xFF26);
		// self.registers.c = 0x11;
		// self.registers.a = 0x80;
		// let hl_content = self.registers.get_hl();
		// self.debug_write_u8(hl_content, self.registers.a);
		// self.registers.set_hl(hl_content - 1);
		// self.debug_write_u8(0xFF00 | self.registers.c as u16, self.registers.a);
		// self.registers.c += 1;
		// self.registers.a = 0xF3;
		// self.debug_write_u8(0xFF00 | self.registers.c as u16, self.registers.a);
		// let hl_content = self.registers.get_hl();
		// self.debug_write_u8(hl_content, self.registers.a);
		// self.registers.set_hl(hl_content - 1);
		// self.registers.a = 0x77;
		// self.debug_write_u8(hl_content, self.registers.a);
		
		// Configure Background Palette
		self.registers.a = 0xFC;
		self.debug_write_u8(0xFF47, self.registers.a);

		// Load logo data from cartridge into VRAM
		self.registers.set_de(0x0104);
		self.registers.set_hl(0x8010);

		loop {
			self.registers.a = self.debug_read_u8(self.registers.get_de());
			
			self.registers.c = self.registers.a;
			for _ in 0..2 {
				self.registers.b = 4;
				while self.registers.b != 0 {
					push16!(self.registers.get_bc());
					rl!(c);
					rl!(a);
					let bc = pop16!();
					self.registers.set_bc(bc);
					rl!(c);
					rl!(a);
					self.registers.b -= 1;
				}
				self.debug_write_u8(self.registers.get_hl(), self.registers.a);
				self.registers.set_hl(self.registers.get_hl() + 2);
				self.debug_write_u8(self.registers.get_hl(), self.registers.a);
				self.registers.set_hl(self.registers.get_hl() + 2);
			}
			
			self.registers.set_de(self.registers.get_de() + 1);
			self.registers.a = self.registers.e;

			assert!(self.registers.a <= 0x34);
			if self.registers.a == 0x34 {
				break;
			}
		}

		// Extra 8 bytes into VRAM from BROM
		const EXTRA_VRAM_DATA: [u8; 8] = [0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C];
		for value in EXTRA_VRAM_DATA {
			self.registers.a = value;
			self.debug_write_u8(self.registers.get_hl(), self.registers.a);
			self.registers.set_hl(self.registers.get_hl() + 2);
		}
		self.registers.set_de(0xE0);
		self.registers.b = 0;

		// BG Tilemap
		self.registers.a = 0x19;
		self.debug_write_u8(0x9910, self.registers.a);
		self.registers.set_hl(0x992F);

		'outer: loop {
			self.registers.c = 0xc;
			while self.registers.c != 0 {
				self.registers.a -= 1;
				if self.registers.a == 0 {
					break 'outer;
				}

				self.debug_write_u8(self.registers.get_hl(), self.registers.a);
				self.registers.set_hl(self.registers.get_hl() - 1);
				self.registers.c -= 1;
			}
			self.registers.l = 0xf;
		}
	}

	fn parse_rom(bytes: Vec<u8>) -> Box<GenericCartridge> {
		if bytes.len() < 0x150 {
			panic!("Bad cartridge (len < 0x150)");
		}
		match bytes[0x147] {
			0 => Box::new(NoMBC::new(bytes)),
			1 => Box::new(MBC1::new(bytes)),
			2 => Box::new(MBC1::new(bytes)),
			3 => Box::new(MBC1::new(bytes)),
			other => unimplemented!("Cartidge type: {:#X}", other),
		}
	}

	pub fn tick_4(&mut self) -> bool {
		let mut request_redraw = false;
		for _ in 0..4 {
			let t_request_redraw = self.tick();
			request_redraw |= t_request_redraw;
		}
		request_redraw
	}

	pub fn tick(&mut self) -> bool {
		if self.tick_count == 0 {
			self.dma.tick_dma(&mut self.ppu, &self.memory, self.cartridge.as_deref());
			cpu::tick_cpu(self);
			let redraw_requested = self.ppu.tick(&self.dma, &mut self.interrupts);
			self.serial.tick(&mut self.interrupts);
			self.timer.tick(&mut self.interrupts);

			self.tick_count += 1;
			redraw_requested
		} else {
			let redraw_requested = self.ppu.tick(&self.dma, &mut self.interrupts);
			self.timer.tick(&mut self.interrupts);
			self.tick_count += 1;
			self.tick_count %= 4;
			redraw_requested
		}
	}

	fn cpu_read_io(&self, address: u16) -> u8 {
		match address {
			0xFF00 => self.joypad.cpu_read(),
			0xFF01 => self.serial.sb,
			0xFF02 => self.serial.get_sc(),
			0xFF03 => 0xFF, // Unused
			0xFF04 => self.timer.read_div(),
			0xFF05 => self.timer.read_tima(),
			0xFF06 => self.timer.read_tma(),
			0xFF07 => self.timer.read_tac(),
			0xFF08..=0xFF0E => 0xFF, // Unused
			0xFF0F => self.interrupts.interrupt_flag,
			0xFF10 => self.sound.nr10,
			0xFF11 => self.sound.nr11,
			0xFF12 => self.sound.nr12,
			0xFF13 => self.sound.nr13,
			0xFF14 => self.sound.nr14,
			0xFF15 => 0xFF,
			0xFF16 => self.sound.nr21,
			0xFF17 => self.sound.nr22,
			0xFF18 => self.sound.nr23,
			0xFF19 => self.sound.nr24,
			0xFF1A => self.sound.nr30,
			0xFF1B => self.sound.nr31,
			0xFF1C => self.sound.nr32,
			0xFF1D => self.sound.nr33,
			0xFF1E => self.sound.nr34,
			0xFF1F => 0xFF,
			0xFF20 => self.sound.nr41,
			0xFF21 => self.sound.nr42,
			0xFF22 => self.sound.nr43,
			0xFF23 => self.sound.nr44,
			0xFF24 => self.sound.nr50,
			0xFF25 => self.sound.nr51,
			0xFF26 => self.sound.nr52,
			0xFF27..=0xFF2F => 0xFF,
			0xFF30..=0xFF3F => self.sound.wave_pattern_ram[address as usize - 0xFF30],
			0xFF40 => self.ppu.registers.lcdc,
			0xFF41 => self.ppu.get_stat(),
			0xFF42 => self.ppu.registers.scy,
			0xFF43 => self.ppu.registers.scx,
			0xFF44 => self.ppu.registers.ly,
			0xFF45 => self.ppu.registers.lyc,
			0xFF46 => self.dma.base,
			0xFF47 => self.ppu.bgp.value(),
			0xFF48 => self.ppu.obp[0].value(),
			0xFF49 => self.ppu.obp[1].value(),
			0xFF4A => self.ppu.registers.wy,
			0xFF4B => self.ppu.registers.wx,
			0xFF4C..=0xFF4E => 0xFF, // Unused
			0xFF4F => 0xFF,          // CGB VRAM Bank Select
			0xFF50 => self.memory.get_bootrom_disabled(),
			0xFF51..=0xFF55 => 0xFF, // CGB VRAM DMA
			0xFF56..=0xFF67 => 0xFF, // Unused
			0xFF68..=0xFF69 => 0xFF, // BJ/OBJ Palettes
			0xFF6A..=0xFF6F => 0xFF, // Unused
			0xFF70 => 0xFF,          // CGB WRAM Bank Select
			0xFF71..=0xFF7F => 0xFF, // Unused
			_ => unreachable!("IO Read Invalid"),
		}
	}

	fn cpu_write_io(&mut self, address: u16, value: u8) {
		match address {
			0xFF00 => self.joypad.cpu_write(value),
			0xFF01 => self.serial.sb = value,
			0xFF02 => self.serial.set_sc(value),
			0xFF03 => {} // Unused
			0xFF04 => self.timer.write_div(),
			0xFF05 => self.timer.write_tima(value),
			0xFF06 => self.timer.write_tma(value),
			0xFF07 => self.timer.write_tac(value),
			0xFF08..=0xFF0E => {} // Unused
			0xFF0F => self.interrupts.interrupt_flag = value | !0b1_1111,
			0xFF10 => {} //self.sound.nr10 = value, - Unwritable on DMG
			0xFF11 => self.sound.nr11 = value,
			0xFF12 => self.sound.nr12 = value,
			0xFF13 => self.sound.nr13 = value,
			0xFF14 => self.sound.nr14 = value,
			0xFF15 => {}
			0xFF16 => self.sound.nr21 = value,
			0xFF17 => self.sound.nr22 = value,
			0xFF18 => self.sound.nr23 = value,
			0xFF19 => self.sound.nr24 = value,
			0xFF1A => {} //self.sound.nr30 = value, - Unwritable on DMG
			0xFF1B => self.sound.nr31 = value,
			0xFF1C => {} //self.sound.nr32 = value, - Unwritable on DMG
			0xFF1D => self.sound.nr33 = value,
			0xFF1E => self.sound.nr34 = value,
			0xFF1F => {}
			0xFF20 => {} //self.sound.nr41 = value, - Unwritable on DMG
			0xFF21 => self.sound.nr42 = value,
			0xFF22 => self.sound.nr43 = value,
			0xFF23 => {} //self.sound.nr44 = value, - Unwritable on DMG
			0xFF24 => self.sound.nr50 = value,
			0xFF25 => self.sound.nr51 = value,
			0xFF26 => {} //self.sound.nr52 = value, - Unwritable on DMG
			0xFF27..=0xFF2F => {}
			0xFF30..=0xFF3F => self.sound.wave_pattern_ram[address as usize - 0xFF30] = value,
			0xFF40 => {
				let old_value = self.ppu.registers.lcdc;
				self.ppu.registers.lcdc = value;

				if value >> 7 == 0 && old_value >> 7 == 1 {
					self.ppu.stop();
				} else if value >> 7 == 1 && old_value >> 7 == 0 {
					self.ppu.start(&mut self.interrupts);
				}
			}
			0xFF41 => self.ppu.set_stat(&mut self.interrupts, value),
			0xFF42 => self.ppu.registers.scy = value,
			0xFF43 => self.ppu.registers.scx = value,
			0xFF44 => {} // LY is read only
			0xFF45 => self.ppu.set_lyc(&mut self.interrupts, value),
			0xFF46 => self.dma.init_request(value),
			0xFF47 => self.ppu.bgp.write_bgp(value),
			0xFF48 => self.ppu.obp[0].write_obp(value),
			0xFF49 => self.ppu.obp[1].write_obp(value),
			0xFF4A => self.ppu.registers.wy = value,
			0xFF4B => self.ppu.registers.wx = value,
			0xFF4C..=0xFF4E => {} // Unused
			0xFF4F => {}          // CGB VRAM Bank Select
			0xFF50 => {} // BROM lockout
			0xFF51..=0xFF55 => {} // CGB VRAM DMA
			0xFF56..=0xFF67 => {} // Unused
			0xFF68..=0xFF69 => {} // CGB BG/OBJ Palettes
			0xFF6A..=0xFF6F => {} // Unused
			0xFF70 => {}          // CGB WRAM Bank Select
			0xFF71..=0xFF7F => {} // Unused
			_ => unreachable!("IO Read Invalid"),
		}
	}

	pub fn dump_memory(&self) -> [u8; 0xFFFF] {
		let mut out = [0u8; 0xFFFF];

		for address in 0..0xFFFF {
			out[address as usize] = self.debug_read_u8(address);
		}

		out
	}

	/// Warning: This bypasses the memory bus and only exists for
	/// debugging/testing purposes
	pub fn debug_read_u8(&self, address: u16) -> u8 {
		match address {
			0..=0x7FFF => match self.cartridge.as_ref() {
				Some(mapper) => mapper.read_rom_u8(address),
				None => 0xFF,
			},
			0x8000..=0x9FFF => self.ppu.cpu_read_vram(address),
			0xA000..=0xBFFF => match self.cartridge.as_ref() {
				Some(mapper) => mapper.read_eram_u8(address - 0xA000),
				None => 0xFF,
			},
			0xC000..=0xDFFF => self.memory.wram[address as usize - 0xC000],
			0xE000..=0xFDFF => self.memory.wram[address as usize - 0xE000],
			0xFE00..=0xFE9F => self.ppu.cpu_read_oam(address),
			0xFEA0..=0xFEFF => 0,
			0xFF00..=0xFF7F => self.cpu_read_io(address),
			0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80],
			0xFFFF => self.interrupts.interrupt_enable,
		}
	}

	/// Warning: This bypasses the memory bus and only exists for
	/// debugging/testing purposes
	#[allow(unused)]
	pub fn debug_write_u8(&mut self, address: u16, value: u8) {
		match address {
			0..=0x7FFF => {
				if let Some(mapper) = self.cartridge.as_mut() {
					mapper.write_rom_u8(address, value)
				}
			}
			0x8000..=0x9FFF => self.ppu.cpu_write_vram(address, value),
			0xA000..=0xBFFF => {
				if let Some(mapper) = self.cartridge.as_mut() {
					mapper.write_eram_u8(address - 0xA000, value)
				}
			}
			0xC000..=0xDFFF => self.memory.wram[address as usize - 0xC000] = value,
			0xE000..=0xFDFF => self.memory.wram[address as usize - 0xE000] = value,
			0xFE00..=0xFE9F => self.ppu.cpu_write_oam(address, value),
			0xFEA0..=0xFEFF => {}
			0xFF00..=0xFF7F => self.cpu_write_io(address, value),
			0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80] = value,
			0xFFFF => self.interrupts.cpu_set_interrupt_enable(value),
		}
	}

	pub fn cpu_read_u8(&mut self, address: u16) {
		self.cpu_read_u8_internal(address, false);
	}

	pub fn cpu_read_u8_internal(&mut self, address: u16, is_next_pc: bool) {
		assert!(!self.registers.mem_op_happened);
		assert!(self.registers.mem_read_hold.is_none());
		self.registers.mem_op_happened = true;
		let value = match self.dma.is_conflict(address) {
			true => match address {
				0..=0xFDFF => self.dma.read_next_byte(&self.ppu, &self.memory, self.cartridge.as_deref()),
				0xFE00..=0xFEFF => 0xFF,
				0xFF00..=0xFF7F => self.cpu_read_io(address),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80],
				0xFFFF => self.interrupts.interrupt_enable,
			},
			false => match address {
				0..=0x7FFF => match self.cartridge.as_ref() {
					Some(mapper) => mapper.read_rom_u8(address),
					None => 0xFF,
				},
				0x8000..=0x9FFF => self.ppu.cpu_read_vram(address),
				0xA000..=0xBFFF => match self.cartridge.as_ref() {
					Some(mapper) => mapper.read_eram_u8(address - 0xA000),
					None => 0xFF,
				},
				0xC000..=0xDFFF => self.memory.wram[address as usize - 0xC000],
				0xE000..=0xFDFF => self.memory.wram[address as usize - 0xE000],
				0xFE00..=0xFE9F => self.ppu.cpu_read_oam(address),
				0xFEA0..=0xFEFF => 0,
				0xFF00..=0xFF7F => self.cpu_read_io(address),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80],
				0xFFFF => self.interrupts.interrupt_enable,
			},
		};
		if !is_next_pc {
			self.last_read = Some((address, value));
		}
		self.registers.mem_read_hold = Some(value);
	}

	pub fn cpu_write_u8(&mut self, address: u16, value: u8) {
		assert!(!self.registers.mem_op_happened);
		self.registers.mem_op_happened = true;
		self.last_write = Some((address, value));

		match self.dma.is_conflict(address) {
			true => match address {
				0..=0xFEFF => {}
				0xFF00..=0xFF7F => self.cpu_write_io(address, value),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80] = value,
				0xFFFF => self.interrupts.cpu_set_interrupt_enable(value),
			},
			false => match address {
				0..=0x7FFF => {
					if let Some(mapper) = self.cartridge.as_mut() {
						mapper.write_rom_u8(address, value)
					}
				}
				0x8000..=0x9FFF => self.ppu.cpu_write_vram(address, value),
				0xA000..=0xBFFF => {
					if let Some(mapper) = self.cartridge.as_mut() {
						mapper.write_eram_u8(address - 0xA000, value)
					}
				}
				0xC000..=0xDFFF => self.memory.wram[address as usize - 0xC000] = value,
				0xE000..=0xFDFF => self.memory.wram[address as usize - 0xE000] = value,
				0xFE00..=0xFE9F => self.ppu.cpu_write_oam(address, value),
				0xFEA0..=0xFEFF => {}
				0xFF00..=0xFF7F => self.cpu_write_io(address, value),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80] = value,
				0xFFFF => self.interrupts.cpu_set_interrupt_enable(value),
			},
		}
	}

	pub fn cpu_push_stack(&mut self, byte: u8) {
		self.registers.sp = self.registers.sp.overflowing_sub(1).0;
		self.cpu_write_u8(self.registers.sp, byte)
	}

	pub fn cpu_pop_stack(&mut self) {
		self.cpu_read_u8(self.registers.sp);
		self.registers.sp = self.registers.sp.overflowing_add(1).0;
	}
}
