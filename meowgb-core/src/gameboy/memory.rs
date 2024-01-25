pub struct Memory {
	pub wram: [u8; 0x2000],
	pub hram: [u8; 0xAF],
}

impl Memory {
	pub fn new() -> Self {
		Self { wram: [0; 0x2000], hram: [0; 0xAF] }
	}

	pub fn get_bootrom_disabled(&self) -> u8 {
		0xFF
	}
}
