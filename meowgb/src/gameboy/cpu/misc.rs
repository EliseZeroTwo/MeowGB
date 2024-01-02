use meowgb_opcode::opcode;

use super::CycleResult;
use crate::gameboy::Gameboy;

opcode!(nop, 0x00, "NOP", false, 1, {
	0 => {
		CycleResult::Finished
	}
});

opcode!(di, 0xF3, "DI", false, 1, {
	0 => {
		state.interrupts.cpu_set_ime(false);
		CycleResult::Finished
	}
});

opcode!(ei, 0xFB, "EI", false, 1, {
		0 => {
			state.interrupts.cpu_set_ime(true);
			CycleResult::Finished
		}
});

opcode!(halt, 0x76, "HALT", false, 1, {
	0 => {
		if !state.interrupts.ime && (state.interrupts.interrupt_enable & state.interrupts.interrupt_flag & 0x1F != 0) {
			state.halt_bug = true;
		} else {
			state.halt = true;
		}
		CycleResult::Finished
	}
});

opcode!(stop, 0x10, "STOP", false, 1, {
	0 => {
		CycleResult::NeedsMore
	},
	1 => {
		let button_held = state.joypad.cpu_read() & 0b1111 != 0;
		let interrupt_pending = state.interrupts.interrupt_enable & state.interrupts.interrupt_flag != 0;

		match button_held {
			true => match interrupt_pending {
				true => {
					state.registers.pc = state.registers.pc.wrapping_add(1);
				},
				false => {
					state.registers.pc = state.registers.pc.wrapping_add(2);
					state.halt = true;
				}
			},
			false => match interrupt_pending {
				true => {
					state.registers.pc = state.registers.pc.wrapping_add(1);
					state.stop = true;
					state.timer.div = 0;
				},
				false => {
					state.registers.pc = state.registers.pc.wrapping_add(2);
					state.stop = true;
					state.timer.div = 0;
				}
			},
		}

		CycleResult::FinishedKeepPc
	}
});
