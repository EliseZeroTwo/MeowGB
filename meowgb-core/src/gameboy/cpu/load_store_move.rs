use meowgb_opcode::opcode;

use crate::gameboy::{cpu::CycleResult, Gameboy};

macro_rules! define_ld_reg_imm_u16 {
	($op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<ld_ $reg _imm_u16>], $op, std::concat!("LD ", std::stringify!($reg), ",u16"), false, 3, {
				0 => {
					state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
					CycleResult::NeedsMore
				},
				1 => {
					let mut reg = state.registers.[<get_ $reg>]();
					reg &= 0xFF00;
					reg |= state.registers.take_mem() as u16;
					state.registers.[<set_ $reg>](reg);
					state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
					CycleResult::NeedsMore
				},
				2 => {
					let mut reg = state.registers.[<get_ $reg>]();
					reg &= 0xFF;
					reg |= (state.registers.take_mem() as u16) << 8;
					state.registers.[<set_ $reg>](reg);
					CycleResult::Finished
				}
			});
		}
	};
}

define_ld_reg_imm_u16!(0x01, bc);
define_ld_reg_imm_u16!(0x11, de);
define_ld_reg_imm_u16!(0x21, hl);
define_ld_reg_imm_u16!(0x31, sp);

opcode!(ld_sp_hl, 0xF9, "LD SP,HL", false, 1, {
		0 => {
			state.registers.sp &= 0xFF00;
			state.registers.sp |= state.registers.l as u16;
			CycleResult::NeedsMore
		},
		1 => {
			state.registers.sp &= 0xFF;
			state.registers.sp |= (state.registers.h as u16) << 8;
			CycleResult::Finished
		}
});

opcode!(ld_deref_imm_u16_sp, 0x08, "LD (u16),SP", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let low_addr = state.registers.take_mem() as u16;
		state.registers.set_hold(low_addr);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		let addr = ((state.registers.take_mem() as u16) << 8) | state.registers.take_hold();
		state.registers.set_hold(addr);
		state.cpu_write_u8(addr, state.registers.sp as u8);
		CycleResult::NeedsMore
	},
	3 => {
		let addr = state.registers.take_hold().overflowing_add(1).0;
		state.cpu_write_u8(addr, (state.registers.sp >> 8) as u8);
		CycleResult::Finished
	}
});

macro_rules! define_ld_reg_reg {
	($opcode:literal, $lreg:ident, $rreg:ident) => {
		paste::paste! {
			opcode!([<ld_ $lreg _ $rreg>], $opcode, std::concat!("LD ", std::stringify!($lreg), ",", std::stringify!($rreg)), false, 1, {
					0 => {
						let res = state.registers.$rreg;
						state.registers.$lreg = res;
						CycleResult::Finished
					}
			});
		}
	};
}

define_ld_reg_reg!(0x40, b, b);
define_ld_reg_reg!(0x41, b, c);
define_ld_reg_reg!(0x42, b, d);
define_ld_reg_reg!(0x43, b, e);
define_ld_reg_reg!(0x44, b, h);
define_ld_reg_reg!(0x45, b, l);
define_ld_reg_reg!(0x47, b, a);
define_ld_reg_reg!(0x48, c, b);
define_ld_reg_reg!(0x49, c, c);
define_ld_reg_reg!(0x4A, c, d);
define_ld_reg_reg!(0x4B, c, e);
define_ld_reg_reg!(0x4C, c, h);
define_ld_reg_reg!(0x4D, c, l);
define_ld_reg_reg!(0x4F, c, a);
define_ld_reg_reg!(0x50, d, b);
define_ld_reg_reg!(0x51, d, c);
define_ld_reg_reg!(0x52, d, d);
define_ld_reg_reg!(0x53, d, e);
define_ld_reg_reg!(0x54, d, h);
define_ld_reg_reg!(0x55, d, l);
define_ld_reg_reg!(0x57, d, a);
define_ld_reg_reg!(0x58, e, b);
define_ld_reg_reg!(0x59, e, c);
define_ld_reg_reg!(0x5A, e, d);
define_ld_reg_reg!(0x5B, e, e);
define_ld_reg_reg!(0x5C, e, h);
define_ld_reg_reg!(0x5D, e, l);
define_ld_reg_reg!(0x5F, e, a);
define_ld_reg_reg!(0x60, h, b);
define_ld_reg_reg!(0x61, h, c);
define_ld_reg_reg!(0x62, h, d);
define_ld_reg_reg!(0x63, h, e);
define_ld_reg_reg!(0x64, h, h);
define_ld_reg_reg!(0x65, h, l);
define_ld_reg_reg!(0x67, h, a);
define_ld_reg_reg!(0x68, l, b);
define_ld_reg_reg!(0x69, l, c);
define_ld_reg_reg!(0x6A, l, d);
define_ld_reg_reg!(0x6B, l, e);
define_ld_reg_reg!(0x6C, l, h);
define_ld_reg_reg!(0x6D, l, l);
define_ld_reg_reg!(0x6F, l, a);
define_ld_reg_reg!(0x78, a, b);
define_ld_reg_reg!(0x79, a, c);
define_ld_reg_reg!(0x7A, a, d);
define_ld_reg_reg!(0x7B, a, e);
define_ld_reg_reg!(0x7C, a, h);
define_ld_reg_reg!(0x7D, a, l);
define_ld_reg_reg!(0x7F, a, a);

