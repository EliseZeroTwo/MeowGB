use deemgee_opcode::opcode;

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
	let half_carry = (lhs & 0xF) < (rhs & 0xF) + carry_u8;

	CarryResult { result, carry, half_carry }
}

pub fn add_with_carry(lhs: u8, rhs: u8, carry: bool) -> CarryResult {
	let carry_u8 = carry as u8;

	let (first_res, first_carry) = lhs.overflowing_add(rhs);
	let (result, second_carry) = first_res.overflowing_add(carry_u8);

	let carry = first_carry || second_carry;
	let half_carry = (lhs & 0xF) + (rhs & 0xF) > 0xF;

	CarryResult { result, carry, half_carry }
}

pub fn add(lhs: u8, rhs: u8) -> CarryResult {
	let (result, carry) = lhs.overflowing_add(rhs);
	let half_carry = (lhs & 0xF) + (rhs & 0xF) > 0xF;

	CarryResult { result, carry, half_carry }
}

pub fn sub(lhs: u8, rhs: u8) -> CarryResult {
	let (result, carry) = lhs.overflowing_sub(rhs);
	let half_carry = (lhs & 0xF) < (rhs & 0xF);

	CarryResult { result, carry, half_carry }
}

macro_rules! define_xor_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<xor_a_ $reg>], $op, std::concat!("XOR A,",std::stringify!($reg)), false, 1, {
				0 => {
					state.registers.a ^= state.registers.$reg;
					state.registers.set_zero(state.registers.a == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(false);
					state.registers.set_carry(false);
					CycleResult::Finished
				}
			});
		}
	};
}

define_xor_reg!(0xAF, a);
define_xor_reg!(0xA8, b);
define_xor_reg!(0xA9, c);
define_xor_reg!(0xAA, d);
define_xor_reg!(0xAB, e);
define_xor_reg!(0xAC, h);
define_xor_reg!(0xAD, l);

opcode!(xor_a_deref_hl, 0xAE, "XOR A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		state.registers.a ^= state.registers.take_mem();
		state.registers.set_zero(state.registers.a == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(false);
		CycleResult::Finished
	}
});

opcode!(xor_a_imm_u8, 0xEE, "XOR A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
	1 => {
		state.registers.a ^= state.registers.take_mem();
		state.registers.set_zero(state.registers.a == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(false);
		CycleResult::Finished
	}
});

macro_rules! define_sbc_reg {
    ($op:literal, $reg:ident) => {
        paste::paste! {
            opcode!([<sbc_a_ $reg>], $op, std::concat!("SBC A,", std::stringify!($reg)), false, 1, {
				0 => {
					let CarryResult { result, half_carry, carry } = sub_with_carry(state.registers.a, state.registers.$reg, state.registers.get_carry());

					state.registers.a = result;
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(true);
					state.registers.set_half_carry(half_carry);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
            });
        }
    };
}

define_sbc_reg!(0x9F, a);
define_sbc_reg!(0x98, b);
define_sbc_reg!(0x99, c);
define_sbc_reg!(0x9A, d);
define_sbc_reg!(0x9B, e);
define_sbc_reg!(0x9C, h);
define_sbc_reg!(0x9D, l);

opcode!(sbc_a_deref_hl, 0x9E, "SBC A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
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
		CycleResult::Finished
	}
});

opcode!(sbc_a_imm_u8, 0xDE, "SBC A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
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
		CycleResult::Finished
	}
});

macro_rules! define_add_reg {
    ($op:literal, $reg:ident) => {
        paste::paste! {
            opcode!([<add_a_ $reg>], $op, std::concat!("ADD A,", std::stringify!($reg)), false, 1, {
                    0 => {
                        let CarryResult { result, half_carry, carry } = add(state.registers.a, state.registers.$reg);

                        state.registers.a = result;
                        state.registers.set_zero(result == 0);
                        state.registers.set_subtract(false);
                        state.registers.set_half_carry(half_carry);
                        state.registers.set_carry(carry);
                        CycleResult::Finished
                    }
            });
        }
    };
}

