use crate::gameboy::{cpu::CycleResult, Gameboy};

macro_rules! define_ld_reg_imm_u16 {
	($reg:ident) => {
		paste::paste! {
			pub fn [<ld_ $reg _imm_u16>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
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
						state.registers.opcode_bytecount = Some(3);
						CycleResult::Finished
					},
					_ => unreachable!(),
				}
			}
		}
	};
}

define_ld_reg_imm_u16!(bc);
define_ld_reg_imm_u16!(de);
define_ld_reg_imm_u16!(hl);
define_ld_reg_imm_u16!(sp);

pub fn ld_sp_hl(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.registers.sp &= 0xFF00;
			state.registers.sp |= state.registers.l as u16;
			CycleResult::NeedsMore
		}
		1 => {
			state.registers.sp &= 0xFF;
			state.registers.sp |= (state.registers.h as u16) << 8;
			state.registers.opcode_bytecount = Some(3);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ld_deref_imm_u16_sp(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let low_addr = state.registers.take_mem() as u16;
			state.registers.set_hold(low_addr);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			let addr = ((state.registers.take_mem() as u16) << 8) | state.registers.take_hold();
			state.registers.set_hold(addr);
			state.cpu_write_u8(addr, state.registers.sp as u8);
			CycleResult::NeedsMore
		}
		3 => {
			let addr = state.registers.take_hold().overflowing_add(1).0;
			state.cpu_write_u8(addr, (state.registers.sp >> 8) as u8);
			state.registers.opcode_bytecount = Some(3);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

macro_rules! define_ld_reg_reg {
	($lreg:ident, $rreg:ident) => {
		paste::paste! {
			pub fn [<ld_ $lreg _ $rreg>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => {
						let res = state.registers.$rreg;
						state.registers.$lreg = res;
						state.registers.opcode_bytecount = Some(1);
						CycleResult::Finished
					},
					_ => unreachable!(),
				}
			}
		}
	};
}

macro_rules! define_ld_reg_regs {
	($lreg:ident) => {
		define_ld_reg_reg!($lreg, b);
		define_ld_reg_reg!($lreg, c);
		define_ld_reg_reg!($lreg, d);
		define_ld_reg_reg!($lreg, e);
		define_ld_reg_reg!($lreg, h);
		define_ld_reg_reg!($lreg, l);
		define_ld_reg_reg!($lreg, a);
	};
}

define_ld_reg_regs!(b);
define_ld_reg_regs!(c);
define_ld_reg_regs!(d);
define_ld_reg_regs!(e);
define_ld_reg_regs!(h);
define_ld_reg_regs!(l);
define_ld_reg_regs!(a);

macro_rules! define_ld_reg_deref {
	($lreg:ident, $rreg:ident) => {
		paste::paste! {
			pub fn [<ld_ $lreg _deref_ $rreg>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => {
						state.cpu_read_u8(state.registers.[<get_ $rreg>]());
						CycleResult::NeedsMore
					},
					1 => {
						state.registers.$lreg = state.registers.take_mem();
						state.registers.opcode_bytecount = Some(1);
						CycleResult::Finished
					},
					_ => unreachable!(),
				}
			}
		}
	};
}

define_ld_reg_deref!(b, hl);
define_ld_reg_deref!(c, hl);
define_ld_reg_deref!(d, hl);
define_ld_reg_deref!(e, hl);
define_ld_reg_deref!(h, hl);
define_ld_reg_deref!(l, hl);
define_ld_reg_deref!(a, hl);
define_ld_reg_deref!(a, bc);
define_ld_reg_deref!(a, de);

pub fn ld_hl_minus_a(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_write_u8(state.registers.get_hl(), state.registers.a);
			CycleResult::NeedsMore
		}
		1 => {
			let reg = state.registers.get_hl().overflowing_sub(1).0;
			state.registers.set_hl(reg);
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ld_a_hl_minus(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.get_hl());
			CycleResult::NeedsMore
		}
		1 => {
			state.registers.a = state.registers.take_mem();
			let reg = state.registers.get_hl().overflowing_sub(1).0;
			state.registers.set_hl(reg);
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ld_hl_plus_a(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_write_u8(state.registers.get_hl(), state.registers.a);
			CycleResult::NeedsMore
		}
		1 => {
			let reg = state.registers.get_hl().overflowing_add(1).0;
			state.registers.set_hl(reg);
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ld_a_hl_plus(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.get_hl());
			CycleResult::NeedsMore
		}
		1 => {
			state.registers.a = state.registers.take_mem();
			let reg = state.registers.get_hl().overflowing_add(1).0;
			state.registers.set_hl(reg);
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

macro_rules! define_ld_reg_imm_u8 {
	($lreg:ident) => {
		paste::paste! {
			pub fn [<ld_ $lreg _imm_u8>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => {
						state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
						CycleResult::NeedsMore
					},
					1 => {
						state.registers.$lreg = state.registers.take_mem();
						state.registers.opcode_bytecount = Some(2);
						CycleResult::Finished
					},
					_ => unreachable!(),
				}
			}
		}
	};
}

define_ld_reg_imm_u8!(b);
define_ld_reg_imm_u8!(c);
define_ld_reg_imm_u8!(d);
define_ld_reg_imm_u8!(e);
define_ld_reg_imm_u8!(h);
define_ld_reg_imm_u8!(l);
define_ld_reg_imm_u8!(a);

pub fn ld_deref_hl_imm_u8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let imm = state.registers.take_mem();
			state.cpu_write_u8(state.registers.get_hl(), imm);
			state.registers.opcode_bytecount = Some(2);
			CycleResult::NeedsMore
		}
		2 => CycleResult::Finished,
		_ => unreachable!(),
	}
}

