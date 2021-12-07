mod cpu;
mod interrupts;
mod joypad;
mod mapper;
mod memory;
mod ppu;
mod serial;
mod sound;
mod timer;

use interrupts::Interrupts;
use joypad::Joypad;
use mapper::Mapper;
use memory::Memory;
use ppu::Ppu;
use timer::Timer;

use self::{cpu::Registers, mapper::NoMBC, serial::Serial, sound::Sound};

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
	pub serial: Serial,
	pub dma: DmaState,
	pub sound: Sound,

	pub single_step: bool,
	pub breakpoints: [bool; u16::MAX as usize + 1],
	pub mem_read_breakpoints: [bool; u16::MAX as usize + 1],
	pub mem_write_breakpoints: [bool; u16::MAX as usize + 1],
	trigger_bp: bool,
}

impl Gameboy {
	pub fn new(bootrom: [u8; 0x100]) -> Self {
		Self {
			memory: Memory::new(bootrom),
			cartridge: None,
			interrupts: Interrupts::new(),
			timer: Timer::new(),
			joypad: Joypad::new(),
			serial: Serial::new(),
			dma: DmaState::new(),
			ppu: Ppu::new(),
			registers: Registers::default(),
			sound: Sound::default(),
			single_step: false,
			breakpoints: [false; u16::MAX as usize + 1],
			mem_read_breakpoints: [false; u16::MAX as usize + 1],
			mem_write_breakpoints: [false; u16::MAX as usize + 1],
			trigger_bp: false,
		}
	}

	fn log_next_opcode(&self) {
		let op = self.internal_cpu_read_u8(self.registers.pc);
		if op == 0xCB {
			let op = self.internal_cpu_read_u8(self.registers.pc.overflowing_add(1).0);
			log::info!(
				"Executing opcode @ {:#X} (prefixed) (cycle {}): {:#X}",
				self.registers.pc,
				self.registers.cycle,
				op
			);
		} else {
			log::info!(
				"Executing opcode @ {:#X} (cycle {}): {:#X}",
				self.registers.pc,
				self.registers.cycle,
				op
			);
		}
	}

	pub fn load_cartridge(&mut self, bytes: Vec<u8>) {
		match bytes[0x147] {
			0 => self.cartridge = Some(Box::new(NoMBC::new(bytes))),
			other => unimplemented!("Cartidge type: {:#X}", other),
		}
	}

	fn log_state(&self) {
		log::info!("-- Registers --\nAF: {:04X}\nBC: {:04X}\nDE: {:04X}\nHL: {:04X}\nSP: {:04X}\nPC: {:04X}\nZero: {}\nSubtract: {}\nHalf-Carry: {}\nCarry: {}", self.registers.get_af(), self.registers.get_bc(), self.registers.get_de(), self.registers.get_hl(), self.registers.get_sp(), self.registers.pc, self.registers.get_zero(), self.registers.get_subtract(), self.registers.get_half_carry(), self.registers.get_carry());
	}