define_add_reg!(0x87, a);
define_add_reg!(0x80, b);
define_add_reg!(0x81, c);
define_add_reg!(0x82, d);
define_add_reg!(0x83, e);
define_add_reg!(0x84, h);
define_add_reg!(0x85, l);

opcode!(add_a_deref_hl, 0x86, "ADD A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } =
			add(state.registers.a, state.registers.take_mem());

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

opcode!(add_a_imm_u8, 0xC6, "ADD A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } =
			add(state.registers.a, state.registers.take_mem());

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

macro_rules! define_adc_reg {
    ($op:literal, $reg:ident) => {
        paste::paste! {
		opcode!([<adc_a_ $reg>], $op, std::concat!("ADC A,", std::stringify!($reg)), false, 1, {
				0 => {
					let CarryResult { result, half_carry, carry } = add_with_carry(state.registers.a, state.registers.$reg, state.registers.get_carry());

					state.registers.a = result;
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(half_carry);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
			});
		}
    };
}

define_adc_reg!(0x8F, a);
define_adc_reg!(0x88, b);
define_adc_reg!(0x89, c);
define_adc_reg!(0x8A, d);
define_adc_reg!(0x8B, e);
define_adc_reg!(0x8C, h);
define_adc_reg!(0x8D, l);

opcode!(adc_a_deref_hl, 0x8E, "ADC A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } = add_with_carry(
			state.registers.a,
			state.registers.take_mem(),
			state.registers.get_carry(),
		);

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

opcode!(adc_a_imm_u8, 0xCE, "ADC A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } = add_with_carry(
			state.registers.a,
			state.registers.take_mem(),
			state.registers.get_carry(),
		);

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

macro_rules! define_sub_reg {
    ($op:literal, $reg:ident) => {
        paste::paste! {
            opcode!([<sub_a_ $reg>], $op, std::concat!("SUB A,", std::stringify!($reg)), false, 1, {
				0 => {
					let CarryResult { result, half_carry, carry } = sub(state.registers.a, state.registers.$reg);

					state.registers.a = result;
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(true);
					state.registers.set_half_carry(half_carry);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
            });
        }
    };
}

define_sub_reg!(0x97, a);
define_sub_reg!(0x90, b);
define_sub_reg!(0x91, c);
define_sub_reg!(0x92, d);
define_sub_reg!(0x93, e);
define_sub_reg!(0x94, h);
define_sub_reg!(0x95, l);

opcode!(sub_a_deref_hl, 0x96, "SUB A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } =
			sub(state.registers.a, state.registers.take_mem());

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(true);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

opcode!(sub_a_imm_u8, 0xD6, "SUB A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } =
			sub(state.registers.a, state.registers.take_mem());

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(true);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

macro_rules! define_inc_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<inc_ $reg>], $op, std::concat!("INC ", std::stringify!($reg)), false, 1, {
				0 => {
					let CarryResult { result, half_carry, .. } = add(
						state.registers.$reg,
						1,
					);

					state.registers.$reg = result;
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(half_carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_inc_reg!(0x04, b);
define_inc_reg!(0x0C, c);
define_inc_reg!(0x14, d);
define_inc_reg!(0x1C, e);
define_inc_reg!(0x24, h);
define_inc_reg!(0x2C, l);
define_inc_reg!(0x3C, a);

opcode!(inc_deref_hl, 0x34, "INC (HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, .. } = add(state.registers.take_mem(), 1);

		state.cpu_write_u8(state.registers.get_hl(), result);
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(half_carry);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::Finished
	}
});

macro_rules! define_dec_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<dec_ $reg>], $op, std::concat!("DEC ", std::stringify!($reg)), false, 1, {
				0 => {
					let CarryResult { result, half_carry, .. } = sub(
						state.registers.$reg,
						1,
					);

					state.registers.$reg = result;
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(true);
					state.registers.set_half_carry(half_carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_dec_reg!(0x05, b);
define_dec_reg!(0x0D, c);
define_dec_reg!(0x15, d);
define_dec_reg!(0x1D, e);
define_dec_reg!(0x25, h);
define_dec_reg!(0x2D, l);
define_dec_reg!(0x3D, a);

opcode!(dec_deref_hl, 0x35, "DEC (HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, .. } = sub(state.registers.take_mem(), 1);

		state.cpu_write_u8(state.registers.get_hl(), result);
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(half_carry);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::Finished
	}
});

opcode!(rla, 0x17, "RLA", false, 1, {
	0 => {
		let carry = state.registers.a >> 7 == 1;
		state.registers.a <<= 1;

		if state.registers.get_carry() {
			state.registers.a = state.registers.a.wrapping_add(1);
		}

		state.registers.set_zero(false);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(carry);

		CycleResult::Finished
	}
});

opcode!(rra, 0x1f, "RRA", false, 1, {
	0 => {
		let carry = state.registers.a & 0b1 == 1;
		state.registers.a >>= 1;

		if state.registers.get_carry() {
			state.registers.a = state.registers.a.wrapping_add(1 << 7);
		}

		state.registers.set_zero(false);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(carry);

		CycleResult::Finished
	}
});

macro_rules! define_inc_u16_reg {
	($op:literal, $lreg:ident, $rreg:ident) => {
		paste::paste! {
			opcode!([<inc_ $lreg $rreg>], $op, std::concat!("INC ", std::stringify!($lreg), std::stringify!($rreg)), false, 1, {
				0 => {
					let (res, carry) = state.registers.$rreg.overflowing_add(1);
					state.registers.$rreg = res;
					state.registers.set_hold(carry as u16);
					CycleResult::NeedsMore
				},
				1 => {
					if state.registers.take_hold() != 0 {
						let (res, _) = state.registers.$lreg.overflowing_add(1);
						state.registers.$lreg = res;
					}
					CycleResult::Finished
				}
			});
		}
	}
}

define_inc_u16_reg!(0x03, b, c);
define_inc_u16_reg!(0x13, d, e);
define_inc_u16_reg!(0x23, h, l);

opcode!(inc_sp, 0x33, "INC SP", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		let (res, _) = state.registers.sp.overflowing_add(1);
		state.registers.sp = res;
		CycleResult::Finished
	}
});

macro_rules! define_dec_u16_reg {
	($op:literal, $lreg:ident, $rreg:ident) => {
		paste::paste! {
			opcode!([<dec_ $lreg $rreg>], $op, std::concat!("DEC ", std::stringify!($lreg), std::stringify!($rreg)), false, 1, {
				0 => {
					let (res, carry) = state.registers.$rreg.overflowing_sub(1);
					state.registers.$rreg = res;
					state.registers.set_hold(carry as u16);
					CycleResult::NeedsMore
				},
				1 => {
					if state.registers.take_hold() != 0 {
						let (res, _) = state.registers.$lreg.overflowing_sub(1);
						state.registers.$lreg = res;
					}
					CycleResult::Finished
				}
			});
		}
	};
}

define_dec_u16_reg!(0x0B, b, c);
define_dec_u16_reg!(0x1B, d, e);
define_dec_u16_reg!(0x2B, h, l);

opcode!(dec_sp, 0x3B, "DEC SP", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		let (res, _) = state.registers.sp.overflowing_sub(1);
		state.registers.sp = res;
		CycleResult::Finished
	}
});

macro_rules! define_cp_reg_reg {
	($op:literal, $lreg:ident, $rreg:ident) => {
		paste::paste! {
			opcode!([<cp_ $lreg _ $rreg>], $op, std::concat!("CP ", std::stringify!($lreg), ",", std::stringify!($rreg)), false, 1, {
				0 => {
					let CarryResult { result, half_carry, carry } = sub(state.registers.$lreg, state.registers.$rreg);
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(true);
					state.registers.set_half_carry(half_carry);
					state.registers.set_carry(carry);
					CycleResult::Finished
				}
			});
		}
	};
}

define_cp_reg_reg!(0xB8, a, b);
define_cp_reg_reg!(0xB9, a, c);
define_cp_reg_reg!(0xBA, a, d);
define_cp_reg_reg!(0xBB, a, e);
define_cp_reg_reg!(0xBC, a, h);
define_cp_reg_reg!(0xBD, a, l);
define_cp_reg_reg!(0xBF, a, a);

opcode!(cp_a_imm_u8, 0xFE, "CP A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } =
			sub(state.registers.a, state.registers.take_mem());
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(true);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

opcode!(cp_a_deref_hl, 0xBE, "CP A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, half_carry, carry } =
			sub(state.registers.a, state.registers.take_mem());
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(true);
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

macro_rules! define_or_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<or_a_ $reg>], $op, std::concat!("OR A,", std::stringify!($reg)), false, 1, {
					0 => {
						let result = state.registers.a | state.registers.$reg;

						state.registers.a = result;
						state.registers.set_zero(result == 0);
						state.registers.set_subtract(false);
						state.registers.set_half_carry(false);
						state.registers.set_carry(false);
						CycleResult::Finished
					}
			});
		}
	};
}

