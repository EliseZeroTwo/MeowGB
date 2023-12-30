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
		0x00 => rlc_b,
		0x01 => rlc_c,
		0x02 => rlc_d,
		0x03 => rlc_e,
		0x04 => rlc_h,
		0x05 => rlc_l,
		0x07 => rlc_a,
		0x08 => rrc_b,
		0x09 => rrc_c,
		0x0a => rrc_d,
		0x0b => rrc_e,
		0x0c => rrc_h,
		0x0d => rrc_l,
		0x0f => rrc_a,
		0x10 => rl_b,
		0x11 => rl_c,
		0x12 => rl_d,
		0x13 => rl_e,
		0x14 => rl_h,
		0x15 => rl_l,
		0x17 => rl_a,
		0x18 => rr_b,
		0x19 => rr_c,
		0x1a => rr_d,
		0x1b => rr_e,
		0x1c => rr_h,
		0x1d => rr_l,
		0x1f => rr_a,
		0x20 => sra_b,
		0x21 => sra_c,
		0x22 => sra_d,
		0x23 => sra_e,
		0x24 => sra_h,
		0x25 => sra_l,
		0x27 => sra_a,
		0x28 => sla_b,
		0x29 => sla_c,
		0x2a => sla_d,
		0x2b => sla_e,
		0x2c => sla_h,
		0x2d => sla_l,
		0x2f => sla_a,
		0x30 => swap_b,
		0x31 => swap_c,
		0x32 => swap_d,
		0x33 => swap_e,
		0x34 => swap_h,
		0x35 => swap_l,
		0x37 => swap_a,
		0x38 => srl_b,
		0x39 => srl_c,
		0x3a => srl_d,
		0x3b => srl_e,
		0x3c => srl_h,
		0x3d => srl_l,
		0x3f => srl_a,
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
		0x80 => res_0_b,
		0x81 => res_0_c,
		0x82 => res_0_d,
		0x83 => res_0_e,
		0x84 => res_0_h,
		0x85 => res_0_l,
		0x86 => res_0_deref_hl,
		0x87 => res_0_a,
		0x88 => res_1_b,
		0x89 => res_1_c,
		0x8A => res_1_d,
		0x8B => res_1_e,
		0x8C => res_1_h,
		0x8D => res_1_l,
		0x8E => res_1_deref_hl,
		0x8F => res_1_a,
		0x90 => res_2_b,
		0x91 => res_2_c,
		0x92 => res_2_d,
		0x93 => res_2_e,
		0x94 => res_2_h,
		0x95 => res_2_l,
		0x96 => res_2_deref_hl,
		0x97 => res_2_a,
		0x98 => res_3_b,
		0x99 => res_3_c,
		0x9A => res_3_d,
		0x9B => res_3_e,
		0x9C => res_3_h,
		0x9D => res_3_l,
		0x9E => res_3_deref_hl,
		0x9F => res_3_a,
		0xA0 => res_4_b,
		0xA1 => res_4_c,
		0xA2 => res_4_d,
		0xA3 => res_4_e,
		0xA4 => res_4_h,
		0xA5 => res_4_l,
		0xA6 => res_4_deref_hl,
		0xA7 => res_4_a,
		0xA8 => res_5_b,
		0xA9 => res_5_c,
		0xAA => res_5_d,
		0xAB => res_5_e,
		0xAC => res_5_h,
		0xAD => res_5_l,
		0xAE => res_5_deref_hl,
		0xAF => res_5_a,
		0xB0 => res_6_b,
		0xB1 => res_6_c,
		0xB2 => res_6_d,
		0xB3 => res_6_e,
		0xB4 => res_6_h,
		0xB5 => res_6_l,
		0xB6 => res_6_deref_hl,
		0xB7 => res_6_a,
		0xB8 => res_7_b,
		0xB9 => res_7_c,
		0xBA => res_7_d,
		0xBB => res_7_e,
		0xBC => res_7_h,
		0xBD => res_7_l,
		0xBE => res_7_deref_hl,
		0xBF => res_7_a,
		0xC0 => set_0_b,
		0xC1 => set_0_c,
		0xC2 => set_0_d,
		0xC3 => set_0_e,
		0xC4 => set_0_h,
		0xC5 => set_0_l,
		0xC6 => set_0_deref_hl,
		0xC7 => set_0_a,
		0xC8 => set_1_b,
		0xC9 => set_1_c,
		0xCA => set_1_d,
		0xCB => set_1_e,
		0xCC => set_1_h,
		0xCD => set_1_l,
		0xCE => set_1_deref_hl,
		0xCF => set_1_a,
		0xD0 => set_2_b,
		0xD1 => set_2_c,
		0xD2 => set_2_d,
		0xD3 => set_2_e,
		0xD4 => set_2_h,
		0xD5 => set_2_l,
		0xD6 => set_2_deref_hl,
		0xD7 => set_2_a,
		0xD8 => set_3_b,
		0xD9 => set_3_c,
		0xDA => set_3_d,
		0xDB => set_3_e,
		0xDC => set_3_h,
		0xDD => set_3_l,
		0xDE => set_3_deref_hl,
		0xDF => set_3_a,
		0xE0 => set_4_b,
		0xE1 => set_4_c,
		0xE2 => set_4_d,
		0xE3 => set_4_e,
		0xE4 => set_4_h,
		0xE5 => set_4_l,
		0xE6 => set_4_deref_hl,
		0xE7 => set_4_a,
		0xE8 => set_5_b,
		0xE9 => set_5_c,
		0xEA => set_5_d,
		0xEB => set_5_e,
		0xEC => set_5_h,
		0xED => set_5_l,
		0xEE => set_5_deref_hl,
		0xEF => set_5_a,
		0xF0 => set_6_b,
		0xF1 => set_6_c,
		0xF2 => set_6_d,
		0xF3 => set_6_e,
		0xF4 => set_6_h,
		0xF5 => set_6_l,
		0xF6 => set_6_deref_hl,
		0xF7 => set_6_a,
		0xF8 => set_7_b,
		0xF9 => set_7_c,
		0xFA => set_7_d,
		0xFB => set_7_e,
		0xFC => set_7_h,
		0xFD => set_7_l,
		0xFE => set_7_deref_hl,
		0xFF => set_7_a,
		unknown => panic!(
			"Unrecognized prefixed opcode: {:#X}\nRegisters: {:#X?}",
			unknown, state.registers
		),
	}(state);

	res
}

