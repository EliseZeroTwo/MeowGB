mod alu;
mod load_store_move;

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
				self.f >> $bit == 1
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
	pub opcode_len: Option<u8>,
	pub current_opcode: Option<u8>,
	pub mem_read_hold: Option<u8>,
	pub mem_op_happened: bool,
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
	define_flag!(subtract, 7);
	define_flag!(half_carry, 7);
	define_flag!(carry, 7);

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

	let instruction_result: CycleResult = match opcode {
		0x01 => load_store_move::ld_bc_imm_u16,
		0x08 => load_store_move::ld_deref_imm_u16_sp,
		0x0a => load_store_move::ld_a_deref_bc,
		0x11 => load_store_move::ld_de_imm_u16,
		0x1a => load_store_move::ld_a_deref_de,
		0x21 => load_store_move::ld_hl_imm_u16,
		0x22 => load_store_move::ld_hl_plus_a,
		0x2a => load_store_move::ld_a_hl_plus,
		0x32 => load_store_move::ld_hl_minus_a,
		0x3a => load_store_move::ld_a_hl_minus,
		0x31 => load_store_move::ld_sp_imm_u16,
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
		0x78 => load_store_move::ld_a_b,
		0x79 => load_store_move::ld_a_c,
		0x7a => load_store_move::ld_a_d,
		0x7b => load_store_move::ld_a_e,
		0x7c => load_store_move::ld_a_h,
		0x7d => load_store_move::ld_a_l,
		0x7e => load_store_move::ld_a_deref_hl,
		0x7f => load_store_move::ld_a_a,
		0x98 => alu::sbc_a_b,
		0x99 => alu::sbc_a_c,
		0x9A => alu::sbc_a_d,
		0x9B => alu::sbc_a_e,
		0x9C => alu::sbc_a_h,
		0x9D => alu::sbc_a_l,
		0x9E => alu::sbc_a_deref_hl,
		0x9F => alu::sbc_a_a,
		0xA8 => alu::xor_a_b,
		0xA9 => alu::xor_a_c,
		0xAA => alu::xor_a_d,
		0xAB => alu::xor_a_e,
		0xAC => alu::xor_a_h,
		0xAD => alu::xor_a_l,
		0xAE => alu::xor_a_deref_hl,
		0xAF => alu::xor_a_a,
		0xDE => alu::sbc_a_imm_u8,
		0xEE => alu::xor_a_imm_u8,
		0xF9 => load_store_move::ld_sp_hl,
		unknown => panic!("Unrecognized opcode: {:#X}\nRegisters: {:#?}", unknown, state.registers),
	}(state);

	if instruction_result == CycleResult::Finished {
		match state.registers.opcode_len {
			Some(len) => state.registers.pc += len as u16,
			None => panic!("Forgot to set opcode len for {:#X}", opcode),
		}

		if !state.registers.mem_op_happened {
			log::trace!("Memory bus clear, precaching next opcode");
			state.cpu_read_u8(state.registers.pc);
		}

		state.registers.cycle = 0;
		state.registers.current_opcode = None;
		state.registers.opcode_len = None;
		log::trace!("Cycle finished");
	} else {
		state.registers.cycle += 1;
	}
}