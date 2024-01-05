use std::io::Write;

use super::interrupts::Interrupts;

pub trait SerialWriter {
	fn write_byte(&mut self, byte: u8);
}

impl<T: Write> SerialWriter for T {
	fn write_byte(&mut self, byte: u8) {
		self.write_all(&[byte])
			.expect(format!("writing serial to {} failed", std::any::type_name::<T>()).as_str());
		self.flush()
			.expect(format!("flushing serial to {} failed", std::any::type_name::<T>()).as_str());
	}
}

pub struct Serial<S: SerialWriter> {
	pub sb: u8,
	sc: u8,

	internal_tick: u16,
	writer: S,
}

impl<S: SerialWriter> Serial<S> {
	pub fn new(writer: S) -> Serial<S> {
		Self { sb: 0, sc: 0, internal_tick: 0, writer }
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

	pub fn set_sc(&mut self, value: u8) {
		self.sc = value | (0b0111_1110);
	}

	pub fn get_sc(&self) -> u8 {
		self.sc | (0b0111_1110)
	}

	#[allow(unused)]
	pub fn set_side(&mut self, conductor: bool) {
		self.sc &= !0b1;
		self.sc |= conductor as u8;
	}

	pub fn tick(&mut self, interrupts: &mut Interrupts) {
		if self.get_transfer_in_process() && self.is_conductor() {
			if self.internal_tick < 128 {
				self.internal_tick += 1;
			} else {
				self.writer.write_byte(self.sb);
				self.sb = 0;
				self.set_transfer_in_process(false);
				self.internal_tick = 0;
				interrupts.write_if_serial(true);
			}
		}
	}
}