	pub fn tick(&mut self) -> bool {
		if self.breakpoints[self.registers.pc as usize] && !self.single_step {
			self.single_step = true;
			log::info!("Breakpoint hit @ {:#X}", self.registers.pc);
		}

		if self.trigger_bp || (self.single_step && self.registers.cycle == 0) {
			self.trigger_bp = false;
			self.single_step = true;
			let mut input = String::new();
			let mut exit = true;
			match std::io::stdin().read_line(&mut input) {
				Ok(_) => {
					let lower = input.trim_end().to_lowercase();
					let (lhs, rhs) = lower.split_once(' ').unwrap_or_else(|| (lower.as_str(), ""));
					match lhs {
						"read" => match u16::from_str_radix(rhs, 16) {
							Ok(address) => {
								let res = self.internal_cpu_read_u8(address);
								log::info!("{:#X}: {:#X} ({:#b})", address, res, res);
							}
							Err(_) => log::error!("Failed to parse input as hex u16 (f.ex 420C)"),
						},
						"regs" => self.log_state(),
						"op" => {
							self.log_next_opcode();
						}
						"bp" => match u16::from_str_radix(rhs, 16) {
							Ok(address) => {
								let bp = &mut self.breakpoints[address as usize];
								*bp = !*bp;
								match *bp {
									true => log::info!("Set breakpoint @ {:#X}", address),
									false => log::info!("Cleared breakpoint @ {:#X}", address),
								}
							}
							Err(_) => log::error!("Failed to parse input as hex u16 (f.ex 420C)"),
						},
						"bpr" => match u16::from_str_radix(rhs, 16) {
							Ok(address) => {
								let bp = &mut self.mem_read_breakpoints[address as usize];
								*bp = !*bp;
								match *bp {
									true => log::info!("Set breakpoint on read @ {:#X}", address),
									false => {
										log::info!("Cleared breakpoint on read @ {:#X}", address)
									}
								}
							}
							Err(_) => log::error!("Failed to parse input as hex u16 (f.ex 420C)"),
						},
						"bpw" => match u16::from_str_radix(rhs, 16) {
							Ok(address) => {
								let bp = &mut self.mem_write_breakpoints[address as usize];
								*bp = !*bp;
								match *bp {
									true => log::info!("Set breakpoint on write @ {:#X}", address),
									false => {
										log::info!("Cleared breakpoint on write @ {:#X}", address)
									}
								}
							}
							Err(_) => log::error!("Failed to parse input as hex u16 (f.ex 420C)"),
						},
						"c" => {
							self.single_step = false;
							log::info!("Continuing");
							exit = false;
						}
						"s" | "step" => {
							self.log_next_opcode();
							exit = false;
						}
						"ls" => {
							self.log_state();
							exit = false;
						}
						"dumpbgtiles" => {
							self.ppu.dump_bg_tiles();
						}
						"dumpfb" => {
							println!("Written to: {}", self.ppu.dump_fb());
						}
						"dumpvram" => {
							for x in 0..0x200 {
								if x % 0x10 == 0 {
									print!("\n{:X}: ", 0x8000 + x)
								}

								let mem_val = self.ppu.vram[x];
								print!("{:02X} ", mem_val);
							}
							println!();
						}
						"dumptilemap" => {
							let base = match (self.ppu.lcdc >> 3) & 0b1 == 1 {
								true => 0x1C00,
								false => 0x1800,
							};

							for x in 0..0x400 {
								if x % 0x10 == 0 {
									print!("\n{:X}: ", 0x8000 + base + x)
								}

								let mem_val = self.ppu.vram[base + x];
								print!("{:02X} ", mem_val);
							}
							println!();
						}
						_ => {}
					}
				}
				Err(stdin_err) => panic!("Failed to lock stdin: {:?}", stdin_err),
			}

			if exit {
				return false;
			}
		}
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
			0xFF01 => self.serial.sb,
			0xFF02 => self.serial.sc,
			0xFF03 => 0, // Unused
			0xFF04 => self.timer.div,
			0xFF05 => self.timer.tima,
			0xFF06 => self.timer.tma,
			0xFF07 => self.timer.read_tac(),
			0xFF08..=0xFF0E => 0, // Unused
			0xFF0F => self.interrupts.interrupt_enable,
			0xFF10 => self.sound.nr10,
			0xFF11 => self.sound.nr11,
			0xFF12 => self.sound.nr12,
			0xFF13 => self.sound.nr13,
			0xFF14 => self.sound.nr14,
			0xFF15 => 0,
			0xFF16 => self.sound.nr21,
			0xFF17 => self.sound.nr22,
			0xFF18 => self.sound.nr23,
			0xFF19 => self.sound.nr24,
			0xFF1A => self.sound.nr30,
			0xFF1B => self.sound.nr31,
			0xFF1C => self.sound.nr32,
			0xFF1D => self.sound.nr33,
			0xFF1E => self.sound.nr34,
			0xFF1F => 0,
			0xFF20 => self.sound.nr41,
			0xFF21 => self.sound.nr42,
			0xFF22 => self.sound.nr43,
			0xFF23 => self.sound.nr44,
			0xFF24 => self.sound.nr50,
			0xFF25 => self.sound.nr51,
			0xFF26 => self.sound.nr52,
			0xFF27..=0xFF2F => 0,
			0xFF30..=0xFF3F => self.sound.wave_pattern_ram[address as usize - 0xFF30],
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
			0xFF01 => self.serial.sb = value,
			0xFF02 => self.serial.sc = value,
			0xFF03 => {} // Unused
			0xFF04 => self.timer.div = value,
			0xFF05 => self.timer.tima = value,
			0xFF06 => self.timer.tma = value,
			0xFF07 => self.timer.write_tac(value),
			0xFF08..=0xFF0E => {} // Unused
			0xFF0F => self.interrupts.interrupt_enable = value & 0b1_1111,
			0xFF10 => self.sound.nr10 = value,
			0xFF11 => self.sound.nr11 = value,
			0xFF12 => self.sound.nr12 = value,
			0xFF13 => self.sound.nr13 = value,
			0xFF14 => self.sound.nr14 = value,
			0xFF15 => {}
			0xFF16 => self.sound.nr21 = value,
			0xFF17 => self.sound.nr22 = value,
			0xFF18 => self.sound.nr23 = value,
			0xFF19 => self.sound.nr24 = value,
			0xFF1A => self.sound.nr30 = value,
			0xFF1B => self.sound.nr31 = value,
			0xFF1C => self.sound.nr32 = value,
			0xFF1D => self.sound.nr33 = value,
			0xFF1E => self.sound.nr34 = value,
			0xFF1F => {}
			0xFF20 => self.sound.nr41 = value,
			0xFF21 => self.sound.nr42 = value,
			0xFF22 => self.sound.nr43 = value,
			0xFF23 => self.sound.nr44 = value,
			0xFF24 => self.sound.nr50 = value,
			0xFF25 => self.sound.nr51 = value,
			0xFF26 => self.sound.nr52 = value,
			0xFF27..=0xFF2F => {}
			0xFF30..=0xFF3F => self.sound.wave_pattern_ram[address as usize - 0xFF30] = value,
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

	fn internal_cpu_read_u8(&self, address: u16) -> u8 {
		if self.dma.remaining_cycles == 0 {
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
		}
	}

	pub fn cpu_read_u8(&mut self, address: u16) {
		assert!(!self.registers.mem_op_happened);
		assert!(self.registers.mem_read_hold.is_none());
		self.registers.mem_op_happened = true;

		if self.mem_read_breakpoints[address as usize] {
			self.trigger_bp = true;
			log::info!("Triggered read bp @ {:#X}", address);
		}

		self.registers.mem_read_hold = Some(self.internal_cpu_read_u8(address));
	}

	pub fn cpu_write_u8(&mut self, address: u16, value: u8) {
		assert!(!self.registers.mem_op_happened);
		self.registers.mem_op_happened = true;

		if self.mem_write_breakpoints[address as usize] {
			self.trigger_bp = true;
			log::info!("Triggered write bp @ {:#X} (value: {:#02X})", address, value);
		}

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
