use super::{memory::Memory, ppu::Ppu, GenericCartridge};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaMemoryBus {
	External,
	Video,
	Other,
}

impl DmaMemoryBus {
	pub fn from_base(base: u8) -> Self {
		match base {
			0..=0x7F | 0xA0..=0xFD => Self::External,
			0x80..=0x9F => Self::Video,
			_ => Self::Other,
		}
	}

	pub fn conflict_in_range(self, address: u16) -> bool {
		let base = (address >> 8) as u8;

		if base == 0xFE {
			true
		} else if base == 0xFF {
			false
		} else {
			match self {
				DmaMemoryBus::External => base < 0x7F || (base >= 0xA0 && base <= 0xFD),
				DmaMemoryBus::Video => base >= 0x80 && base <= 0x9F,
				DmaMemoryBus::Other => false,
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct DmaState {
	original_base: u8,
	pub dma_in_progress: Option<u16>,
	pub base: u8,
	pub remaining_cycles: u8,
	restarting: Option<(u8, bool)>,
}

impl DmaState {
	pub fn is_conflict(&self, address: u16) -> bool {
		self.in_progress().map(|bus| bus.conflict_in_range(address)).unwrap_or_default()
	}

	pub fn in_progress(&self) -> Option<DmaMemoryBus> {
		match self.dma_in_progress.is_some() {
			true => Some(DmaMemoryBus::from_base(self.original_base)),
			false => None,
		}
	}

	pub fn new() -> Self {
		Self {
			dma_in_progress: None,
			original_base: 0,
			base: 0,
			remaining_cycles: 0,
			restarting: None,
		}
	}

	pub fn init_request(&mut self, base: u8) {
		self.base = base;
		self.restarting = Some((base, false));
	}

	pub fn read_next_byte(
		&self,
		ppu: &Ppu,
		memory: &Memory,
		cartridge: Option<&GenericCartridge>,
	) -> u8 {
		let read_address = self.dma_in_progress.unwrap() as usize;
		match self.original_base {
			0..=0x7F => match cartridge {
				Some(cart) => cart.read_rom_u8(read_address as u16),
				None => 0xFF,
			},
			0x80..=0x9F => ppu.vram[read_address - 0x8000],
			0xA0..=0xBF => match cartridge {
				Some(mapper) => mapper.read_eram_u8(read_address as u16 - 0xA000),
				None => 0xFF,
			},
			0xC0..=0xDF => memory.wram[read_address - 0xC000],
			0xE0..=0xFD => memory.wram[read_address - 0xE000],
			0xFE..=0xFF => 0xFF,
		}
	}

	pub fn tick_dma(
		&mut self,
		ppu: &mut Ppu,
		memory: &Memory,
		cartridge: Option<&GenericCartridge>,
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

		// We do not clear this after running because the "in progress" should remain
		// the entire cycle
		self.dma_in_progress = match self.remaining_cycles > 0 {
			true => {
				Some(((self.original_base as u16) << 8) | (0xA0 - self.remaining_cycles) as u16)
			}
			false => None,
		};

		if self.remaining_cycles > 0 {
			let value = self.read_next_byte(ppu, memory, cartridge);
			ppu.dma_write_oam(0xA0 - self.remaining_cycles, value);
			self.remaining_cycles -= 1;
		}
	}
}
