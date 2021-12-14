pub trait Mapper {
	fn read_rom_u8(&self, address: u16) -> u8;
	fn write_rom_u8(&mut self, address: u16, value: u8);

	fn read_eram_u8(&self, address: u16) -> u8;
	fn write_eram_u8(&mut self, address: u16, value: u8);
}

pub struct NoMBC {
	rom: [u8; 0x8000],
	ram: Option<[u8; 0x2000]>,
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

	fn write_rom_u8(&mut self, address: u16, value: u8) {
		self.rom[address as usize] = value
	}

	fn read_eram_u8(&self, address: u16) -> u8 {
		let decoded_address = address - 0xA000;
		match &self.ram {
			Some(ram) => ram[decoded_address as usize],
			None => 0,
		}
	}

	fn write_eram_u8(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0xA000;
		if let Some(ram) = &mut self.ram {
			ram[decoded_address as usize] = value;
		}
	}
}