define_or_reg!(0xB7, a);
define_or_reg!(0xB0, b);
define_or_reg!(0xB1, c);
define_or_reg!(0xB2, d);
define_or_reg!(0xB3, e);
define_or_reg!(0xB4, h);
define_or_reg!(0xB5, l);

opcode!(or_a_deref_hl, 0xB6, "OR A,(HL)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let result = state.registers.a | state.registers.take_mem();

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(false);
		CycleResult::Finished
	}
});

opcode!(or_a_imm_u8, 0xF6, "OR A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
	1 => {
		let result = state.registers.a | state.registers.take_mem();

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(false);
		CycleResult::Finished
	}
});

macro_rules! define_and_reg {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<and_a_ $reg>], $op, std::concat!("AND A,", std::stringify!($reg)), false, 1, {
				0 => {
					let result = state.registers.a & state.registers.$reg;

					state.registers.a = result;
					state.registers.set_zero(result == 0);
					state.registers.set_subtract(false);
					state.registers.set_half_carry(true);
					state.registers.set_carry(false);
					CycleResult::Finished
				}
			});
		}
	};
}

define_and_reg!(0xA7, a);
define_and_reg!(0xA0, b);
define_and_reg!(0xA1, c);
define_and_reg!(0xA2, d);
define_and_reg!(0xA3, e);
define_and_reg!(0xA4, h);
define_and_reg!(0xA5, l);