macro_rules! define_bit_reg {
	($op:literal, $bit:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<bit_ $bit _ $reg>], $op, std::concat!("BIT ", std::stringify!($bit), ",", std::stringify!($reg)), true, 2, {
					1 => {
						state.registers.set_zero(state.registers.$reg & (1 << $bit) == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(true);
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
			opcode!([<bit_ $bit _deref_hl>], $op, std::concat!("BIT ", std::stringify!($bit), ",(HL)"), true, 2, {
				1 => {
					state.cpu_read_u8(state.registers.get_hl());
					CycleResult::NeedsMore
				},
				2 => {
					let mem_read = state.registers.take_mem();
					state.registers.set_zero(mem_read & (1 << $bit) == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(true);
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

macro_rules! define_rlc_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<rlc_ $reg>], $op, std::concat!("RLC ", std::stringify!($reg)), true, 2, {
					1 => {
						let carry = state.registers.$reg >> 7 == 1;
						state.registers.$reg <<= 1;
						state.registers.$reg |= carry as u8;

						state.registers.set_zero(state.registers.$reg == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(false);
						state.registers.set_carry(carry);
						CycleResult::Finished
					}
			});
		}
	};
}

define_rlc_reg!(0x00, b);
define_rlc_reg!(0x01, c);
define_rlc_reg!(0x02, d);
define_rlc_reg!(0x03, e);
define_rlc_reg!(0x04, h);
define_rlc_reg!(0x05, l);
define_rlc_reg!(0x07, a);

macro_rules! define_rrc_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<rrc_ $reg>], $op, std::concat!("RRC ", std::stringify!($reg)), true, 2, {
					1 => {
						let carry = state.registers.$reg & 0b1 == 1;
						state.registers.$reg >>= 1;
						state.registers.$reg |= (carry as u8) << 7;

						state.registers.set_zero(state.registers.$reg == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(false);
						state.registers.set_carry(carry);
						CycleResult::Finished
					}
			});
		}
	};
}

