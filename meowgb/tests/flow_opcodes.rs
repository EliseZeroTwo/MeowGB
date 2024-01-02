use meowgb::setup_test_emulator;

macro_rules! conditional_jump_relative_testgen {
	($flag:ident, $not_opcode:literal, $opcode:literal) => {
		paste::paste! {
			#[test]
			fn [<test_jr_not_ $flag _i8_unset>]() {
				let mut emulator = setup_test_emulator([$not_opcode, 0x1]);

				emulator.registers.[<set_ $flag>](false);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x103);
			}

			#[test]
			fn [<test_jr_not_ $flag _i8_set>]() {
				let mut emulator = setup_test_emulator([$not_opcode, 0x1]);

				emulator.registers.[<set_ $flag>](true);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x102);
			}

			#[test]
			fn [<test_jr_ $flag _i8_set>]() {
				let mut emulator = setup_test_emulator([$opcode, 0x1]);

				emulator.registers.[<set_ $flag>](true);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x103);
			}

			#[test]
			fn [<test_jr_ $flag _i8_unset>]() {
				let mut emulator = setup_test_emulator([$opcode, 0x1]);

				emulator.registers.[<set_ $flag>](false);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x102);
			}
		}
	};
}

conditional_jump_relative_testgen!(zero, 0x20, 0x28);
conditional_jump_relative_testgen!(carry, 0x30, 0x38);

#[test]
fn test_jr_i8() {
	let mut emulator = setup_test_emulator([0x18, 0x1]);

	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x103);
}

#[test]
fn test_jp_u16() {
	let mut emulator = setup_test_emulator([0xC3, 0xFE, 0xCA]);

	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0xCAFE);
}

#[test]
fn test_jp_hl() {
	let mut emulator = setup_test_emulator([0xE9]);

	emulator.registers.set_hl(0xCAFE);

	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0xCAFE);
}

macro_rules! conditional_jump_testgen {
	($flag:ident, $not_opcode:literal, $opcode:literal) => {
		paste::paste! {
			#[test]
			fn [<test_jp_not_ $flag _u16_unset>]() {
				let mut emulator = setup_test_emulator([$not_opcode, 0xFE, 0xCA]);

				emulator.registers.[<set_ $flag>](false);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0xCAFE);
			}

			#[test]
			fn [<test_jp_not_ $flag _u16_set>]() {
				let mut emulator = setup_test_emulator([$not_opcode, 0xFE, 0xCA]);

				emulator.registers.[<set_ $flag>](true);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x103);
			}

			#[test]
			fn [<test_jp_ $flag _u16_set>]() {
				let mut emulator = setup_test_emulator([$opcode, 0xFE, 0xCA]);

				emulator.registers.[<set_ $flag>](true);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0xCAFE);
			}

			#[test]
			fn [<test_jp_ $flag _u16_unset>]() {
				let mut emulator = setup_test_emulator([$opcode, 0xFE, 0xCA]);

				emulator.registers.[<set_ $flag>](false);

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x103);
			}
		}
	};
}

conditional_jump_testgen!(zero, 0xC2, 0xCA);
conditional_jump_testgen!(carry, 0xD2, 0xDA);

#[test]
fn test_call_u16() {
	let mut emulator = setup_test_emulator([0xCD, 0xFE, 0xCA]);

	let orignal_sp = emulator.registers.sp;

	emulator.tick_4(); // <-- Read first u8
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4(); // <-- Read second u8
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4(); // <-- Push next instruction PC hi to stack
	assert_eq!(emulator.registers.sp, orignal_sp - 1);
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4(); // <-- Push next instruction PC lo to stack
	assert_eq!(emulator.registers.sp, orignal_sp - 2);
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0xCAFE);

	assert_eq!(emulator.debug_read_u8(orignal_sp - 1), 0x01);
	assert_eq!(emulator.debug_read_u8(orignal_sp - 2), 0x03);
}

macro_rules! conditional_call_testgen {
	($flag:ident, $not_opcode:literal, $opcode:literal) => {
		paste::paste! {
			#[test]
			fn [<test_call_not_ $flag _u16_unset>]() {
				let mut emulator = setup_test_emulator([$not_opcode, 0xFE, 0xCA]);

				let orignal_sp = emulator.registers.sp;

				emulator.registers.[<set_ $flag>](false);

				emulator.tick_4(); // <-- Read first u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Read second u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Check flag
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Push next instruction PC hi to stack
				assert_eq!(emulator.registers.sp, orignal_sp - 1);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Push next instruction PC lo to stack
				assert_eq!(emulator.registers.sp, orignal_sp - 2);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0xCAFE);

				assert_eq!(emulator.debug_read_u8(orignal_sp - 1), 0x01);
				assert_eq!(emulator.debug_read_u8(orignal_sp - 2), 0x03);
			}

			#[test]
			fn [<test_call_not_ $flag _u16_set>]() {
				let mut emulator = setup_test_emulator([$not_opcode, 0xFE, 0xCA]);

				let orignal_sp = emulator.registers.sp;

				emulator.registers.[<set_ $flag>](true);

				emulator.tick_4(); // <-- Read first u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Read second u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Check flag
				assert_eq!(emulator.registers.pc, 0x103);

				assert_eq!(emulator.registers.sp, orignal_sp);
			}

			#[test]
			fn [<test_call_ $flag _u16_set>]() {
				let mut emulator = setup_test_emulator([$opcode, 0xFE, 0xCA]);

				let orignal_sp = emulator.registers.sp;

				emulator.registers.[<set_ $flag>](true);

				emulator.tick_4(); // <-- Read first u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Read second u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Check flag
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Push next instruction PC hi to stack
				assert_eq!(emulator.registers.sp, orignal_sp - 1);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Push next instruction PC lo to stack
				assert_eq!(emulator.registers.sp, orignal_sp - 2);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0xCAFE);

				assert_eq!(emulator.debug_read_u8(orignal_sp - 1), 0x01);
				assert_eq!(emulator.debug_read_u8(orignal_sp - 2), 0x03);
			}

			#[test]
			fn [<test_call_ $flag _u16_unset>]() {
				let mut emulator = setup_test_emulator([$opcode, 0xFE, 0xCA]);

				let orignal_sp = emulator.registers.sp;

				emulator.registers.[<set_ $flag>](false);

				emulator.tick_4(); // <-- Read first u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Read second u8
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4(); // <-- Check flag
				assert_eq!(emulator.registers.pc, 0x103);

				assert_eq!(emulator.registers.sp, orignal_sp);
			}
		}
	};
}

