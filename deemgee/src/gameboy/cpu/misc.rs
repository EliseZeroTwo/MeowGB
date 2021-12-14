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
		state.interrupts.ime = false;
		state.registers.opcode_bytecount = Some(1);
		CycleResult::Finished
	}
});

opcode!(ei, 0xFB, "EI", false, {
		0 => {
			state.interrupts.ime = true;
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
});
