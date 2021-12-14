#[derive(Debug, Default)]
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
