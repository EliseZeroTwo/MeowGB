use super::interrupts::Interrupts;

#[derive(Debug)]
pub struct Timer {
	enable: bool,
	clock: TimerClock,
	div: u16,
	tima: u8,
	tma: u8,
	overflow_begin_div: u16,
	overflow: u8,
}

impl Timer {
	pub fn new() -> Self {
		Self {
			enable: false,
			clock: TimerClock::C1024,
			tima: 0,
			tma: 0,
			div: 0xAC << 8,
			overflow_begin_div: 0,
			overflow: 0u8,
		}
	}

	pub fn tick(&mut self, interrupts: &mut Interrupts) {
		self.overflow %= 4;
		let old_div = self.div;
		self.div = self.div.wrapping_add(1);

		if self.enable {
			if self.div & self.clock.div_falling_edge_bit() == 0
				&& old_div & self.clock.div_falling_edge_bit() != 0
			{
				self.increment_tima();
				return;
			}
		}

		match self.overflow {
			0 => {}
			1..=2 => {
				if self.overflow_begin_div != old_div {
					self.overflow += 1;
				}
			}
			3 => {
				self.tima = self.tma;
				self.overflow += 1;
				interrupts.write_if_timer(true);
			}
			_ => unreachable!(),
		}
	}

	pub fn read_div(&self) -> u8 {
		(self.div >> 8) as u8
	}

	pub fn read_tima(&self) -> u8 {
		self.tima
	}

	pub fn read_tma(&self) -> u8 {
		self.tma
	}

	pub fn write_tma(&mut self, value: u8) {
		self.tma = value;
		if self.overflow == 4 {
			self.tima = value;
		}
	}

	pub fn write_tima(&mut self, value: u8) {
		if self.overflow < 3 {
			self.tima = value;
			self.overflow = 0;
		}
	}

	fn increment_tima(&mut self) {
		let (new_tima, overflowed) = self.tima.overflowing_add(1);
		self.tima = new_tima;
		if overflowed {
			self.overflow = 1;
			self.overflow_begin_div = self.div;
		}
	}

	pub fn write_div(&mut self) {
		if self.div & self.clock.div_falling_edge_bit() != 0 {
			self.increment_tima();
		}

		self.div = 0;
	}

	pub fn read_tac(&self) -> u8 {
		((self.enable as u8) << 2) | self.clock.tac_clock() | 0b1111_1000
	}

	pub fn write_tac(&mut self, value: u8) {
		let new_enable = (value >> 2) & 0b1 == 1;
		let new_clock = TimerClock::from_tac_clock(value);

		if self.enable {
			let should_increment = match new_enable {
				true => {
					self.div & self.clock.div_falling_edge_bit() != 0
						&& self.div & new_clock.div_falling_edge_bit() == 0
				}
				false => self.div & self.clock.div_falling_edge_bit() != 0,
			};

			if should_increment {
				self.increment_tima();
			}
		}

		self.enable = new_enable;
		self.clock = new_clock;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerClock {
	C16 = 1,
	C64 = 2,
	C256 = 3,
	C1024 = 0,
}

impl TimerClock {
	pub const fn cycles(self) -> u16 {
		match self {
			Self::C16 => 16,
			Self::C64 => 64,
			Self::C256 => 256,
			Self::C1024 => 1024,
		}
	}

	pub const fn tac_clock(self) -> u8 {
		match self {
			Self::C16 => 1,
			Self::C64 => 2,
			Self::C256 => 3,
			Self::C1024 => 0,
		}
	}

	pub const fn from_tac_clock(value: u8) -> Self {
		match value & 0b11 {
			1 => Self::C16,
			2 => Self::C64,
			3 => Self::C256,
			0 => Self::C1024,
			_ => unreachable!(),
		}
	}

	pub const fn div_falling_edge_bit(self) -> u16 {
		self.cycles() / 2
	}
}
