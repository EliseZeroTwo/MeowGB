mod cpu;
mod interrupts;
mod joypad;
mod mapper;
mod memory;
mod ppu;
mod timer;

use interrupts::Interrupts;
use joypad::Joypad;
use mapper::Mapper;
use memory::Memory;
use ppu::Ppu;
use timer::Timer;

use self::cpu::Registers;

pub struct DmaState {
	pub base: u8,
	pub remaining_cycles: u8,
	pub remaining_delay: u8,
}

impl DmaState {
	pub fn new() -> Self {
		Self { base: 0, remaining_cycles: 0, remaining_delay: 0 }
	}

	pub fn init_request(&mut self, base: u8) {
		self.base = base;
		self.remaining_cycles = 0xA0;
		self.remaining_delay = 2;
	}
}

pub struct Gameboy {
	pub ppu: Ppu,
	memory: Memory,
	cartridge: Option<Box<dyn Mapper>>,
	interrupts: Interrupts,
	timer: Timer,
	pub registers: Registers,
	pub joypad: Joypad,
	pub dma: DmaState,
}

impl Gameboy {
	pub fn new(bootrom: [u8; 0x100]) -> Self {
		Self {
			memory: Memory::new(bootrom),
			cartridge: None,
			interrupts: Interrupts::new(),
			timer: Timer::new(),
			joypad: Joypad::new(),
			dma: DmaState::new(),
			ppu: Ppu::new(),
			registers: Registers::default(),
		}
	}

	pub fn tick(&mut self) -> bool {
		if self.timer.tick() {
			self.interrupts.write_if_timer(true);
		}

		cpu::tick_cpu(self);
		let redraw_requested = self.ppu.tick(&mut self.interrupts);
		self.tick_dma();
		redraw_requested
	}

	fn tick_dma(&mut self) {
		if self.dma.remaining_delay > 0 {
			self.dma.remaining_delay -= 1;
		} else if self.dma.remaining_cycles > 0 {
			let offset = 0xA0 - self.dma.remaining_cycles;

			let value = if self.dma.base <= 0x7F {
				match self.cartridge.as_ref() {
					Some(cart) => cart.read_rom_u8((self.dma.base as u16) << 8 | offset as u16),
					None => 0xFF,
				}
			} else if self.dma.base <= 0x9F {
				self.ppu.dma_read_vram(offset)
			} else {
				0xFF
			};

			self.ppu.dma_write_oam(offset, value);
			self.dma.remaining_cycles -= 1;
		}
	}

	fn cpu_read_io(&self, address: u16) -> u8 {
		match address {
			0xFF00 => self.joypad.cpu_read(),
			0xFF01..=0xFF02 => unimplemented!("Serial"),
			0xFF03 => 0, // Unused
			0xFF04 => self.timer.div,
			0xFF05 => self.timer.tima,
			0xFF06 => self.timer.tma,
			0xFF07 => self.timer.read_tac(),
			0xFF08..=0xFF0E => 0, // Unused
			0xFF0F => self.interrupts.interrupt_enable,
			0xFF10..=0xFF3F => unimplemented!("Sound IO"),
			0xFF40 => self.ppu.lcdc,
			0xFF41 => self.ppu.stat,
			0xFF42 => self.ppu.scy,
			0xFF43 => self.ppu.scx,
			0xFF44 => self.ppu.ly,
			0xFF45 => self.ppu.lyc,
			0xFF46 => self.dma.base,
			0xFF47..=0xFF49 => 0,
			0xFF4A => self.ppu.wy,
			0xFF4B => self.ppu.wx,
			0xFF4C..=0xFF4E => 0, // Unused
			0xFF4F => 0,          // CGB VRAM Bank Select
			0xFF50 => self.memory.bootrom_disabled as u8,
			0xFF51..=0xFF55 => 0, // CGB VRAM DMA
			0xFF56..=0xFF67 => 0, // Unused
			0xFF68..=0xFF69 => 0, // BJ/OBJ Palettes
			0xFF6A..=0xFF6F => 0, // Unused
			0xFF70 => 0,          // CGB WRAM Bank Select
			0xFF71..=0xFF7F => 0, // Unused
			_ => unreachable!("IO Read Invalid"),
		}
	}

