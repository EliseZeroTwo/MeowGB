use deemgee_opcode::opcode;

use super::CycleResult;
use crate::gameboy::Gameboy;

opcode!(nop, 0x00, "NOP", false, {
	0 => {
		state.registers.opcode_bytecount = Some(1);
		CycleResult::Finished
	}
});

opcode!(di, 0xF3, "DI", false, {
	0 => {
		state.interrupts.cpu_set_ime(false);
		state.registers.opcode_bytecount = Some(1);
		CycleResult::Finished
	}
});

opcode!(ei, 0xFB, "EI", false, {
		0 => {
			state.interrupts.cpu_set_ime(true);
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
});

opcode!(halt, 0x76, "HALT", false, {
	0 => {
		if !state.interrupts.ime && (state.interrupts.interrupt_enable & state.interrupts.interrupt_flag & 0x1F != 0) {
			state.halt_bug = true;
		} else {
			state.halt = true;
		}
		state.registers.opcode_bytecount = Some(1);
		CycleResult::Finished
	}
});