pub mod gameboy;
#[allow(unused)]
mod settings;
#[allow(unused)]
mod window;

pub fn setup_test_emulator<const ROM_LENGTH: usize>(
	test_opcodes: [u8; ROM_LENGTH],
) -> gameboy::Gameboy {
	let mut gameboy = gameboy::Gameboy::new(None);

	let mut cartridge = gameboy::mapper::NoMBC { rom: [0u8; 0x8000], ram: None };

	(&mut cartridge.rom[0x100..ROM_LENGTH + 0x100]).copy_from_slice(&test_opcodes);

	gameboy.cartridge = Some(Box::new(cartridge));

	gameboy.tick(); // Prefetch instruction
	assert!(gameboy.registers.mem_read_hold.is_some()); // Assert prefetch happened and opcode is now sitting in the memory bus
	assert_eq!(gameboy.registers.cycle, 0); // Assert tick really did just prefetch instruction and not run the opcode at
										// all

	gameboy
}
