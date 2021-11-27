use super::CycleResult;
use crate::gameboy::Gameboy;

#[derive(Debug)]
pub struct CarryResult {
	pub result: u8,
	pub half_carry: bool,
	pub carry: bool,
}

pub fn sub_with_carry(lhs: u8, rhs: u8, carry: bool) -> CarryResult {
	let carry_u8 = carry as u8;

	let (first_res, first_carry) = lhs.overflowing_sub(rhs);
	let (result, second_carry) = first_res.overflowing_sub(carry_u8);

	let carry = first_carry || second_carry;

	let (first_hc_res, first_half_carry) = (lhs & 0xF).overflowing_sub(rhs & 0xF);
	let (_, second_half_carry) = first_hc_res.overflowing_sub(carry_u8);

	let half_carry = first_half_carry || second_half_carry;

	CarryResult { result, carry, half_carry }
}

macro_rules! define_xor_reg {
	($reg:ident) => {
		paste::paste! {
			pub fn [<xor_a_ $reg>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => {
						state.registers.a ^= state.registers.$reg;
						state.registers.set_zero(state.registers.a == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(false);
						state.registers.set_carry(false);
						state.registers.opcode_len = Some(1);
						CycleResult::Finished
					}
					_ => unreachable!(),
				}
			}
		}
	};
}

macro_rules! define_sbc_reg {
    ($reg:ident) => {
        paste::paste! {
            pub fn [<sbc_a_ $reg>](state: &mut Gameboy) -> CycleResult {
                match state.registers.cycle {
                    0 => {
                        let CarryResult { result, half_carry, carry } = sub_with_carry(state.registers.a, state.registers.$reg, state.registers.get_carry());

                        state.registers.a = result;
                        state.registers.set_zero(result == 0);
                        state.registers.set_subtract(true);
                        state.registers.set_half_carry(half_carry);
                        state.registers.set_carry(carry);
                        state.registers.opcode_len = Some(1);
                        CycleResult::Finished
                    },
                    _ => unreachable!(),
                }
            }
        }
    };
}

define_xor_reg!(a);
define_xor_reg!(b);
define_xor_reg!(c);
define_xor_reg!(d);
define_xor_reg!(e);
define_xor_reg!(h);
define_xor_reg!(l);

pub fn xor_a_deref_hl(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.get_hl());
			CycleResult::NeedsMore
		}
		1 => {
			state.registers.a ^= state.registers.take_mem();
			state.registers.set_zero(state.registers.a == 0);
			state.registers.set_subtract(false);
			state.registers.set_half_carry(false);
			state.registers.set_carry(false);
			state.registers.opcode_len = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn xor_a_imm_u8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc + 1);
			CycleResult::NeedsMore
		}
		1 => {
			state.registers.a ^= state.registers.take_mem();
			state.registers.set_zero(state.registers.a == 0);
			state.registers.set_subtract(false);
			state.registers.set_half_carry(false);
			state.registers.set_carry(false);
			state.registers.opcode_len = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

define_sbc_reg!(a);
define_sbc_reg!(b);
define_sbc_reg!(c);
define_sbc_reg!(d);
define_sbc_reg!(e);
define_sbc_reg!(h);
define_sbc_reg!(l);

pub fn sbc_a_deref_hl(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.get_hl());
			CycleResult::NeedsMore
		}
		1 => {
			let CarryResult { result, half_carry, carry } = sub_with_carry(
				state.registers.a,
				state.registers.take_mem(),
				state.registers.get_carry(),
			);

			state.registers.a = result;
			state.registers.set_zero(result == 0);
			state.registers.set_subtract(true);
			state.registers.set_half_carry(half_carry);
			state.registers.set_carry(carry);
			state.registers.opcode_len = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn sbc_a_imm_u8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc + 1);
			CycleResult::NeedsMore
		}
		1 => {
			let CarryResult { result, half_carry, carry } = sub_with_carry(
				state.registers.a,
				state.registers.take_mem(),
				state.registers.get_carry(),
			);

			state.registers.a = result;
			state.registers.set_zero(result == 0);
			state.registers.set_subtract(true);
			state.registers.set_half_carry(half_carry);
			state.registers.set_carry(carry);
			state.registers.opcode_len = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}