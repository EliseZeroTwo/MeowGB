use super::Mapper;

pub struct MBC1 {
	rom: Vec<u8>,
	ram: Option<Vec<u8>>,
	rom_bank_count: u8,
	ram_bank_count: u8,
	ram_enabled: bool,
	rom_bank_number: u8,
	extra_2_bit_reg: u8,
	banking_mode_select: bool,
}

impl MBC1 {
	pub fn new(data: Vec<u8>) -> Self {
		let rom_bank_count = 0b10 << data[0x148];
		let ram_bank_count = match data[0x149] {
			0 | 1 => 0,
			2 => 1,
			3 => 4,
			4 => 16,
			5 => 8,
			_ => panic!("Bad RAM bank count for MBC1"),
		};

		let ram = match ram_bank_count {
			0 | 1 => None,
			2 | 3 | 4 | 5 => Some(vec![0u8; ram_bank_count as usize * (8 * 1024)]),
			_ => panic!("Bad RAM bank count for MBC1"),
		};

		assert_eq!(data.len(), rom_bank_count as usize * (16 * 1024));

		Self {
			rom: data,
			rom_bank_count,
			ram_enabled: false,
			rom_bank_number: 1,
			extra_2_bit_reg: 0,
			ram,
			ram_bank_count,
			banking_mode_select: false,
		}
	}

	fn set_ram_enabled(&mut self, val: u8) {
		self.ram_enabled = val & 0xA != 0;
	}

	fn set_rom_bank_number(&mut self, mut val: u8) {
		if val == 0 {
			val = 1;
		}
		self.rom_bank_number = val & 0b1_1111;
	}

	fn set_extra_2_bit_reg(&mut self, val: u8) {
		self.extra_2_bit_reg = val & 0b11;
	}

	fn set_banking_mode_select(&mut self, val: u8) {
		self.banking_mode_select = val & 0b1 == 1;
	}

	fn is_large_rom(&self) -> bool {
		self.rom_bank_count >= 64
	}

	#[allow(unused)]
	fn is_large_ram(&self) -> bool {
		self.ram_bank_count > 2
	}
}

impl Mapper for MBC1 {
	fn read_rom_u8(&self, address: u16) -> u8 {
		if address <= 0x3FFF {
			self.rom[if self.is_large_rom() && self.banking_mode_select {
				((self.extra_2_bit_reg << 5) as usize * 0x4000) + address as usize
			} else {
				address as usize
			}]
		} else {
			self.rom[if self.is_large_rom() {
				(self.rom_bank_number | (self.extra_2_bit_reg << 5)) as usize * 0x4000
			} else {
				self.rom_bank_number as usize * 0x4000
			} + (address as usize - 0x4000)]
		}
	}

	fn write_rom_u8(&mut self, address: u16, value: u8) {
		match address {
			0..=0x1FFF => self.set_ram_enabled(value),
			0x2000..=0x3FFF => self.set_rom_bank_number(value),
			0x4000..=0x5FFF => self.set_extra_2_bit_reg(value),
			0x6000..=0x7FFF => self.set_banking_mode_select(value),
			_ => unreachable!(),
		}
	}

	fn read_eram_u8(&self, _address: u16) -> u8 {
		match self.ram.as_ref() {
			Some(_ram) => 0,
			None => 0,
		}
	}

	fn write_eram_u8(&mut self, _address: u16, _value: u8) {
		match self.ram.as_ref() {
			Some(_ram) => {}
			None => {}
		}
	}
}
