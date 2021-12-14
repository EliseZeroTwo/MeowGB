pub struct Serial {
	pub sb: u8,
	pub sc: u8,
}

impl Serial {
	pub fn new() -> Serial {
		Self { sb: 0, sc: 0 }
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
}
