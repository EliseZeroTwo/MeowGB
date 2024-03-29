use meowgb_opcode::opcode;

use super::CycleResult;
use crate::gameboy::Gameboy;

opcode!(jr_nz_i8, 0x20, "JR NZ,i8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		if state.registers.get_zero() {
			state.registers.take_mem();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	2 => {
		let relative = state.registers.take_mem() as i8;

		if relative >= 0 {
			state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
		} else {
			state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
		}

		CycleResult::Finished
	}
});

opcode!(jr_nc_i8, 0x30, "JR NC,i8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		if state.registers.get_carry() {
			state.registers.take_mem();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	2 => {
		let relative = state.registers.take_mem() as i8;

		if relative >= 0 {
			state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
		} else {
			state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
		}

		CycleResult::Finished
	}
});

opcode!(jr_z_i8, 0x28, "JR Z,i8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		if !state.registers.get_zero() {
			state.registers.take_mem();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	2 => {
		let relative = state.registers.take_mem() as i8;

		if relative >= 0 {
			state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
		} else {
			state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
		}

		CycleResult::Finished
	}
});

opcode!(jr_c_i8, 0x38, "JR C,i8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		if !state.registers.get_carry() {
			state.registers.take_mem();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	2 => {
		let relative = state.registers.take_mem() as i8;

		if relative >= 0 {
			state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
		} else {
			state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
		}

		CycleResult::Finished
	}
});

opcode!(jr_i8, 0x18, "JR i8", false, 2, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		CycleResult::NeedsMore
	},
	2 => {
		let relative = state.registers.take_mem() as i8;

		if relative >= 0 {
			state.registers.pc = state.registers.pc.overflowing_add(relative as u16).0;
		} else {
			state.registers.pc = state.registers.pc.overflowing_sub(relative.abs() as u16).0;
		}

		CycleResult::Finished
	}
});

opcode!(jp_u16, 0xC3, "JP u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::NeedsMore
	},
	3 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(jp_hl, 0xE9, "JP HL", false, 1, {
	0 => {
		state.registers.pc = state.registers.get_hl();
		CycleResult::FinishedKeepPc
	}
});

opcode!(jp_nz_u16, 0xC2, "JP NZ,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if state.registers.get_zero() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(jp_nc_u16, 0xD2, "JP NC,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if state.registers.get_carry() {
			state.registers.take_mem();
			state.registers.take_hold();
			state.registers.opcode_bytecount = Some(3);
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(jp_z_u16, 0xCA, "JP Z,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if !state.registers.get_zero() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(jp_c_u16, 0xDA, "JP C,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if !state.registers.get_carry() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(call_u16, 0xCD, "CALL u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		CycleResult::NeedsMore
	},
	3 => {
		state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
		CycleResult::NeedsMore
	},
	4 => {
		state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
		CycleResult::NeedsMore
	},
	5 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(call_nz_u16, 0xC4, "CALL NZ,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if state.registers.get_zero() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
		CycleResult::NeedsMore
	},
	4 => {
		state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
		CycleResult::NeedsMore
	},
	5 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(call_nc_u16, 0xD4, "CALL NC,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if state.registers.get_carry() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
		CycleResult::NeedsMore
	},
	4 => {
		state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
		CycleResult::NeedsMore
	},
	5 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(call_z_u16, 0xCC, "CALL Z,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if !state.registers.get_zero() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
		CycleResult::NeedsMore
	},
	4 => {
		state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
		CycleResult::NeedsMore
	},
	5 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(call_c_u16, 0xDC, "CALL C,u16", false, 3, {
	0 => {
		state.cpu_read_u8(state.registers.pc.overflowing_add(1).0);
		CycleResult::NeedsMore
	},
	1 => {
		let lower = state.registers.take_mem() as u16;
		state.registers.set_hold(lower);
		state.cpu_read_u8(state.registers.pc.overflowing_add(2).0);
		CycleResult::NeedsMore
	},
	2 => {
		if !state.registers.get_carry() {
			state.registers.take_mem();
			state.registers.take_hold();
			CycleResult::Finished
		} else {
			CycleResult::NeedsMore
		}
	},
	3 => {
		state.cpu_push_stack((state.registers.pc.overflowing_add(3).0 >> 8) as u8);
		CycleResult::NeedsMore
	},
	4 => {
		state.cpu_push_stack(state.registers.pc.overflowing_add(3).0 as u8);
		CycleResult::NeedsMore
	},
	5 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(ret, 0xC9, "RET", false, 1, {
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
		CycleResult::NeedsMore
	},
	3 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(reti, 0xD9, "RETI", false, 1, {
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
		CycleResult::NeedsMore
	},
	3 => {
		state.interrupts.ime = true;
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(ret_nz, 0xC0, "RET NZ", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		if state.registers.get_zero() {
			CycleResult::Finished
		} else {
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
	},
	2 => {
		let lsb = state.registers.take_mem() as u16;
		state.registers.set_hold(lsb);
		state.cpu_pop_stack();
		CycleResult::NeedsMore
	},
	3 => {
		CycleResult::NeedsMore
	},
	4 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(ret_nc, 0xD0, "RET NC", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		if state.registers.get_carry() {
			CycleResult::Finished
		} else {
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
	},
	2 => {
		let lsb = state.registers.take_mem() as u16;
		state.registers.set_hold(lsb);
		state.cpu_pop_stack();
		CycleResult::NeedsMore
	},
	3 => {
		CycleResult::NeedsMore
	},
	4 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(ret_z, 0xC8, "RET Z", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		if !state.registers.get_zero() {
			CycleResult::Finished
		} else {
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
	},
	2 => {
		let lsb = state.registers.take_mem() as u16;
		state.registers.set_hold(lsb);
		state.cpu_pop_stack();
		CycleResult::NeedsMore
	},
	3 => {
		CycleResult::NeedsMore
	},
	4 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

opcode!(ret_c, 0xD8, "RET C", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		if !state.registers.get_carry() {
			CycleResult::Finished
		} else {
			state.cpu_pop_stack();
			CycleResult::NeedsMore
		}
	},
	2 => {
		let lsb = state.registers.take_mem() as u16;
		state.registers.set_hold(lsb);
		state.cpu_pop_stack();
		CycleResult::NeedsMore
	},
	3 => {
		CycleResult::NeedsMore
	},
	4 => {
		let address = (state.registers.take_mem() as u16) << 8 | state.registers.take_hold();
		state.registers.pc = address;
		CycleResult::FinishedKeepPc
	}
});

macro_rules! define_rst {
	($op:literal, $addr:literal) => {
		paste::paste! {
			opcode!([<rst_ $addr>], $op, std::concat!("RST ", std::stringify!($addr)), false, 1, {
				0 => {
					CycleResult::NeedsMore
				},
				1 => {
					state.cpu_push_stack((state.registers.pc.overflowing_add(1).0 >> 8) as u8);
					CycleResult::NeedsMore
				},
				2 => {
					state.cpu_push_stack((state.registers.pc.overflowing_add(1).0 & 0xFF) as u8);
					CycleResult::NeedsMore
				},
				3 => {
					state.registers.pc = $addr;
					CycleResult::FinishedKeepPc
				}
			});
		}
	};
}

define_rst!(0xC7, 0x0);
define_rst!(0xCF, 0x08);
define_rst!(0xD7, 0x10);
define_rst!(0xDF, 0x18);
define_rst!(0xE7, 0x20);
define_rst!(0xEF, 0x28);
define_rst!(0xF7, 0x30);
define_rst!(0xFF, 0x38);
