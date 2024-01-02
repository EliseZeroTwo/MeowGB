use meowgb_core::setup_test_emulator;

macro_rules! ld_reg_imm_u16_testgen {
	($hireg:ident, $loreg:ident, $opcode:literal) => {
		paste::paste! {
			#[test]
			fn [<test_ld_reg_ $hireg $loreg _imm_u16>]() {
				let mut emulator = setup_test_emulator([$opcode, 0xFE, 0xCA]);

				emulator.registers.$hireg = 0x00;
				emulator.registers.$loreg = 0x00;

				emulator.tick_4();
				assert_eq!(emulator.registers.$hireg, 0x00);
				assert_eq!(emulator.registers.$loreg, 0x00);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.$hireg, 0x00);
				assert_eq!(emulator.registers.$loreg, 0xFE);
				assert_eq!(emulator.registers.pc, 0x100);
				emulator.tick_4();
				assert_eq!(emulator.registers.$hireg, 0xCA);
				assert_eq!(emulator.registers.$loreg, 0xFE);
				assert_eq!(emulator.registers.pc, 0x103);
			}
		}
	};
}

ld_reg_imm_u16_testgen!(b, c, 0x01);
ld_reg_imm_u16_testgen!(d, e, 0x11);
ld_reg_imm_u16_testgen!(h, l, 0x21);

#[test]
fn test_ld_reg_sp_imm_u16() {
	let mut emulator = setup_test_emulator([0x31, 0xFE, 0xCA]);

	emulator.registers.sp = 0x0000;

	emulator.tick_4();
	assert_eq!(emulator.registers.sp, 0x0000);
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.sp, 0x00FE);
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.sp, 0xCAFE);
	assert_eq!(emulator.registers.pc, 0x103);
}

#[test]
fn test_ld_sp_hl() {
	let mut emulator = setup_test_emulator([0xF9]);

	emulator.registers.sp = 0x0000;
	emulator.registers.set_hl(0xCAFE);

	emulator.tick_4();
	assert_eq!(emulator.registers.sp, 0x00FE);
	assert_eq!(emulator.registers.pc, 0x100);
	emulator.tick_4();
	assert_eq!(emulator.registers.sp, 0xCAFE);
	assert_eq!(emulator.registers.pc, 0x101);
}
