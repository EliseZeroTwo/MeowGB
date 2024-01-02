use meowgb::setup_test_emulator;

#[test]
fn test_nop() {
	let mut emulator = setup_test_emulator([0x00]);

	let expected_register_state = {
		let mut state = emulator.registers.clone();
		state.pc += 1;
		state
	};

	emulator.tick_4();
	assert_eq!(emulator.registers, expected_register_state);
}

#[test]
fn test_di() {
	let mut emulator = setup_test_emulator([0xF3]);

	let expected_register_state = {
		let mut state = emulator.registers.clone();
		state.pc += 1;
		state
	};

	emulator.interrupts.ime = true;

	emulator.tick_4();
	assert_eq!(emulator.registers, expected_register_state);
	assert!(!emulator.interrupts.ime);
}

#[test]
fn test_ei() {
	let mut emulator = setup_test_emulator([0xFB]);

	let expected_register_state = {
		let mut state = emulator.registers.clone();
		state.pc += 1;
		state
	};

	emulator.interrupts.ime = false;

	emulator.tick_4();
	assert_eq!(emulator.registers, expected_register_state);
	assert!(!emulator.interrupts.ime);
	emulator.tick_4(); // <-- Execute the NOP that comes after as the `EI` instruction only takes
				   // effect a cycle after
	assert!(emulator.interrupts.ime);
}

#[test]
fn test_halt() {
	let mut emulator = setup_test_emulator([0x76]);

	let expected_register_state = {
		let mut state = emulator.registers.clone();
		state.pc += 1;
		state
	};

	emulator.interrupts.ime = true;

	emulator.tick_4();
	assert_eq!(emulator.registers, expected_register_state);
	assert!(emulator.halt);
}
