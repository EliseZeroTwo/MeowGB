use deemgee_opcode::opcode;

use super::CycleResult;
use crate::gameboy::Gameboy;

pub fn prefixed_handler(state: &mut Gameboy) -> CycleResult {
	let opcode = match state.registers.current_prefixed_opcode {
		Some(prefixed_opcode) => prefixed_opcode,
		None => match state.registers.mem_read_hold.take() {
			Some(opcode) => {
				state.registers.current_prefixed_opcode = Some(opcode);
				opcode
			}
			None => {
				state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
				return CycleResult::NeedsMore;
			}
		},
	};

	let res: CycleResult = match opcode {
		0x10 => rl_b,
		0x11 => rl_c,
		0x12 => rl_d,
		0x13 => rl_e,
		0x14 => rl_h,
		0x15 => rl_l,
		0x17 => rl_a,
		0x30 => swap_b,
		0x31 => swap_c,
		0x32 => swap_d,
		0x33 => swap_e,
		0x34 => swap_h,
		0x35 => swap_l,
		0x37 => swap_a,
		0x40 => bit_0_b,
		0x41 => bit_0_c,
		0x42 => bit_0_d,
		0x43 => bit_0_e,
		0x44 => bit_0_h,
		0x45 => bit_0_l,
		0x46 => bit_0_deref_hl,
		0x47 => bit_0_a,
		0x48 => bit_1_b,
		0x49 => bit_1_c,
		0x4a => bit_1_d,
		0x4b => bit_1_e,
		0x4c => bit_1_h,
		0x4d => bit_1_l,
		0x4e => bit_1_deref_hl,
		0x4f => bit_1_a,
		0x50 => bit_2_b,
		0x51 => bit_2_c,
		0x52 => bit_2_d,
		0x53 => bit_2_e,
		0x54 => bit_2_h,
		0x55 => bit_2_l,
		0x56 => bit_2_deref_hl,
		0x57 => bit_2_a,
		0x58 => bit_3_b,
		0x59 => bit_3_c,
		0x5a => bit_3_d,
		0x5b => bit_3_e,
		0x5c => bit_3_h,
		0x5d => bit_3_l,
		0x5e => bit_3_deref_hl,
		0x5f => bit_3_a,
		0x60 => bit_4_b,
		0x61 => bit_4_c,
		0x62 => bit_4_d,
		0x63 => bit_4_e,
		0x64 => bit_4_h,
		0x65 => bit_4_l,
		0x66 => bit_4_deref_hl,
		0x67 => bit_4_a,
		0x68 => bit_5_b,
		0x69 => bit_5_c,
		0x6a => bit_5_d,
		0x6b => bit_5_e,
		0x6c => bit_5_h,
		0x6d => bit_5_l,
		0x6e => bit_5_deref_hl,
		0x6f => bit_5_a,
		0x70 => bit_6_b,
		0x71 => bit_6_c,
		0x72 => bit_6_d,
		0x73 => bit_6_e,
		0x74 => bit_6_h,
		0x75 => bit_6_l,
		0x76 => bit_6_deref_hl,
		0x77 => bit_6_a,
		0x78 => bit_7_b,
		0x79 => bit_7_c,
		0x7a => bit_7_d,
		0x7b => bit_7_e,
		0x7c => bit_7_h,
		0x7d => bit_7_l,
		0x7e => bit_7_deref_hl,
		0x7f => bit_7_a,
		unknown => panic!(
			"Unrecognized prefixed opcode: {:#X}\nRegisters: {:#?}",
			unknown, state.registers
		),
	}(state);

	res
}

macro_rules! define_bit_reg {
	($op:literal, $bit:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<bit_ $bit _ $reg>], $op, std::concat!("BIT ", std::stringify!($bit), ",", std::stringify!($reg)), false, {
					1 => {
						state.registers.set_zero(state.registers.$reg & (1 << $bit) == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(true);
						state.registers.opcode_bytecount = Some(2);
						CycleResult::Finished
					}
			});
		}
	};
}

