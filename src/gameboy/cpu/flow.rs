use super::CycleResult;
use crate::gameboy::Gameboy;

pub fn jr_nz_i8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			if state.registers.get_zero() {
				state.registers.take_mem();
				state.registers.opcode_bytecount = Some(2);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		2 => {
			let relative = state.registers.take_mem() as i8;

			if relative >= 0 {
				state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
			} else {
				state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
			}

			state.registers.opcode_bytecount = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jr_nc_i8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			if state.registers.get_carry() {
				state.registers.take_mem();
				state.registers.opcode_bytecount = Some(2);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		2 => {
			let relative = state.registers.take_mem() as i8;

			if relative >= 0 {
				state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
			} else {
				state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
			}

			state.registers.opcode_bytecount = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jr_z_i8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			if !state.registers.get_zero() {
				state.registers.take_mem();
				state.registers.opcode_bytecount = Some(2);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		2 => {
			let relative = state.registers.take_mem() as i8;

			if relative >= 0 {
				state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
			} else {
				state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
			}

			state.registers.opcode_bytecount = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jr_c_i8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			if !state.registers.get_carry() {
				state.registers.take_mem();
				state.registers.opcode_bytecount = Some(2);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		2 => {
			let relative = state.registers.take_mem() as i8;

			if relative >= 0 {
				state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
			} else {
				state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
			}

			state.registers.opcode_bytecount = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jr_i8(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => CycleResult::NeedsMore,
		2 => {
			let relative = state.registers.take_mem() as i8;

			if relative >= 0 {
				state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
			} else {
				state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
			}

			state.registers.opcode_bytecount = Some(2);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jp_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => CycleResult::NeedsMore,
		3 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jp_hl(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.registers.pc = state.registers.get_hl();
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jp_nz_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if state.registers.get_zero() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jp_nc_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if state.registers.get_carry() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jp_z_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if !state.registers.get_zero() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn jp_c_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if !state.registers.get_carry() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn call_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => CycleResult::NeedsMore,
		3 => {
			state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
			CycleResult::NeedsMore
		}
		4 => {
			state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
			CycleResult::NeedsMore
		}
		5 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn call_nz_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if state.registers.get_zero() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
			CycleResult::NeedsMore
		}
		4 => {
			state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
			CycleResult::NeedsMore
		}
		5 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn call_nc_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if state.registers.get_carry() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
			CycleResult::NeedsMore
		}
		4 => {
			state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
			CycleResult::NeedsMore
		}
		5 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn call_z_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if !state.registers.get_zero() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
			CycleResult::NeedsMore
		}
		4 => {
			state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
			CycleResult::NeedsMore
		}
		5 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn call_c_u16(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
			CycleResult::NeedsMore
		}
		1 => {
			let lower = state.registers.take_mem() as u16;
			state.registers.set_hold(lower);
			state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
			CycleResult::NeedsMore
		}
		2 => {
			if !state.registers.get_carry() {
				state.registers.take_mem();
				state.registers.take_hold();
				state.registers.opcode_bytecount = Some(3);
				CycleResult::Finished
			} else {
				CycleResult::NeedsMore
			}
		}
		3 => {
			state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
			CycleResult::NeedsMore
		}
		4 => {
			state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
			CycleResult::NeedsMore
		}
		5 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ret(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		1 => {
			let lsb = state.registers.take_mem() as u16;
			state.registers.set_hold(lsb);
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		2 => CycleResult::NeedsMore,
		3 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn reti(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		1 => {
			let lsb = state.registers.take_mem() as u16;
			state.registers.set_hold(lsb);
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		2 => CycleResult::NeedsMore,
		3 => {
			state.interrupts.ime = true;
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ret_nz(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => CycleResult::NeedsMore,
		1 => {
			if state.registers.get_zero() {
				state.registers.opcode_bytecount = Some(1);
				CycleResult::Finished
			} else {
				state.cpu_pop_stack();
				CycleResult::NeedsMore
			}
		}
		2 => {
			let lsb = state.registers.take_mem() as u16;
			state.registers.set_hold(lsb);
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		3 => CycleResult::NeedsMore,
		4 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ret_nc(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => CycleResult::NeedsMore,
		1 => {
			if state.registers.get_carry() {
				state.registers.opcode_bytecount = Some(1);
				CycleResult::Finished
			} else {
				state.cpu_pop_stack();
				CycleResult::NeedsMore
			}
		}
		2 => {
			let lsb = state.registers.take_mem() as u16;
			state.registers.set_hold(lsb);
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		3 => CycleResult::NeedsMore,
		4 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ret_z(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => CycleResult::NeedsMore,
		1 => {
			if !state.registers.get_zero() {
				state.registers.opcode_bytecount = Some(1);
				CycleResult::Finished
			} else {
				state.cpu_pop_stack();
				CycleResult::NeedsMore
			}
		}
		2 => {
			let lsb = state.registers.take_mem() as u16;
			state.registers.set_hold(lsb);
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		3 => CycleResult::NeedsMore,
		4 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ret_c(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => CycleResult::NeedsMore,
		1 => {
			if !state.registers.get_carry() {
				state.registers.opcode_bytecount = Some(1);
				CycleResult::Finished
			} else {
				state.cpu_pop_stack();
				CycleResult::NeedsMore
			}
		}
		2 => {
			let lsb = state.registers.take_mem() as u16;
			state.registers.set_hold(lsb);
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
		3 => CycleResult::NeedsMore,
		4 => {
			let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
			state.registers.pc = address;
			state.registers.opcode_bytecount = Some(0);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

macro_rules! define_rst {
	($addr:literal) => {
		paste::paste! {
			pub fn [<rst_ $addr>](state: &mut Gameboy) -> CycleResult {
				match state.registers.cycle {
					0 => CycleResult::NeedsMore,
					1 => {
						state.cpu_push_stack((state.registers.pc >> 8) as u8);
						CycleResult::NeedsMore
					},
					2 => {
						state.cpu_push_stack((state.registers.pc & 0xFF) as u8);
						CycleResult::NeedsMore
					},
					3 => {
						state.registers.pc = $addr;
						state.registers.opcode_bytecount = Some(0);
						CycleResult::Finished
					},
					_ => unreachable!(),
				}
			}
		}
	};
}

define_rst!(0x0);
define_rst!(0x08);
define_rst!(0x10);
define_rst!(0x18);
define_rst!(0x20);
define_rst!(0x28);
define_rst!(0x30);
define_rst!(0x38);
