pub mod mbc1;

pub trait Mapper {
	fn read_rom_u8(&self, address: u16) -> u8;
	fn write_rom_u8(&mut self, address: u16, value: u8);

	/// The address passed into this function MUST be zero indexed
	fn read_eram_u8(&self, address: u16) -> u8;
	/// The address passed into this function MUST be zero indexed
	fn write_eram_u8(&mut self, address: u16, value: u8);
}

pub struct NoMBC {
	pub rom: [u8; 0x8000],
	pub ram: Option<[u8; 0x2000]>,
}

impl NoMBC {
	pub fn new(data: Vec<u8>) -> Self {
		let mut out = Self { rom: [0; 0x8000], ram: None };

		match data[0x149] {
			0 => {}
			2 => out.ram = Some([0; 0x2000]),
			other => unreachable!("RAM Type of {} on NoMBC", other),
		}

		for (idx, data) in data.iter().enumerate() {
			out.rom[idx] = *data;
		}

		out
	}
}

impl Mapper for NoMBC {
	fn read_rom_u8(&self, address: u16) -> u8 {
		self.rom[address as usize]
	}

	fn write_rom_u8(&mut self, _address: u16, _value: u8) {}

	fn read_eram_u8(&self, address: u16) -> u8 {
		match &self.ram {
			Some(ram) => ram[address as usize],
			None => 0,
		}
	}

	fn write_eram_u8(&mut self, address: u16, value: u8) {
		if let Some(ram) = &mut self.ram {
			ram[address as usize] = value;
		}
	}
}