define_bit_reg!(0x40, 0, b);
define_bit_reg!(0x48, 1, b);
define_bit_reg!(0x50, 2, b);
define_bit_reg!(0x58, 3, b);
define_bit_reg!(0x60, 4, b);
define_bit_reg!(0x68, 5, b);
define_bit_reg!(0x70, 6, b);
define_bit_reg!(0x78, 7, b);
define_bit_reg!(0x41, 0, c);
define_bit_reg!(0x49, 1, c);
define_bit_reg!(0x51, 2, c);
define_bit_reg!(0x59, 3, c);
define_bit_reg!(0x61, 4, c);
define_bit_reg!(0x69, 5, c);
define_bit_reg!(0x71, 6, c);
define_bit_reg!(0x79, 7, c);
define_bit_reg!(0x42, 0, d);
define_bit_reg!(0x4a, 1, d);
define_bit_reg!(0x52, 2, d);
define_bit_reg!(0x5a, 3, d);
define_bit_reg!(0x62, 4, d);
define_bit_reg!(0x6a, 5, d);
define_bit_reg!(0x72, 6, d);
define_bit_reg!(0x7a, 7, d);
define_bit_reg!(0x43, 0, e);
define_bit_reg!(0x4b, 1, e);
define_bit_reg!(0x53, 2, e);
define_bit_reg!(0x5b, 3, e);
define_bit_reg!(0x63, 4, e);
define_bit_reg!(0x6b, 5, e);
define_bit_reg!(0x73, 6, e);
define_bit_reg!(0x7b, 7, e);
define_bit_reg!(0x44, 0, h);
define_bit_reg!(0x4c, 1, h);
define_bit_reg!(0x54, 2, h);
define_bit_reg!(0x5c, 3, h);
define_bit_reg!(0x64, 4, h);
define_bit_reg!(0x6c, 5, h);
define_bit_reg!(0x74, 6, h);
define_bit_reg!(0x7c, 7, h);
define_bit_reg!(0x45, 0, l);
define_bit_reg!(0x4d, 1, l);
define_bit_reg!(0x55, 2, l);
define_bit_reg!(0x5d, 3, l);
define_bit_reg!(0x65, 4, l);
define_bit_reg!(0x6d, 5, l);
define_bit_reg!(0x75, 6, l);
define_bit_reg!(0x7d, 7, l);
define_bit_reg!(0x47, 0, a);
define_bit_reg!(0x4f, 1, a);
define_bit_reg!(0x57, 2, a);
define_bit_reg!(0x5f, 3, a);
define_bit_reg!(0x67, 4, a);
define_bit_reg!(0x6f, 5, a);
define_bit_reg!(0x77, 6, a);
define_bit_reg!(0x7f, 7, a);

macro_rules! define_bit_deref_hl {
	($op:literal, $bit:literal) => {
		paste::paste! {
			opcode!([<bit_ $bit _deref_hl>], $op, std::concat!("BIT ", std::stringify!($bit), ",(HL)"), false, {
				1 => {
					state.cpu_read_u8(state.registers.get_hl());
					CycleResult::NeedsMore
				},
				2 => {
					let mem_read = state.registers.take_mem();
					state.registers.set_zero(mem_read & (1 << $bit) == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(true);
					state.registers.opcode_bytecount = Some(2);
					CycleResult::Finished
				}
			});
		}
	};
}

define_bit_deref_hl!(0x46, 0);
define_bit_deref_hl!(0x4e, 1);
define_bit_deref_hl!(0x56, 2);
define_bit_deref_hl!(0x5e, 3);
define_bit_deref_hl!(0x66, 4);
define_bit_deref_hl!(0x6e, 5);
define_bit_deref_hl!(0x76, 6);
define_bit_deref_hl!(0x7e, 7);

macro_rules! define_rl_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<rl_ $reg>], $op, std::concat!("RL ", std::stringify!($reg)), false, {
					1 => {
						let carry = state.registers.$reg >> 7 == 1;
						state.registers.$reg <<= 1;

						if state.registers.get_carry() {
							state.registers.$reg |= 1;
						}

						state.registers.set_zero(state.registers.$reg == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(false);
						state.registers.set_carry(carry);
						state.registers.opcode_bytecount = Some(2);
						CycleResult::Finished
					}
			});
		}
	};
}

define_rl_reg!(0x10, b);
define_rl_reg!(0x11, c);
define_rl_reg!(0x12, d);
define_rl_reg!(0x13, e);
define_rl_reg!(0x14, h);
define_rl_reg!(0x15, l);
define_rl_reg!(0x17, a);

macro_rules! define_swap_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<swap_ $reg>], $op, std::concat!("SWAP ", std::stringify!($reg)), false, {
				1 => {
					state.registers.$reg = (state.registers.$reg >> 4) | (state.registers.$reg << 4);

					state.registers.set_zero(state.registers.$reg == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(false);
					state.registers.opcode_bytecount = Some(2);
					CycleResult::Finished
				}
			});
		}
	};
}

define_swap_reg!(0x30, b);
define_swap_reg!(0x31, c);
define_swap_reg!(0x32, d);
define_swap_reg!(0x33, e);
define_swap_reg!(0x34, h);
define_swap_reg!(0x35, l);
define_swap_reg!(0x37, a);
