use super::{mapper::Mapper, memory::Memory, ppu::Ppu};

#[derive(Debug)]
pub struct DmaState {
	original_base: u8,
	pub base: u8,
	pub remaining_cycles: u8,
	restarting: Option<(u8, bool)>,
}

impl DmaState {
	pub fn new() -> Self {
		Self { original_base: 0, base: 0, remaining_cycles: 0, restarting: None }
	}

	pub fn init_request(&mut self, base: u8) {
		self.base = base;
		self.restarting = Some((base, false));
	}

	pub fn tick_dma(
		&mut self,
		ppu: &mut Ppu,
		memory: &Memory,
		cartridge: Option<&(dyn Mapper + Send + Sync)>,
	) {
		match self.restarting {
			Some((base, false)) => self.restarting = Some((base, true)),
			Some((base, true)) => {
				self.original_base = base;
				self.remaining_cycles = 0xA0;
				self.restarting = None;
			}
			None => {}
		}

		ppu.dma_occuring = self.remaining_cycles > 0;

		if self.remaining_cycles > 0 {
			let offset = 0xA0 - self.remaining_cycles;
			let read_address = ((self.original_base as usize) << 8) | offset as usize;

			let value = if self.original_base <= 0x7F {
				match cartridge {
					Some(cart) => cart.read_rom_u8(read_address as u16),
					None => 0xFF,
				}
			} else if self.original_base <= 0x9F {
				let address = read_address - 0x8000;
				ppu.vram[address]
			} else if self.original_base <= 0xDF {
				let address = read_address - 0xC000;
				memory.wram[address]
			} else if self.original_base <= 0xFD {
				let address = read_address - 0xE000;
				memory.wram[address]
			} else {
				0xFF
			};
			ppu.dma_write_oam(offset, value);
			self.remaining_cycles -= 1;
		}
	}
}
