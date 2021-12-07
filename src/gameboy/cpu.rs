mod alu;
mod flow;
mod load_store_move;
mod misc;
mod prefixed;

use super::Gameboy;

macro_rules! define_register {
	($lident:ident, $rident:ident) => {
		paste::paste! {
			pub fn [<get_ $lident $rident>](&self) -> u16 {
				(self.$lident as u16) << 8 | self.$rident as u16
			}

			pub fn [<set_ $lident $rident>](&mut self, value: u16) {
				self.$lident = (value >> 8) as u8;
				self.$rident = value as u8;
			}
		}
	};
}

macro_rules! define_flag {
	($flag:ident, $bit:literal) => {
		paste::paste! {
			pub fn [<get_ $flag>](&self) -> bool {
				(self.f >> $bit) & 0b1 == 1
			}

			pub fn [<set_ $flag>](&mut self, value: bool) {
				self.f &= !(1 << $bit);
				self.f |= (value as u8) << $bit;
			}
		}
	};
}

#[derive(Debug, PartialEq, Eq)]
pub enum CycleResult {
	NeedsMore,
	Finished,
}

#[derive(Debug, Default)]
pub struct Registers {
	pub a: u8,
	pub f: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub h: u8,
	pub l: u8,
	pub sp: u16,
	pub pc: u16,

	// Not actual registers
	pub cycle: u8,
	pub hold: Option<u16>,
	pub opcode_bytecount: Option<u8>,
	pub current_opcode: Option<u8>,
	pub current_prefixed_opcode: Option<u8>,
	pub mem_read_hold: Option<u8>,
	pub mem_op_happened: bool,
	pub in_interrupt_vector: Option<u8>,
}

impl Registers {
	define_register!(a, f);
	define_register!(b, c);
	define_register!(d, e);
	define_register!(h, l);

	/// This is just a helper function for macros utilizing ident pasting
	pub fn get_sp(&self) -> u16 {
		self.sp
	}

	/// This is just a helper function for macros utilizing ident pasting
	pub fn set_sp(&mut self, value: u16) {
		self.sp = value;
	}

	define_flag!(zero, 7);
	define_flag!(subtract, 6);
	define_flag!(half_carry, 5);
	define_flag!(carry, 4);

	pub fn take_mem(&mut self) -> u8 {
		self.mem_read_hold.take().unwrap()
	}

	pub fn take_hold(&mut self) -> u16 {
		self.hold.take().unwrap()
	}

	pub fn set_hold(&mut self, value: u16) {
		assert!(self.hold.is_none());
		self.hold = Some(value);
	}
}

