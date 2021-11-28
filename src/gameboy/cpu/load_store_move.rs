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

define_ld_reg_reg!(b, b);
define_ld_reg_reg!(b, c);
define_ld_reg_reg!(b, d);
define_ld_reg_reg!(b, e);
define_ld_reg_reg!(b, h);
define_ld_reg_reg!(b, l);
define_ld_reg_reg!(b, a);

define_ld_reg_reg!(c, b);
define_ld_reg_reg!(c, c);
define_ld_reg_reg!(c, d);
define_ld_reg_reg!(c, e);
define_ld_reg_reg!(c, h);
define_ld_reg_reg!(c, l);
define_ld_reg_reg!(c, a);

define_ld_reg_reg!(d, b);
define_ld_reg_reg!(d, c);
define_ld_reg_reg!(d, d);
define_ld_reg_reg!(d, e);
define_ld_reg_reg!(d, h);
define_ld_reg_reg!(d, l);
define_ld_reg_reg!(d, a);

define_ld_reg_reg!(e, b);
define_ld_reg_reg!(e, c);
define_ld_reg_reg!(e, d);
define_ld_reg_reg!(e, e);
define_ld_reg_reg!(e, h);
define_ld_reg_reg!(e, l);
define_ld_reg_reg!(e, a);

define_ld_reg_reg!(h, b);
define_ld_reg_reg!(h, c);
define_ld_reg_reg!(h, d);
define_ld_reg_reg!(h, e);
define_ld_reg_reg!(h, h);
define_ld_reg_reg!(h, l);
define_ld_reg_reg!(h, a);

define_ld_reg_reg!(l, b);
define_ld_reg_reg!(l, c);
define_ld_reg_reg!(l, d);
define_ld_reg_reg!(l, e);
define_ld_reg_reg!(l, h);
define_ld_reg_reg!(l, l);
define_ld_reg_reg!(l, a);

define_ld_reg_reg!(a, b);
define_ld_reg_reg!(a, c);
define_ld_reg_reg!(a, d);
define_ld_reg_reg!(a, e);
define_ld_reg_reg!(a, h);
define_ld_reg_reg!(a, l);
define_ld_reg_reg!(a, a);

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