define_rrc_reg!(0x08, b);
define_rrc_reg!(0x09, c);
define_rrc_reg!(0x0A, d);
define_rrc_reg!(0x0B, e);
define_rrc_reg!(0x0C, h);
define_rrc_reg!(0x0D, l);
define_rrc_reg!(0x0F, a);

macro_rules! define_rl_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<rl_ $reg>], $op, std::concat!("RL ", std::stringify!($reg)), true, 2, {
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

macro_rules! define_sla_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<sla_ $reg>], $op, std::concat!("SLA ", std::stringify!($reg)), true, 2, {
				1 => {
					let carry = state.registers.$reg & (0b1 << 7) == 1;
					state.registers.$reg = ((state.registers.$reg as i8) << 1) as u8;

					state.registers.set_zero(state.registers.$reg == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_sla_reg!(0x20, b);
define_sla_reg!(0x21, c);
define_sla_reg!(0x22, d);
define_sla_reg!(0x23, e);
define_sla_reg!(0x24, h);
define_sla_reg!(0x25, l);
define_sla_reg!(0x27, a);

macro_rules! define_sra_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<sra_ $reg>], $op, std::concat!("SRA ", std::stringify!($reg)), true, 2, {
				1 => {
					let carry = state.registers.$reg & 0b1 == 1;
					state.registers.$reg = ((state.registers.$reg as i8) >> 1) as u8;

					state.registers.set_zero(state.registers.$reg == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_sra_reg!(0x28, b);
define_sra_reg!(0x29, c);
define_sra_reg!(0x2A, d);
define_sra_reg!(0x2B, e);
define_sra_reg!(0x2C, h);
define_sra_reg!(0x2D, l);
define_sra_reg!(0x2F, a);

macro_rules! define_swap_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<swap_ $reg>], $op, std::concat!("SWAP ", std::stringify!($reg)), true, 2, {
				1 => {
					state.registers.$reg = (state.registers.$reg >> 4) | (state.registers.$reg << 4);

					state.registers.set_zero(state.registers.$reg == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(false);
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

macro_rules! define_srl_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<srl_ $reg>], $op, std::concat!("SRL ", std::stringify!($reg)), true, 2, {
				1 => {
					let carry = state.registers.$reg & 0b1 == 1;
					state.registers.$reg >>= 1;

					state.registers.set_zero(state.registers.$reg == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_srl_reg!(0x38, b);
define_srl_reg!(0x39, c);
define_srl_reg!(0x3a, d);
define_srl_reg!(0x3b, e);
define_srl_reg!(0x3c, h);
define_srl_reg!(0x3d, l);
define_srl_reg!(0x3f, a);

macro_rules! define_rr_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<rr_ $reg>], $op, std::concat!("RR ", std::stringify!($reg)), true, 2, {
				1 => {
					let carry = state.registers.$reg & 0b1 == 1;
					state.registers.$reg >>= 1;

					state.registers.set_zero(state.registers.$reg == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_rr_reg!(0x18, b);
define_rr_reg!(0x19, c);
define_rr_reg!(0x1a, d);
define_rr_reg!(0x1b, e);
define_rr_reg!(0x1c, h);
define_rr_reg!(0x1d, l);
define_rr_reg!(0x1f, a);

macro_rules! define_res_idx_reg {
	($op:literal, $idx:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<res_ $idx _ $reg>], $op, std::concat!("RES ", std::stringify!($idx), ", ", std::stringify!($reg)), true, 2, {
				1 => {
					state.registers.$reg &= !(1u8 << $idx);
					CycleResult::Finished
				}
			});
		}
	};
}

define_res_idx_reg!(0x80, 0, b);
define_res_idx_reg!(0x81, 0, c);
define_res_idx_reg!(0x82, 0, d);
define_res_idx_reg!(0x83, 0, e);
define_res_idx_reg!(0x84, 0, h);
define_res_idx_reg!(0x85, 0, l);
define_res_idx_reg!(0x87, 0, a);
define_res_idx_reg!(0x88, 1, b);
define_res_idx_reg!(0x89, 1, c);
define_res_idx_reg!(0x8a, 1, d);
define_res_idx_reg!(0x8b, 1, e);
define_res_idx_reg!(0x8c, 1, h);
define_res_idx_reg!(0x8d, 1, l);
define_res_idx_reg!(0x8f, 1, a);
define_res_idx_reg!(0x90, 2, b);
define_res_idx_reg!(0x91, 2, c);
define_res_idx_reg!(0x92, 2, d);
define_res_idx_reg!(0x93, 2, e);
define_res_idx_reg!(0x94, 2, h);
define_res_idx_reg!(0x95, 2, l);
define_res_idx_reg!(0x97, 2, a);
define_res_idx_reg!(0x98, 3, b);
define_res_idx_reg!(0x99, 3, c);
define_res_idx_reg!(0x9a, 3, d);
define_res_idx_reg!(0x9b, 3, e);
define_res_idx_reg!(0x9c, 3, h);
define_res_idx_reg!(0x9d, 3, l);
define_res_idx_reg!(0x9f, 3, a);
define_res_idx_reg!(0xa0, 4, b);
define_res_idx_reg!(0xa1, 4, c);
define_res_idx_reg!(0xa2, 4, d);
define_res_idx_reg!(0xa3, 4, e);
define_res_idx_reg!(0xa4, 4, h);
define_res_idx_reg!(0xa5, 4, l);
define_res_idx_reg!(0xa7, 4, a);
define_res_idx_reg!(0xa8, 5, b);
define_res_idx_reg!(0xa9, 5, c);
define_res_idx_reg!(0xaa, 5, d);
define_res_idx_reg!(0xab, 5, e);
define_res_idx_reg!(0xac, 5, h);
define_res_idx_reg!(0xad, 5, l);
define_res_idx_reg!(0xaf, 5, a);
define_res_idx_reg!(0xb0, 6, b);
define_res_idx_reg!(0xb1, 6, c);
define_res_idx_reg!(0xb2, 6, d);
define_res_idx_reg!(0xb3, 6, e);
define_res_idx_reg!(0xb4, 6, h);
define_res_idx_reg!(0xb5, 6, l);
define_res_idx_reg!(0xb7, 6, a);
define_res_idx_reg!(0xb8, 7, b);
define_res_idx_reg!(0xb9, 7, c);
define_res_idx_reg!(0xba, 7, d);
define_res_idx_reg!(0xbb, 7, e);
define_res_idx_reg!(0xbc, 7, h);
define_res_idx_reg!(0xbd, 7, l);
define_res_idx_reg!(0xbf, 7, a);

macro_rules! define_res_idx_deref_hl {
	($op:literal, $idx:literal) => {
		paste::paste! {
			opcode!([<res_ $idx _deref_hl>], $op, std::concat!("RES ", std::stringify!($idx), ", (HL)"), true, 2, {
				1 => {
					state.cpu_read_u8(state.registers.get_hl());
					CycleResult::NeedsMore
				},
				2 => {
					let res = state.registers.take_mem() & !(1u8 << $idx);
					state.cpu_write_u8(state.registers.get_hl(), res);
					CycleResult::NeedsMore
				},
				3 => {
					CycleResult::Finished
				}
			});
		}
	};
}

define_res_idx_deref_hl!(0x86, 0);
define_res_idx_deref_hl!(0x8E, 1);
define_res_idx_deref_hl!(0x96, 2);
define_res_idx_deref_hl!(0x9E, 3);
define_res_idx_deref_hl!(0xA6, 4);
define_res_idx_deref_hl!(0xAE, 5);
define_res_idx_deref_hl!(0xB6, 6);
define_res_idx_deref_hl!(0xBE, 7);

macro_rules! define_set_idx_reg {
	($op:literal, $idx:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<set_ $idx _ $reg>], $op, std::concat!("SET ", std::stringify!($idx), ", ", std::stringify!($reg)), true, 2, {
				1 => {
					state.registers.$reg |= 1u8 << $idx;
					CycleResult::Finished
				}
			});
		}
	};
}

define_set_idx_reg!(0xc0, 0, b);
define_set_idx_reg!(0xc1, 0, c);
define_set_idx_reg!(0xc2, 0, d);
define_set_idx_reg!(0xc3, 0, e);
define_set_idx_reg!(0xc4, 0, h);
define_set_idx_reg!(0xc5, 0, l);
define_set_idx_reg!(0xc7, 0, a);
define_set_idx_reg!(0xc8, 1, b);
define_set_idx_reg!(0xc9, 1, c);
define_set_idx_reg!(0xca, 1, d);
define_set_idx_reg!(0xcb, 1, e);
define_set_idx_reg!(0xcc, 1, h);
define_set_idx_reg!(0xcd, 1, l);
define_set_idx_reg!(0xcf, 1, a);
define_set_idx_reg!(0xd0, 2, b);
define_set_idx_reg!(0xd1, 2, c);
define_set_idx_reg!(0xd2, 2, d);
define_set_idx_reg!(0xd3, 2, e);
define_set_idx_reg!(0xd4, 2, h);
define_set_idx_reg!(0xd5, 2, l);
define_set_idx_reg!(0xd7, 2, a);
define_set_idx_reg!(0xd8, 3, b);
define_set_idx_reg!(0xd9, 3, c);
define_set_idx_reg!(0xda, 3, d);
define_set_idx_reg!(0xdb, 3, e);
define_set_idx_reg!(0xdc, 3, h);
define_set_idx_reg!(0xdd, 3, l);
define_set_idx_reg!(0xdf, 3, a);
define_set_idx_reg!(0xe0, 4, b);
define_set_idx_reg!(0xe1, 4, c);
define_set_idx_reg!(0xe2, 4, d);
define_set_idx_reg!(0xe3, 4, e);
define_set_idx_reg!(0xe4, 4, h);
define_set_idx_reg!(0xe5, 4, l);
define_set_idx_reg!(0xe7, 4, a);
define_set_idx_reg!(0xe8, 5, b);
define_set_idx_reg!(0xe9, 5, c);
define_set_idx_reg!(0xea, 5, d);
define_set_idx_reg!(0xeb, 5, e);
define_set_idx_reg!(0xec, 5, h);
define_set_idx_reg!(0xed, 5, l);
define_set_idx_reg!(0xef, 5, a);
define_set_idx_reg!(0xf0, 6, b);
define_set_idx_reg!(0xf1, 6, c);
define_set_idx_reg!(0xf2, 6, d);
define_set_idx_reg!(0xf3, 6, e);
define_set_idx_reg!(0xf4, 6, h);
define_set_idx_reg!(0xf5, 6, l);
define_set_idx_reg!(0xf7, 6, a);
define_set_idx_reg!(0xf8, 7, b);
define_set_idx_reg!(0xf9, 7, c);
define_set_idx_reg!(0xfa, 7, d);
define_set_idx_reg!(0xfb, 7, e);
define_set_idx_reg!(0xfc, 7, h);
define_set_idx_reg!(0xfd, 7, l);
define_set_idx_reg!(0xff, 7, a);

macro_rules! define_set_idx_deref_hl {
	($op:literal, $idx:literal) => {
		paste::paste! {
			opcode!([<set_ $idx _deref_hl>], $op, std::concat!("SET ", std::stringify!($idx), ", (HL)"), true, 2, {
				1 => {
					state.cpu_read_u8(state.registers.get_hl());
					CycleResult::NeedsMore
				},
				2 => {
					let res = state.registers.take_mem() | (1u8 << $idx);
					state.cpu_write_u8(state.registers.get_hl(), res);
					CycleResult::NeedsMore
				},
				3 => {
					CycleResult::Finished
				}
			});
		}
	};
}

define_set_idx_deref_hl!(0xC6, 0);
define_set_idx_deref_hl!(0xCE, 1);
define_set_idx_deref_hl!(0xD6, 2);
define_set_idx_deref_hl!(0xDE, 3);
define_set_idx_deref_hl!(0xE6, 4);
define_set_idx_deref_hl!(0xEE, 5);
define_set_idx_deref_hl!(0xF6, 6);
define_set_idx_deref_hl!(0xFE, 7);