pub fn ldh_a_imm_u8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let imm = state.registers.take_mem();
			let addr = 0xFF00u16 | imm as u16;
			state.cpu_read_u8(addr);
			CycleResult::NeedsMore
		}
		2 => {
			state.registers.a = state.registers.take_mem();
			state.registers.opcode_bytecount = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ldh_imm_u8_a(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let imm = state.registers.take_mem();
			let addr = 0xFF00u16 | imm as u16;
			state.cpu_write_u8(addr, state.registers.a);
			state.registers.opcode_bytecount = Some(2);
			CycleResult::NeedsMore
		}
		2 => CycleResult::Finished,
		_ => unreachable!(),
	}
}

pub fn ldh_a_deref_c(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			let imm = state.registers.c;
			let addr = 0xFF00u16 | imm as u16;
			state.cpu_read_u8(addr);
			CycleResult::NeedsMore
		}
		1 => {
			state.registers.a = state.registers.take_mem();
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ldh_deref_c_a(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			let addr = 0xFF00u16 | state.registers.c as u16;
			state.cpu_write_u8(addr, state.registers.a);
			state.registers.opcode_bytecount = Some(1);
			CycleResult::NeedsMore
		}
		1 => CycleResult::Finished,
		_ => unreachable!(),
	}
}

pub fn ld_a_deref_imm_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lsb = state.registers.take_mem() as u16;
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			state.registers.set_hold(lsb);
			CycleResult::NeedsMore
		}
		2 => {
			let addr = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.cpu_read_u8(addr);
			CycleResult::NeedsMore
		}
		3 => {
			state.registers.a = state.registers.take_mem();
			state.registers.opcode_bytecount = Some(3);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ld_deref_imm_u16_a(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lsb = state.registers.take_mem() as u16;
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			state.registers.set_hold(lsb);
			CycleResult::NeedsMore
		}
		2 => {
			let addr = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.cpu_write_u8(addr, state.registers.a);
			state.registers.opcode_bytecount = Some(3);
			CycleResult::NeedsMore
		}
		3 => CycleResult::Finished,
		_ => unreachable!(),
	}
}

macro_rules! define_ld_deref_hl_reg {
	($lreg:ident) => {
		paste::paste! {
			pub fn [<ld_deref_hl_ $lreg>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => {
						state.cpu_write_u8(state.registers.get_hl(), state.registers.$lreg);
						state.registers.opcode_bytecount = Some(1);
						CycleResult::NeedsMore
					},
					1 => CycleResult::Finished,
					_ => unreachable!(),
				}
			}
		}
	};
}

define_ld_deref_hl_reg!(b);
define_ld_deref_hl_reg!(c);
define_ld_deref_hl_reg!(d);
define_ld_deref_hl_reg!(e);
define_ld_deref_hl_reg!(h);
define_ld_deref_hl_reg!(l);
define_ld_deref_hl_reg!(a);

macro_rules! define_push_pop_reg {
	($reg:ident) => {
		paste::paste! {
			pub fn [<push_ $reg>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => CycleResult::NeedsMore,
					1 => {
						state.cpu_push_stack((state.registers.[<get_ $reg>]() >> 8) as u8);
						CycleResult::NeedsMore
					},
					2 => {
						state.cpu_push_stack(state.registers.[<get_ $reg>]() as u8);
						state.registers.opcode_bytecount = Some(1);
						CycleResult::NeedsMore
					},
					3 => CycleResult::Finished,
					_ => unreachable!(),
				}
			}

			pub fn [<pop_ $reg>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
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
						state.registers.opcode_bytecount = Some(1);
						CycleResult::Finished
					},
					_ => unreachable!(),
				}
			}
		}
	};
}

define_push_pop_reg!(bc);
define_push_pop_reg!(de);
define_push_pop_reg!(hl);
define_push_pop_reg!(af);