opcode!(and_a_deref_hl, 0xA6, "AND A,(hl)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		let result = state.registers.a & state.registers.take_mem();

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(true);
		state.registers.set_carry(false);
		CycleResult::Finished
	}
});

opcode!(and_a_imm_u8, 0xE6, "AND A,u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc + 1);
		CycleResult::NeedsMore
	},
	1 => {
		let result = state.registers.a & state.registers.take_mem();

		state.registers.a = result;
		state.registers.set_zero(result == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(true);
		state.registers.set_carry(false);
		CycleResult::Finished
	}
});

opcode!(cpl, 0x2F, "CPL", false, 1, {
	0 => {
		state.registers.a = !state.registers.a;
		state.registers.set_subtract(true);
		state.registers.set_half_carry(true);
		CycleResult::Finished
	}
});

opcode!(ccf, 0x3F, "CCF", false, 1, {
	0 => {
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(!state.registers.get_carry());
		CycleResult::Finished
	}
});

opcode!(scf, 0x37, "SCF", false, 1, {
	0 => {
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(true);
		CycleResult::Finished
	}
});

macro_rules! define_add_hl_u16_reg {
	($op:literal, $lreg:ident, $rreg:ident) => {
		paste::paste! {
			opcode!([<add_hl_ $lreg $rreg>], $op, std::concat!("ADD HL, ", std::stringify!($lreg), std::stringify!($rreg)), false, 1, {
				0 => {
					let CarryResult { result, carry, .. } = add(state.registers.l, state.registers.$rreg);
					state.registers.l = result;
					state.registers.set_hold(carry as u16);
					CycleResult::NeedsMore
				},
				1 => {
					let CarryResult { result, carry, half_carry } = add(state.registers.h, state.registers.$lreg);
					state.registers.h = result;
					state.registers.set_half_carry(half_carry);
					state.registers.set_carry(carry);

					if state.registers.take_hold() != 0 {
						let CarryResult { result, carry: s_carry, half_carry: s_half_carry } = add(state.registers.h, 1);
						state.registers.h = result;
						state.registers.set_half_carry(half_carry || s_half_carry);
						state.registers.set_carry(carry || s_carry);
					}

					state.registers.set_subtract(false);
					CycleResult::Finished
				}
			});
		}
	};
}

