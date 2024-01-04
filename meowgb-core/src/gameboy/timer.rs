#[derive(Debug)]
pub struct Timer {
	pub enable: bool,
	pub clock: TimerClock,
	pub div: u8,
	pub div_counter: u8,
	pub tima: u8,
	pub tima_counter: u16,
	pub tma: u8,
	overflow: bool,
}

impl Timer {
	pub fn new() -> Self {
		Self {
			enable: false,
			clock: TimerClock::C1024,
			tima: 0,
			tma: 0,
			div: 0xAD,
			div_counter: 0,
			tima_counter: 0,
			overflow: false,
		}
	}

	pub fn tick(&mut self) -> bool {
		self.div_counter = self.div_counter.wrapping_add(1);
		if self.div_counter == 0 {
			self.div = self.div.wrapping_add(1);
		}

		if self.enable {
			self.tima_counter = self.tima_counter.wrapping_add(4);
			if self.tima_counter >= self.clock.cycles() {
				self.tima_counter = 0;
				self.tima = self.tima.wrapping_add(1);

				self.overflow = self.tima == 0;
				return false;
			}
		}

		if self.overflow {
			self.tima = self.tma;
			self.overflow = false;
			return true;
		}
		false
	}

	pub fn read_tac(&self) -> u8 {
		((self.enable as u8) << 2) | self.clock.tac_clock() | 0b1111_1000
	}

	pub fn write_tac(&mut self, value: u8) {
		self.enable = (value >> 2) & 0b1 == 1;
		self.tima_counter = 0;
		let new_clock = TimerClock::from_tac_clock(value);
		if self.clock == TimerClock::C16 && new_clock == TimerClock::C1024 && self.enable {
			self.tima = self.tima.wrapping_add(1);

			self.overflow = self.tima == 0;
		}
		self.clock = new_clock;
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimerClock {
	C16,
	C64,
	C256,
	C1024,
}

impl TimerClock {
	pub fn cycles(&self) -> u16 {
		match self {
			Self::C16 => 16,
			Self::C64 => 64,
			Self::C256 => 256,
			Self::C1024 => 1024,
		}
	}

	pub fn tac_clock(&self) -> u8 {
		match self {
			Self::C16 => 1,
			Self::C64 => 2,
			Self::C256 => 3,
			Self::C1024 => 0,
		}
	}

	pub fn from_tac_clock(value: u8) -> Self {
		match value & 0b11 {
			1 => Self::C16,
			2 => Self::C64,
			3 => Self::C256,
			0 => Self::C1024,
			_ => unreachable!(),
		}
	}
}