macro_rules! define_ld_reg_deref {
	($op:literal, $lreg:ident, $rreg:ident) => {
		paste::paste! {
			opcode!([<ld_ $lreg _deref_ $rreg>], $op, std::concat!("LD ", std::stringify!($lreg), ",(", std::stringify!($rreg), ")"), false, 1, {
				0 => {
					state.cpu_read_u8(state.registers.[<get_ $rreg>]());
					CycleResult::NeedsMore
				},
				1 => {
					state.registers.$lreg = state.registers.take_mem();
					CycleResult::Finished
				}
			});
		}
	};
}

define_ld_reg_deref!(0x46, b, hl);
define_ld_reg_deref!(0x4E, c, hl);
define_ld_reg_deref!(0x56, d, hl);
define_ld_reg_deref!(0x5E, e, hl);
define_ld_reg_deref!(0x66, h, hl);
define_ld_reg_deref!(0x6E, l, hl);
define_ld_reg_deref!(0x7E, a, hl);
define_ld_reg_deref!(0x0A, a, bc);
define_ld_reg_deref!(0x1A, a, de);

opcode!(ld_deref_bc_a, 0x02, "LD (BC),A", false, 1, {
	0 => {
		state.cpu_write_u8(state.registers.get_bc(), state.registers.a);
		CycleResult::NeedsMore
	},
	1 => {
		CycleResult::Finished
	}
});

opcode!(ld_deref_de_a, 0x12, "LD (DE),A", false, 1, {
	0 => {
		state.cpu_write_u8(state.registers.get_de(), state.registers.a);
		CycleResult::NeedsMore
	},
	1 => {
		CycleResult::Finished
	}
});

opcode!(ld_hl_plus_a, 0x22, "LD (HL+),A", false, 1, {
	0 => {
		state.cpu_write_u8(state.registers.get_hl(), state.registers.a);
		CycleResult::NeedsMore
	},
	1 => {
		let reg = state.registers.get_hl().overflowing_add(1).0;
		state.registers.set_hl(reg);
		CycleResult::Finished
	}
});

opcode!(ld_hl_minus_a, 0x32, "LD (HL-),A", false, 1, {
	0 => {
		state.cpu_write_u8(state.registers.get_hl(), state.registers.a);
		CycleResult::NeedsMore
	},
	1 => {
		let reg = state.registers.get_hl().overflowing_sub(1).0;
		state.registers.set_hl(reg);
		CycleResult::Finished
	}
});

opcode!(ld_a_hl_plus, 0x2A, "LD A,(HL+)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		state.registers.a = state.registers.take_mem();
		let reg = state.registers.get_hl().overflowing_add(1).0;
		state.registers.set_hl(reg);
		CycleResult::Finished
	}
});

opcode!(ld_a_hl_minus, 0x3A, "LD A,(HL-)", false, 1, {
	0 => {
		state.cpu_read_u8(state.registers.get_hl());
		CycleResult::NeedsMore
	},
	1 => {
		state.registers.a = state.registers.take_mem();
		let reg = state.registers.get_hl().overflowing_sub(1).0;
		state.registers.set_hl(reg);
		CycleResult::Finished
	}
});

opcode!(ld_hl_sp_i8, 0xF8, "LD HL,SP+i8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let val = state.registers.take_mem() as i8;

		let rhs = if val < 0 {
			state.registers.sp.overflowing_sub(val.abs() as u16).0
		} else {
			state.registers.sp.overflowing_add(val as u16).0
		};

		state.registers.set_hl(rhs);
		state.registers.set_zero(false);
		state.registers.set_subtract(false);
		state.registers.set_half_carry((state.registers.sp & 0xF) + (val as u16 & 0xF) > 0xF);
		state.registers.set_carry((state.registers.sp & 0xFF) + (val as u16 & 0xFF) > 0xFF);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::Finished
	}
});

macro_rules! define_ld_reg_imm_u8 {
	($op:literal, $lreg:ident) => {
		paste::paste! {
			opcode!([<ld_ $lreg _imm_u8>], $op, std::concat!("LD ", std::stringify!($lreg), ",u8"), false, 2, {
				0 => {
					state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
					CycleResult::NeedsMore
				},
				1 => {
					state.registers.$lreg = state.registers.take_mem();
					CycleResult::Finished
				}
			});
		}
	};
}

