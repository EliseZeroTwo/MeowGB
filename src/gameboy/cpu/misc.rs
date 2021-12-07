use super::CycleResult;
use crate::gameboy::Gameboy;

pub fn nop(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn di(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.interrupts.ime = false;
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}

pub fn ei(state: &mut Gameboy) -> CycleResult {
	match state.registers.cycle {
		0 => {
			state.interrupts.ime = true;
			state.registers.opcode_bytecount = Some(1);
			CycleResult::Finished
		}
		_ => unreachable!(),
	}
}
