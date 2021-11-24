macro_rules! define_bitfield_u8_gs {
	($name:ident, $offset:literal, $loc:ident) => {
		paste::paste! {
			pub fn [<read_ $name>](&self) -> bool {
				((self.$loc >> $offset) & 0b1) == 1
			}

			pub fn [<write_ $name>](&mut self, value: bool) {
				log::debug!(std::concat!("Setting ", std::stringify!($name), " to {}"), value);
				self.$loc &= !(0b1 << $offset);
				self.$loc |= (value as u8) << $offset;
			}
		}
	};
}

pub struct Interrupts {
	pub ime: bool,
	pub interrupt_enable: u8,
	pub interrupt_flag: u8,
}

impl Interrupts {
	pub fn new() -> Self {
		Self { ime: true, interrupt_enable: 0b1_1111, interrupt_flag: 0b0_0000 }
	}

	define_bitfield_u8_gs!(ie_vblank, 0, interrupt_enable);
	define_bitfield_u8_gs!(ie_lcd_stat, 1, interrupt_enable);
	define_bitfield_u8_gs!(ie_timer, 2, interrupt_enable);
	define_bitfield_u8_gs!(ie_serial, 3, interrupt_enable);
	define_bitfield_u8_gs!(ie_joypad, 4, interrupt_enable);
	define_bitfield_u8_gs!(if_vblank, 0, interrupt_flag);
	define_bitfield_u8_gs!(if_lcd_stat, 1, interrupt_flag);
	define_bitfield_u8_gs!(if_timer, 2, interrupt_flag);
	define_bitfield_u8_gs!(if_serial, 3, interrupt_flag);
	define_bitfield_u8_gs!(if_joypad, 4, interrupt_flag);
}