define_ld_reg_imm_u8!(0x06, b);
define_ld_reg_imm_u8!(0x0E, c);
define_ld_reg_imm_u8!(0x16, d);
define_ld_reg_imm_u8!(0x1E, e);
define_ld_reg_imm_u8!(0x26, h);
define_ld_reg_imm_u8!(0x2E, l);
define_ld_reg_imm_u8!(0x3E, a);

opcode!(ld_deref_hl_imm_u8, 0x36, "LD (HL),u8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let imm = state.registers.take_mem();
		state.cpu_write_u8(state.registers.get_hl(), imm);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::Finished
	}
});

opcode!(ldh_a_imm_u8, 0xF0, "LDH A,(u8)", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let imm = state.registers.take_mem();
		let addr = 0xFF00u16 | imm as u16;
		state.cpu_read_u8(addr);
		CycleResult::NeedsMore
	},
	2 => {
		state.registers.a = state.registers.take_mem();
		CycleResult::Finished
	}
});

opcode!(ldh_imm_u8_a, 0xE0, "LDH (u8),A", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let imm = state.registers.take_mem();
		let addr = 0xFF00u16 | imm as u16;
		state.cpu_write_u8(addr, state.registers.a);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::Finished
	}
});

opcode!(ldh_a_deref_c, 0xF2, "LDH A,(C)", false, 1, {
	0 => {
		let imm = state.registers.c;
		let addr = 0xFF00u16 | imm as u16;
		state.cpu_read_u8(addr);
		CycleResult::NeedsMore
	},
	1 => {
		state.registers.a = state.registers.take_mem();
		CycleResult::Finished
	}
});

opcode!(ldh_deref_c_a, 0xE2, "LDH (C),A", false, 1, {
	0 => {
		let addr = 0xFF00u16 | state.registers.c as u16;
		state.cpu_write_u8(addr, state.registers.a);
		CycleResult::NeedsMore
	},
	1 => {
		CycleResult::Finished
	}
});

opcode!(ld_a_deref_imm_u16, 0xFA, "LD A,(u16)", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lsb = state.registers.take_mem() as u16;
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		state.registers.set_hold(lsb);
		CycleResult::NeedsMore
	},
	2 => {
		let addr = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.cpu_read_u8(addr);
		CycleResult::NeedsMore
	},
	3 => {
		state.registers.a = state.registers.take_mem();
		CycleResult::Finished
	}
});

opcode!(ld_deref_imm_u16_a, 0xEA, "LD (u16),A", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lsb = state.registers.take_mem() as u16;
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		state.registers.set_hold(lsb);
		CycleResult::NeedsMore
	},
	2 => {
		let addr = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.cpu_write_u8(addr, state.registers.a);
		CycleResult::NeedsMore
	},
	3 => {
		CycleResult::Finished
	}
});

macro_rules! define_ld_deref_hl_reg {
	($op:literal, $lreg:ident) => {
		paste::paste! {
			opcode!([<ld_deref_hl_ $lreg>], $op, std::concat!("LD (HL),", std::stringify!($lreg)), false, 1, {
					0 => {
						state.cpu_write_u8(state.registers.get_hl(), state.registers.$lreg);
						CycleResult::NeedsMore
					},
					1 => {
						CycleResult::Finished
					}
			});
		}
	};
}

define_ld_deref_hl_reg!(0x70, b);
define_ld_deref_hl_reg!(0x71, c);
define_ld_deref_hl_reg!(0x72, d);
define_ld_deref_hl_reg!(0x73, e);
define_ld_deref_hl_reg!(0x74, h);
define_ld_deref_hl_reg!(0x75, l);
define_ld_deref_hl_reg!(0x77, a);

macro_rules! define_push_pop_reg {
	($push_op:literal, $pop_op:literal, $reg:ident) => {
		paste::paste! {
			opcode!([<push_ $reg>], $push_op, std::concat!("PUSH ", std::stringify!($reg)), false, 1, {
					0 => {
						CycleResult::NeedsMore
					},
					1 => {
						state.cpu_push_stack((state.registers.[<get_ $reg>]() >> 8) as u8);
						CycleResult::NeedsMore
					},
					2 => {
						state.cpu_push_stack(state.registers.[<get_ $reg>]() as u8);
						CycleResult::NeedsMore
					},
					3 => {
						CycleResult::Finished
					}
			});

			opcode!([<pop_ $reg>], $pop_op, std::concat!("POP ", std::stringify!($reg)), false, 1, {
					0 => {
						state.cpu_pop_stack();
						CycleResult::NeedsMore
					},
					1 => {
						let lsb = state.registers.take_mem() as u16;
						state.registers.set_hold(lsb);
						state.cpu_pop_stack();
						CycleResult::NeedsMore
					},
					2 => {
						let val = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
						state.registers.[<set_ $reg>](val);
						CycleResult::Finished
					}
			});
		}
	};
}

define_push_pop_reg!(0xC5, 0xC1, bc);
define_push_pop_reg!(0xD5, 0xD1, de);
define_push_pop_reg!(0xE5, 0xE1, hl);
define_push_pop_reg!(0xF5, 0xF1, af);