pub fn tick_cpu(state: &mut Gameboy) {
	state.registers.mem_op_happened = false;

	// TODO: Interrupts
	if state.registers.cycle == 0 && state.interrupts.ime {
		if state.interrupts.read_ie_vblank() && state.interrupts.read_if_vblank() {
			state.registers.in_interrupt_vector = Some(0);
			state.interrupts.ime = false;
			state.interrupts.write_if_vblank(false);
		} else if state.interrupts.read_ie_lcd_stat() && state.interrupts.read_if_lcd_stat() {
			state.registers.in_interrupt_vector = Some(1);
			state.interrupts.ime = false;
			state.interrupts.write_if_lcd_stat(false);
		} else if state.interrupts.read_ie_timer() && state.interrupts.read_if_timer() {
			state.registers.in_interrupt_vector = Some(2);
			state.interrupts.ime = false;
			state.interrupts.write_if_timer(false);
		} else if state.interrupts.read_ie_serial() && state.interrupts.read_if_serial() {
			state.registers.in_interrupt_vector = Some(3);
			state.interrupts.ime = false;
			state.interrupts.write_if_serial(false);
		} else if state.interrupts.read_ie_joypad() && state.interrupts.read_if_joypad() {
			state.registers.in_interrupt_vector = Some(4);
			state.interrupts.ime = false;
			state.interrupts.write_if_joypad(false);
		}
	}

	let result = if let Some(idx) = state.registers.in_interrupt_vector {
		match state.registers.cycle {
			0 => {
				// Invalidate prefetch if present
				state.registers.mem_read_hold = None;
				CycleResult::NeedsMore
			}
			1 => CycleResult::NeedsMore,
			2 => {
				state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
				CycleResult::NeedsMore
			}
			3 => {
				state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
				CycleResult::NeedsMore
			}
			4 => {
				state.registers.pc = match idx {
					0 => 0x40,
					1 => 0x48,
					2 => 0x50,
					3 => 0x58,
					4 => 0x60,
					_ => unreachable!(),
				};
				state.registers.in_interrupt_vector = None;
				state.registers.opcode_bytecount = Some(0);
				log::info!("Triggering interrupt to {:#X}", state.registers.pc);
				CycleResult::Finished
			}
			_ => unreachable!(),
		}
	} else {
		let opcode = match state.registers.current_opcode {
			Some(opcode) => opcode,
			None => match state.registers.mem_read_hold.take() {
				Some(opcode) => {
					log::debug!("Executing instruction {:#X}", opcode);
					state.registers.current_opcode = Some(opcode);
					opcode
				}
				None => {
					state.cpu_read_u8(state.registers.pc);
					return;
				}
			},
		};

		let result: CycleResult = match opcode {
			0x00 => misc::nop,
			0x01 => load_store_move::ld_bc_imm_u16,
			0x03 => alu::inc_bc,
			0x04 => alu::inc_b,
			0x05 => alu::dec_b,
			0x06 => load_store_move::ld_b_imm_u8,
			0x08 => load_store_move::ld_deref_imm_u16_sp,
			0x09 => alu::add_hl_bc,
			0x0a => load_store_move::ld_a_deref_bc,
			0x0b => alu::dec_bc,
			0x0c => alu::inc_c,
			0x0d => alu::dec_c,
			0x0e => load_store_move::ld_c_imm_u8,
			0x11 => load_store_move::ld_de_imm_u16,
			0x13 => alu::inc_de,
			0x14 => alu::inc_d,
			0x15 => alu::dec_d,
			0x16 => load_store_move::ld_d_imm_u8,
			0x17 => alu::rla,
			0x18 => flow::jr_i8,
			0x19 => alu::add_hl_de,
			0x1a => load_store_move::ld_a_deref_de,
			0x1b => alu::dec_de,
			0x1c => alu::inc_e,
			0x1d => alu::dec_e,
			0x1e => load_store_move::ld_e_imm_u8,
			0x20 => flow::jr_nz_i8,
			0x21 => load_store_move::ld_hl_imm_u16,
			0x22 => load_store_move::ld_hl_plus_a,
			0x23 => alu::inc_hl,
			0x24 => alu::inc_h,
			0x25 => alu::dec_h,
			0x26 => load_store_move::ld_h_imm_u8,
			0x28 => flow::jr_z_i8,
			0x29 => alu::add_hl_hl,
			0x2a => load_store_move::ld_a_hl_plus,
			0x2b => alu::dec_hl,
			0x2c => alu::inc_l,
			0x2d => alu::dec_l,
			0x2e => load_store_move::ld_l_imm_u8,
			0x2f => alu::cpl,
			0x30 => flow::jr_nc_i8,
			0x31 => load_store_move::ld_sp_imm_u16,
			0x32 => load_store_move::ld_hl_minus_a,
			0x33 => alu::inc_sp,
			0x34 => alu::inc_deref_hl,
			0x35 => alu::dec_deref_hl,
			0x36 => load_store_move::ld_deref_hl_imm_u8,
			0x37 => alu::scf,
			0x38 => flow::jr_c_i8,
			0x39 => alu::add_hl_sp,
			0x3a => load_store_move::ld_a_hl_minus,
			0x3b => alu::dec_sp,
			0x3c => alu::inc_a,
			0x3d => alu::dec_a,
			0x3e => load_store_move::ld_a_imm_u8,
			0x3f => alu::ccf,
			0x40 => load_store_move::ld_b_b,
			0x41 => load_store_move::ld_b_c,
			0x42 => load_store_move::ld_b_d,
			0x43 => load_store_move::ld_b_e,
			0x44 => load_store_move::ld_b_h,
			0x45 => load_store_move::ld_b_l,
			0x46 => load_store_move::ld_b_deref_hl,
			0x47 => load_store_move::ld_b_a,
			0x48 => load_store_move::ld_c_b,
			0x49 => load_store_move::ld_c_c,
			0x4a => load_store_move::ld_c_d,
			0x4b => load_store_move::ld_c_e,
			0x4c => load_store_move::ld_c_h,
			0x4d => load_store_move::ld_c_l,
			0x4e => load_store_move::ld_c_deref_hl,
			0x4f => load_store_move::ld_c_a,
			0x50 => load_store_move::ld_d_b,
			0x51 => load_store_move::ld_d_c,
			0x52 => load_store_move::ld_d_d,
			0x53 => load_store_move::ld_d_e,
			0x54 => load_store_move::ld_d_h,
			0x55 => load_store_move::ld_d_l,
			0x56 => load_store_move::ld_d_deref_hl,
			0x57 => load_store_move::ld_d_a,
			0x58 => load_store_move::ld_e_b,
			0x59 => load_store_move::ld_e_c,
			0x5a => load_store_move::ld_e_d,
			0x5b => load_store_move::ld_e_e,
			0x5c => load_store_move::ld_e_h,
			0x5d => load_store_move::ld_e_l,
			0x5e => load_store_move::ld_e_deref_hl,
			0x5f => load_store_move::ld_e_a,
			0x60 => load_store_move::ld_h_b,
			0x61 => load_store_move::ld_h_c,
			0x62 => load_store_move::ld_h_d,
			0x63 => load_store_move::ld_h_e,
			0x64 => load_store_move::ld_h_h,
			0x65 => load_store_move::ld_h_l,
			0x66 => load_store_move::ld_h_deref_hl,
			0x67 => load_store_move::ld_h_a,
			0x68 => load_store_move::ld_l_b,
			0x69 => load_store_move::ld_l_c,
			0x6a => load_store_move::ld_l_d,
			0x6b => load_store_move::ld_l_e,
			0x6c => load_store_move::ld_l_h,
			0x6d => load_store_move::ld_l_l,
			0x6e => load_store_move::ld_l_deref_hl,
			0x6f => load_store_move::ld_l_a,
			0x70 => load_store_move::ld_deref_hl_b,
			0x71 => load_store_move::ld_deref_hl_c,
			0x72 => load_store_move::ld_deref_hl_d,
			0x73 => load_store_move::ld_deref_hl_e,
			0x74 => load_store_move::ld_deref_hl_h,
			0x75 => load_store_move::ld_deref_hl_l,
			0x77 => load_store_move::ld_deref_hl_a,
			0x78 => load_store_move::ld_a_b,
			0x79 => load_store_move::ld_a_c,
			0x7a => load_store_move::ld_a_d,
			0x7b => load_store_move::ld_a_e,
			0x7c => load_store_move::ld_a_h,
			0x7d => load_store_move::ld_a_l,
			0x7e => load_store_move::ld_a_deref_hl,
			0x7f => load_store_move::ld_a_a,
			0x80 => alu::add_a_b,
			0x81 => alu::add_a_c,
			0x82 => alu::add_a_d,
			0x83 => alu::add_a_e,
			0x84 => alu::add_a_h,
			0x85 => alu::add_a_l,
			0x86 => alu::add_a_deref_hl,
			0x87 => alu::add_a_a,
			0x88 => alu::adc_a_b,
			0x89 => alu::adc_a_c,
			0x8A => alu::adc_a_d,
			0x8B => alu::adc_a_e,
			0x8C => alu::adc_a_h,
			0x8D => alu::adc_a_l,
			0x8E => alu::adc_a_deref_hl,
			0x8F => alu::adc_a_a,
			0x90 => alu::sub_a_b,
			0x91 => alu::sub_a_c,
			0x92 => alu::sub_a_d,
			0x93 => alu::sub_a_e,
			0x94 => alu::sub_a_h,
			0x95 => alu::sub_a_l,
			0x96 => alu::sub_a_deref_hl,
			0x97 => alu::sub_a_a,
			0x98 => alu::sbc_a_b,
			0x99 => alu::sbc_a_c,
			0x9A => alu::sbc_a_d,
			0x9B => alu::sbc_a_e,
			0x9C => alu::sbc_a_h,
			0x9D => alu::sbc_a_l,
			0x9E => alu::sbc_a_deref_hl,
			0x9F => alu::sbc_a_a,
			0xA0 => alu::and_a_b,
			0xA1 => alu::and_a_c,
			0xA2 => alu::and_a_d,
			0xA3 => alu::and_a_e,
			0xA4 => alu::and_a_h,
			0xA5 => alu::and_a_l,
			0xA6 => alu::and_a_deref_hl,
			0xA7 => alu::and_a_a,
			0xA8 => alu::xor_a_b,
			0xA9 => alu::xor_a_c,
			0xAA => alu::xor_a_d,
			0xAB => alu::xor_a_e,
			0xAC => alu::xor_a_h,
			0xAD => alu::xor_a_l,
			0xAE => alu::xor_a_deref_hl,
			0xAF => alu::xor_a_a,
			0xB0 => alu::or_a_b,
			0xB1 => alu::or_a_c,
			0xB2 => alu::or_a_d,
			0xB3 => alu::or_a_e,
			0xB4 => alu::or_a_h,
			0xB5 => alu::or_a_l,
			0xB6 => alu::or_a_deref_hl,
			0xB7 => alu::or_a_a,
			0xB8 => alu::cp_a_b,
			0xB9 => alu::cp_a_c,
			0xBA => alu::cp_a_d,
			0xBB => alu::cp_a_e,
			0xBC => alu::cp_a_h,
			0xBD => alu::cp_a_l,
			0xBE => alu::cp_a_deref_hl,
			0xBF => alu::cp_a_a,
			0xC0 => flow::ret_nz,
			0xC1 => load_store_move::pop_bc,
			0xC2 => flow::jp_nz_u16,
			0xC3 => flow::jp_u16,
			0xC4 => flow::call_nz_u16,
			0xC5 => load_store_move::push_bc,
			0xC6 => alu::add_a_imm_u8,
			0xC7 => flow::rst_0x0,
			0xC8 => flow::ret_z,
			0xC9 => flow::ret,
			0xCA => flow::jp_z_u16,
			0xCB => prefixed::prefixed_handler,
			0xCC => flow::call_z_u16,
			0xCD => flow::call_u16,
			0xCE => alu::adc_a_imm_u8,
			0xCF => flow::rst_0x08,
			0xD0 => flow::ret_nc,
			0xD1 => load_store_move::pop_de,
			0xD2 => flow::jp_nc_u16,
			0xD4 => flow::call_nc_u16,
			0xD5 => load_store_move::push_de,
			0xD6 => alu::sub_a_imm_u8,
			0xD7 => flow::rst_0x10,
			0xD8 => flow::ret_c,
			0xD9 => flow::reti,
			0xDA => flow::jp_c_u16,
			0xDC => flow::call_c_u16,
			0xDE => alu::sbc_a_imm_u8,
			0xDF => flow::rst_0x18,
			0xE0 => load_store_move::ldh_imm_u8_a,
			0xE1 => load_store_move::pop_hl,
			0xE2 => load_store_move::ldh_deref_c_a,
			0xE5 => load_store_move::push_hl,
			0xE6 => alu::and_a_imm_u8,
			0xE7 => flow::rst_0x20,
			0xE9 => flow::jp_hl,
			0xEA => load_store_move::ld_deref_imm_u16_a,
			0xEE => alu::xor_a_imm_u8,
			0xEF => flow::rst_0x28,
			0xF0 => load_store_move::ldh_a_imm_u8,
			0xF1 => load_store_move::pop_af,
			0xF2 => load_store_move::ldh_a_deref_c,
			0xF3 => misc::di,
			0xF5 => load_store_move::push_af,
			0xF6 => alu::or_a_imm_u8,
			0xF7 => flow::rst_0x30,
			0xF9 => load_store_move::ld_sp_hl,
			0xFA => load_store_move::ld_a_deref_imm_u16,
			0xFB => misc::ei,
			0xFE => alu::cp_a_imm_u8,
			0xFF => flow::rst_0x38,
			unknown => {
				panic!("Unrecognized opcode: {:#X}\nRegisters: {:#x?}", unknown, state.registers)
			}
		}(state);

		if result == CycleResult::Finished && state.registers.opcode_bytecount.is_none() {
			panic!("Forgot to set opcode len for {:#X}", opcode)
		}

		result
	};

	if result == CycleResult::Finished {
		match state.registers.opcode_bytecount {
			Some(len) => state.registers.pc += len as u16,
			None => panic!("Forgot to set opcode len"),
		}

		if !state.registers.mem_op_happened {
			log::trace!("Memory bus clear, precaching next opcode");
			state.cpu_read_u8(state.registers.pc);
		}

		state.registers.cycle = 0;
		state.registers.current_prefixed_opcode = None;
		state.registers.current_opcode = None;
		state.registers.opcode_bytecount = None;
		log::trace!("Cycle finished");
	} else {
		state.registers.cycle += 1;
	}
}
