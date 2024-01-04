#[derive(Debug)]
pub struct Sound {
	pub nr10: u8,
	pub nr11: u8,
	pub nr12: u8,
	pub nr13: u8,
	pub nr14: u8,

	pub nr21: u8,
	pub nr22: u8,
	pub nr23: u8,
	pub nr24: u8,

	pub nr30: u8,
	pub nr31: u8,
	pub nr32: u8,
	pub nr33: u8,
	pub nr34: u8,

	pub nr41: u8,
	pub nr42: u8,
	pub nr43: u8,
	pub nr44: u8,

	pub nr50: u8,
	pub nr51: u8,
	pub nr52: u8,

	pub wave_pattern_ram: [u8; 0x10],
}

impl Sound {
	pub fn new() -> Self {
		Self {
			nr10: 0b1000_0000,
			nr11: 0b1011_1111,
			nr12: 0b1111_0011,
			nr13: 0b1111_1111,
			nr14: 0b1011_1111,
			nr21: 0b0011_1111,
			nr22: 0b0000_0000,
			nr23: 0b1111_1111,
			nr24: 0b1011_1111,
			nr30: 0b0111_1111,
			nr31: 0b1111_1111,
			nr32: 0b1001_1111,
			nr33: 0b1111_1111,
			nr34: 0b1011_1111,
			nr41: 0b1111_1111,
			nr42: 0b0000_0000,
			nr43: 0b0000_0000,
			nr44: 0b1011_1111,
			nr50: 0b0111_0111,
			nr51: 0b1111_0011,
			nr52: 0b1111_0001,
			wave_pattern_ram: [0u8;16],
		}
	}
}
