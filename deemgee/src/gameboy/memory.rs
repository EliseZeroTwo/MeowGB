pub struct Memory {
	pub wram: [u8; 0x2000],
	pub hram: [u8; 0xAF],

	pub bootrom_disabled: bool,
	pub bootrom: [u8; 0x100],
}

impl Memory {
	pub fn new(bootrom: Option<[u8; 0x100]>) -> Self {
		Self {
			wram: [0; 0x2000],
			hram: [0; 0xAF],
			bootrom: bootrom.unwrap_or_else(|| [0u8; 0x100]),
			bootrom_disabled: bootrom.is_none(),
		}
	}
}