define_add_hl_u16_reg!(0x09, b, c);
define_add_hl_u16_reg!(0x19, d, e);
define_add_hl_u16_reg!(0x29, h, l);
opcode!(add_hl_sp, 0x39, "ADD HL, SP", false, 1, {
	0 => {
		let CarryResult { result, carry, .. } =
			add(state.registers.l, state.registers.sp as u8);
		state.registers.l = result;
		state.registers.set_hold(carry as u16);
		CycleResult::NeedsMore
	},
	1 => {
		let CarryResult { result, carry, half_carry } = add(state.registers.h, (state.registers.sp >> 8) as u8);
		state.registers.h = result;
		state.registers.set_half_carry(half_carry);
		state.registers.set_carry(carry);

		if state.registers.take_hold() != 0 {
			let CarryResult { result, carry: s_carry, half_carry: s_half_carry } = add(state.registers.h, 1);
			state.registers.h = result;
			state.registers.set_half_carry(half_carry || s_half_carry);
			state.registers.set_carry(carry || s_carry);
		}

		state.registers.set_subtract(false);
		CycleResult::Finished
	}
});

opcode!(rlca, 0x7, "RLCA", false, 1, {
	0 => {
		let carry = state.registers.a >> 7 == 1;
		state.registers.a <<= 1;
		state.registers.a |= carry as u8;

		state.registers.set_zero(state.registers.a == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

opcode!(rrca, 0xF, "RRCA", false, 1, {
	0 => {
		let carry = state.registers.a & 0b1 == 1;
		state.registers.a >>= 1;
		state.registers.a |= (carry as u8) << 7;

		state.registers.set_zero(state.registers.a == 0);
		state.registers.set_subtract(false);
		state.registers.set_half_carry(false);
		state.registers.set_carry(carry);
		CycleResult::Finished
	}
});

opcode!(daa, 0x27, "DAA", false, 1, {
	0 => {
		let mut value = 0;
		let mut carry = false;

		if state.registers.get_half_carry() || (!state.registers.get_subtract() && (state.registers.a & 0xF) > 9) {
			value |= 0x06;
		}

		if state.registers.get_carry() || (!state.registers.get_subtract() && state.registers.a > 0x99) {
			value |= 0x60;
			carry = true;
		}

		state.registers.a = match state.registers.get_subtract() {
			true => state.registers.a.wrapping_sub(value),
			false => state.registers.a.wrapping_add(value)
		};

		state.registers.set_half_carry(false);
		state.registers.set_carry(carry);
		state.registers.set_zero(state.registers.a == 0);
		CycleResult::Finished
	}
});