conditional_call_testgen!(zero, 0xC4, 0xCC);
conditional_call_testgen!(carry, 0xD4, 0xDC);

#[test]
fn test_ret() {
	let mut emulator = setup_test_emulator([0xC9]);

	emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
	emulator.debug_write_u8(emulator.registers.sp, 0xCA);
	emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
	emulator.debug_write_u8(emulator.registers.sp, 0xFE);

	let orignal_sp = emulator.registers.sp;

	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	assert_eq!(emulator.registers.sp, orignal_sp + 1);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	assert_eq!(emulator.registers.sp, orignal_sp + 2);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0xCAFE);
}

#[test]
fn test_reti() {
	let mut emulator = setup_test_emulator([0xD9]);

	emulator.interrupts.ime = false;

	emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
	emulator.debug_write_u8(emulator.registers.sp, 0xCA);
	emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
	emulator.debug_write_u8(emulator.registers.sp, 0xFE);

	let orignal_sp = emulator.registers.sp;

	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	assert_eq!(emulator.registers.sp, orignal_sp + 1);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	assert_eq!(emulator.registers.sp, orignal_sp + 2);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.pc, 0xCAFE);
	assert_eq!(emulator.interrupts.ime, true);
}

macro_rules! conditional_ret_testgen {
	($flag:ident, $not_opcode:literal, $opcode:literal) => {
		paste::paste! {
			#[test]
			fn [<test_ret_not_ $flag _unset>]() {
				let mut emulator = setup_test_emulator([$not_opcode]);

				emulator.registers.[<set_ $flag>](false);

				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xCA);
				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xFE);

				let orignal_sp = emulator.registers.sp;

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				assert_eq!(emulator.registers.sp, orignal_sp + 1);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				assert_eq!(emulator.registers.sp, orignal_sp + 2);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0xCAFE);
			}

			#[test]
			fn [<test_ret_not_ $flag _set>]() {
				let mut emulator = setup_test_emulator([$not_opcode]);

				emulator.registers.[<set_ $flag>](true);

				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xCA);
				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xFE);

				let orignal_sp = emulator.registers.sp;

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x101);
				assert_eq!(emulator.registers.sp, orignal_sp);
			}

			#[test]
			fn [<test_ret_ $flag _set>]() {
				let mut emulator = setup_test_emulator([$opcode]);

				emulator.registers.[<set_ $flag>](true);

				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xCA);
				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xFE);

				let orignal_sp = emulator.registers.sp;

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				assert_eq!(emulator.registers.sp, orignal_sp + 1);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				assert_eq!(emulator.registers.sp, orignal_sp + 2);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0xCAFE);
			}

			#[test]
			fn [<test_ret_ $flag _unset>]() {
				let mut emulator = setup_test_emulator([$opcode]);

				emulator.registers.[<set_ $flag>](false);

				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xCA);
				emulator.registers.sp = emulator.registers.sp.overflowing_sub(1).0;
				emulator.debug_write_u8(emulator.registers.sp, 0xFE);

				let orignal_sp = emulator.registers.sp;

				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, 0x101);
				assert_eq!(emulator.registers.sp, orignal_sp);
			}
		}
	};
}

conditional_ret_testgen!(zero, 0xC0, 0xC8);
conditional_ret_testgen!(carry, 0xD0, 0xD8);

macro_rules! rst_testgen {
	($opcode:literal, $addr:literal) => {
		paste::paste! {
			#[test]
			fn [<test_rst_ $addr>]() {
				let mut emulator = setup_test_emulator([$opcode]);

				let original_sp = emulator.registers.sp;

				emulator.tick_4();
				assert_eq!(emulator.registers.sp, original_sp);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.sp, original_sp - 1);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.sp, original_sp - 2);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.pc, $addr);

				assert_eq!(emulator.debug_read_u8(original_sp - 1), 0x01);
				assert_eq!(emulator.debug_read_u8(original_sp - 2), 0x01);
			}
		}
	};
}

rst_testgen!(0xC7, 0x0);
rst_testgen!(0xCF, 0x08);
rst_testgen!(0xD7, 0x10);
rst_testgen!(0xDF, 0x18);
rst_testgen!(0xE7, 0x20);
rst_testgen!(0xEF, 0x28);
rst_testgen!(0xF7, 0x30);
rst_testgen!(0xFF, 0x38);
