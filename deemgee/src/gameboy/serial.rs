use std::io::Write;

pub struct Serial {
	pub sb: u8,
	pub sc: u8,

	internal_tick: u16,
}

impl Serial {
	pub fn new() -> Serial {
		Self { sb: 0, sc: 0, internal_tick: 0 }
	}

	pub fn set_transfer_in_process(&mut self, value: bool) {
		self.sc &= !(1 << 7);
		self.sc |= (value as u8) << 7;
	}

	pub fn get_transfer_in_process(&mut self) -> bool {
		(self.sc >> 7) & 0b1 == 1
	}

	pub fn is_conductor(&self) -> bool {
		self.sc & 0b1 == 1
	}

	pub fn set_side(&mut self, conductor: bool) {
		self.sc &= !0b1;
		self.sc |= conductor as u8;
	}

	pub fn tick(&mut self) -> bool {
		if self.get_transfer_in_process() && self.is_conductor() {
			if self.internal_tick < 128 {
				self.internal_tick += 1;
			} else {
				print!("{}", self.sb as char);
				std::io::stdout().flush().expect("flushing stdout failed");
				self.sb = 0;
				self.set_transfer_in_process(false);
				self.internal_tick = 0;
				return true;
			}
		}
		false
	}
}
