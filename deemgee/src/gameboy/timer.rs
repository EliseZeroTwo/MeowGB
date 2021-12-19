pub struct Timer {
	pub enable: bool,
	pub clock: TimerClock,
	pub div: u8,
	pub div_counter: u8,
	pub tima: u8,
	pub tima_counter: u16,
	pub tma: u8,
}

impl Timer {
	pub fn new() -> Self {
		Self {
			enable: false,
			clock: TimerClock::C1024,
			tima: 0,
			tma: 0,
			div: 0,
			div_counter: 0,
			tima_counter: 0,
		}
	}

	pub fn tick(&mut self) -> bool {
		self.div_counter = self.div_counter.overflowing_add(1).0;
		if self.div_counter == 0 {
			self.div = self.div.overflowing_add(1).0;
		}

		if self.enable {
			self.tima_counter = self.tima_counter.overflowing_add(1).0;
			if self.tima_counter >= self.clock.cycles() {
				self.tima = self.tima.overflowing_add(1).0;

				return self.tima == 0;
			}
		}
		false
	}

	pub fn read_tac(&self) -> u8 {
		((self.enable as u8) << 2) | self.clock.tac_clock()
	}

	pub fn write_tac(&mut self, value: u8) {
		self.enable = (value >> 2) & 0b1 == 1;
		self.clock = TimerClock::from_tac_clock(value);
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
