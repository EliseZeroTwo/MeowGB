#[derive(Debug, PartialEq, Eq)]
pub enum JoypadMode {
	Action,
	Direction,
}

macro_rules! joypad_input {
	($input:ident, $mode:ident) => {
		paste::paste! {
			pub fn [<set_ $input>](&mut self, val: bool) {
				if val && self.mode == JoypadMode::$mode {
					self.interrupt_triggered = true;
				}
				self.$input = val
			}
		}
	};
}

pub struct Joypad {
	mode: JoypadMode,
	pub down: bool,
	pub up: bool,
	pub left: bool,
	pub right: bool,
	pub start: bool,
	pub select: bool,
	pub b: bool,
	pub a: bool,
	pub interrupt_triggered: bool,
}

impl Joypad {
	pub fn new() -> Self {
		Self {
			mode: JoypadMode::Action,
			down: false,
			up: false,
			left: false,
			right: false,
			start: false,
			select: false,
			b: false,
			a: false,
			interrupt_triggered: false,
		}
	}

	pub fn cpu_read(&self) -> u8 {
		match self.mode {
			JoypadMode::Action => {
				(1 << 4)
					| ((!self.start as u8) << 3)
					| ((!self.select as u8) << 2)
					| ((!self.b as u8) << 1)
					| (!self.a as u8)
			}
			JoypadMode::Direction => {
				(1 << 5)
					| ((!self.down as u8) << 3)
					| ((!self.up as u8) << 2)
					| ((!self.left as u8) << 1)
					| (!self.right as u8)
			}
		}
	}

	joypad_input!(a, Action);
	joypad_input!(b, Action);
	joypad_input!(start, Action);
	joypad_input!(select, Action);
	joypad_input!(up, Direction);
	joypad_input!(down, Direction);
	joypad_input!(left, Direction);
	joypad_input!(right, Direction);

	pub fn cpu_write(&mut self, content: u8) {
		if (content >> 5) & 0b1 == 0 {
			self.mode = JoypadMode::Action;
		}

		if (content >> 4) & 0b1 == 0 {
			self.mode = JoypadMode::Direction;
		}
	}
}
