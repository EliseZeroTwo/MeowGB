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
	is_mbc1m: bool,
}

impl MBC1 {
	pub fn new(data: Vec<u8>) -> Self {
		assert!(data[0x147] >= 1 && data[0x147] <= 0x3);
		let rom_bank_count = data[0x148];
		assert!(rom_bank_count <= 0x6, "{:#X}", rom_bank_count);

		let rom_length = (1 << rom_bank_count as usize) * 0x8000;

		assert_eq!(data.len(), rom_length);

		let ram_bank_count = match data[0x149] {
			0 | 1 => 0,
			2 => 1,
			3 => 4,
			4 => 16,
			5 => 8,
			_ => panic!("Bad RAM bank count for MBC1"),
		};

		let ram = match data[0x149] {
			0 | 1 => None,
			2 | 3 | 4 | 5 => Some(vec![0u8; ram_bank_count as usize * (8 * 1024)]),
			_ => panic!("Bad RAM bank count for MBC1"),
		};

		let is_mbc1m = match rom_bank_count > 3 {
			true => &data[0x104..0x134] == &data[((0x10 << 14) + 0x104)..((0x10 << 14) + 0x134)],
			false => false,
		};

		Self {
			rom: data,
			rom_bank_count,
			ram_enabled: false,
			rom_bank_number: 1,
			extra_2_bit_reg: 0,
			ram,
			ram_bank_count,
			banking_mode_select: false,
			is_mbc1m,
		}
	}

	fn set_ram_enabled(&mut self, val: u8) {
		self.ram_enabled = val & 0b1111 == 0xA;
	}

	fn set_rom_bank_number(&mut self, val: u8) {
		self.rom_bank_number = (val & 0b11111).max(1);
	}

	fn set_extra_2_bit_reg(&mut self, val: u8) {
		self.extra_2_bit_reg = val & 0b11;
	}

	fn set_banking_mode_select(&mut self, val: u8) {
		self.banking_mode_select = val & 0b1 == 1;
	}

	fn is_large_rom(&self) -> bool {
		self.rom_bank_count >= 5
	}

	#[allow(unused)]
	fn is_large_ram(&self) -> bool {
		self.ram_bank_count >= 4
	}
}

impl Mapper for MBC1 {
	fn read_rom_u8(&self, address: u16) -> u8 {
		let mask = match self.rom_bank_count {
			0 => 0b1,
			1 => 0b11,
			2 => 0b111,
			3 => 0b1111,
			4 => 0b11111,
			5 => 0b111111,
			6 => 0b1111111,
			_ => unreachable!(),
		};
		let rom_bank = match address <= 0x3FFF {
			true if self.banking_mode_select && self.is_mbc1m => {
				(self.extra_2_bit_reg << 4) as usize
			}
			true if self.is_large_rom() && self.banking_mode_select && !self.is_mbc1m => {
				((self.extra_2_bit_reg as usize) << 5) & mask
			}
			true => 0,
			false if self.is_mbc1m => {
				((self.rom_bank_number & 0b1111) | (self.extra_2_bit_reg << 4)) as usize
			}
			false => (self.rom_bank_number | (self.extra_2_bit_reg << 5)) as usize,
		} & mask;

		let real_address = rom_bank << 14 | (address as usize & 0x3FFF);

		self.rom[real_address]
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

	fn read_eram_u8(&self, address: u16) -> u8 {
		let is_large_ram = self.is_large_ram();

		if !self.ram_enabled {
			return 0xFF;
		}

		match self.ram.as_ref() {
			Some(ram) if is_large_ram && self.banking_mode_select => {
				ram[(self.extra_2_bit_reg as usize * 0x2000) + address as usize]
			}
			Some(ram) => ram[address as usize],
			None => 0xFF,
		}
	}

	fn write_eram_u8(&mut self, address: u16, value: u8) {
		let is_large_ram = self.is_large_ram();

		if !self.ram_enabled {
			return;
		}

		match self.ram.as_mut() {
			Some(ram) if is_large_ram && self.banking_mode_select => {
				ram[(self.extra_2_bit_reg as usize * 0x2000) + address as usize] = value
			}
			Some(ram) => ram[address as usize] = value,
			None => {}
		}
	}
}