	fn cpu_write_io(&mut self, address: u16, value: u8) {
		match address {
			0xFF00 => self.joypad.cpu_write(value),
			0xFF01..=0xFF02 => unimplemented!("Serial"),
			0xFF03 => {} // Unused
			0xFF04 => self.timer.div = value,
			0xFF05 => self.timer.tima = value,
			0xFF06 => self.timer.tma = value,
			0xFF07 => self.timer.write_tac(value),
			0xFF08..=0xFF0E => {} // Unused
			0xFF0F => self.interrupts.interrupt_enable = value & 0b1_1111,
			0xFF10..=0xFF3F => unimplemented!("Sound IO"),
			0xFF40 => self.ppu.lcdc = value,
			0xFF41 => self.ppu.cpu_write_stat(value),
			0xFF42 => self.ppu.scy = value,
			0xFF43 => self.ppu.scx = value,
			0xFF44 => {} // LY is read only
			0xFF45 => self.ppu.lyc = value,
			0xFF46 => {
				if self.dma.remaining_cycles == 0 {
					self.dma.init_request(value);
				}
			}
			0xFF47..=0xFF49 => {}
			0xFF4A => self.ppu.wy = value,
			0xFF4B => self.ppu.wx = value,
			0xFF4C..=0xFF4E => {} // Unused
			0xFF4F => {}          // CGB VRAM Bank Select
			0xFF50 => {
				if value & 0b1 == 1 {
					self.memory.bootrom_disabled = true;
				}
			}
			0xFF51..=0xFF55 => {} // CGB VRAM DMA
			0xFF56..=0xFF67 => {} // Unused
			0xFF68..=0xFF69 => {} // CGB BG/OBJ Palettes
			0xFF6A..=0xFF6F => {} // Unused
			0xFF70 => {}          // CGB WRAM Bank Select
			0xFF71..=0xFF7F => {} // Unused
			_ => unreachable!("IO Read Invalid"),
		}
	}

	pub fn cpu_read_u8(&mut self, address: u16) {
		assert!(!self.registers.mem_op_happened);
		assert!(self.registers.mem_read_hold.is_none());
		self.registers.mem_op_happened = true;
		self.registers.mem_read_hold = Some(if self.dma.remaining_cycles == 0 {
			match address {
				0..=0xFF if !self.memory.bootrom_disabled => self.memory.bootrom[address as usize],
				0..=0x7FFF => match self.cartridge.as_ref() {
					Some(mapper) => mapper.read_rom_u8(address),
					None => 0,
				},
				0x8000..=0x9FFF => self.ppu.cpu_read_vram(address),
				0xA000..=0xBFFF => match self.cartridge.as_ref() {
					Some(mapper) => mapper.read_eram_u8(address),
					None => 0,
				},
				0xC000..=0xDFFF => self.memory.wram[address as usize - 0xC000],
				0xE000..=0xFDFF => self.memory.wram[address as usize - 0xE000],
				0xFE00..=0xFE9F => self.ppu.cpu_read_oam(address),
				0xFEA0..=0xFEFF => 0,
				0xFF00..=0xFF7F => self.cpu_read_io(address),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80],
				0xFFFF => self.interrupts.interrupt_enable,
			}
		} else {
			match address {
				0..=0xFEFF => 0,
				0xFF00..=0xFF7F => self.cpu_read_io(address),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80],
				0xFFFF => self.interrupts.interrupt_enable,
			}
		})
	}

	pub fn cpu_write_u8(&mut self, address: u16, value: u8) {
		assert!(!self.registers.mem_op_happened);
		self.registers.mem_op_happened = true;
		if self.dma.remaining_cycles == 0 {
			match address {
				0..=0xFF if !self.memory.bootrom_disabled => {}
				0..=0x7FFF => {
					if let Some(mapper) = self.cartridge.as_mut() {
						mapper.write_rom_u8(address, value)
					}
				}
				0x8000..=0x9FFF => self.ppu.cpu_write_vram(address, value),
				0xA000..=0xBFFF => {
					if let Some(mapper) = self.cartridge.as_mut() {
						mapper.write_eram_u8(address, value)
					}
				}
				0xC000..=0xDFFF => self.memory.wram[address as usize - 0xC000] = value,
				0xE000..=0xFDFF => self.memory.wram[address as usize - 0xE000] = value,
				0xFE00..=0xFE9F => self.ppu.cpu_write_oam(address, value),
				0xFEA0..=0xFEFF => {}
				0xFF00..=0xFF7F => self.cpu_write_io(address, value),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80] = value,
				0xFFFF => self.interrupts.interrupt_enable = value & 0b1_1111,
			}
		} else {
			match address {
				0..=0xFEFF => {}
				0xFF00..=0xFF7F => self.cpu_write_io(address, value),
				0xFF80..=0xFFFE => self.memory.hram[address as usize - 0xFF80] = value,
				0xFFFF => self.interrupts.interrupt_enable = value & 0b1_1111,
			}
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
