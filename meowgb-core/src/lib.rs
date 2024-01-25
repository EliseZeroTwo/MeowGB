pub mod gameboy;
#[cfg(feature = "instr-dbg")]
pub mod ringbuffer;

/// A helper for writing CPU tests in Rust, the emulator returned by this
/// function has already fetched the first instruction. The next tick will be
/// the first tick of the instruction
pub fn setup_test_emulator<const ROM_LENGTH: usize>(
	test_opcodes: [u8; ROM_LENGTH],
) -> gameboy::Gameboy<std::io::Stdout> {
	let mut cartridge = gameboy::mapper::NoMBC { rom: [0u8; 0x8000], ram: None };

	(&mut cartridge.rom[0x100..ROM_LENGTH + 0x100]).copy_from_slice(&test_opcodes);

	let mut gameboy =
		gameboy::Gameboy::new_with_cartridge(std::io::stdout(), Some(Box::new(cartridge)));

	gameboy.tick_4(); // Prefetch instruction
	assert!(gameboy.registers.mem_read_hold.is_some()); // Assert prefetch happened and opcode is now sitting in the memory bus
	assert_eq!(gameboy.registers.cycle, 0); // Assert tick really did just prefetch instruction and not run the opcode at
										// all

	gameboy
}
