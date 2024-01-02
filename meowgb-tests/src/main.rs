use std::{path::{PathBuf, Path}, sync::{RwLock, Arc}, time::{Duration, Instant}};

use clap::{Parser, Subcommand};
use meowgb_core::gameboy::{Gameboy, serial::SerialWriter};

#[derive(Debug, Parser)]
/// DMG Emulator
pub struct CliArgs {
	/// game path
	pub rom: PathBuf,
	#[clap(subcommand)]
	pub operation: Operation
}

#[derive(Debug, Subcommand)]
pub enum Operation {
	Test {
		/// maximum M-cycles
		#[clap(short='m', long)]
		maximum_m_cycles: u64,
		/// path to expected serial output
		#[clap(short='s', long)]
		expected_serial: PathBuf,
	},
	GenerateOutput {
		/// M-cycles to run for
		#[clap(short='m', long)]
		m_cycles: u64,
		/// path to expected serial output
		#[clap(short='s', long)]
		expected_serial: PathBuf,
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DmgTestError {
	#[error("ROM not found")]
	RomNotFound,
	#[error("ROM reading error: {0}")]
	RomRead(std::io::Error),
	#[error("Missing serial output file")]
	SerialOutputFileNotFound,
	#[error("Error reading serial output file: {0}")]
	SerialOutputFileRead(std::io::Error),
	#[error("Error writing serial output file: {0}")]
	SerialOutputFileWrite(std::io::Error),
	#[error("Serial mismatch\nExpected: {0}\nFound: {1}")]
	SerialDifferent(String, String),
}

#[derive(Debug, Clone)]
pub struct SyncWriter(pub Arc<RwLock<Vec<u8>>>);

impl SyncWriter {
	pub fn new() -> Self {
		Self(Arc::new(RwLock::new(Vec::new())))
	}

	pub fn compare(&self, expected: &[u8]) -> bool {
		self.0.read().unwrap().as_slice() == expected
	}

	pub fn into_inner(self) -> Vec<u8> {
		std::sync::Arc::into_inner(self.0).unwrap().into_inner().unwrap()
	}
}

impl SerialWriter for SyncWriter {
    fn write_byte(&mut self, byte: u8) {
        self.0.write().unwrap().write_byte(byte);
    }
}

fn generate_output(rom: &Path, m_cycles: u64, expected_serial: &Path) -> Result<Duration, DmgTestError> {
	let rom = {
		if !rom.is_file() {
			return Err(DmgTestError::RomNotFound);
		}
		std::fs::read(rom).map_err(DmgTestError::RomRead)?
	};

	let sync_writer = SyncWriter::new();

	let mut gameboy = Gameboy::new(None, sync_writer.clone());
	gameboy.load_cartridge(rom);

	let instant = std::time::Instant::now();

	for _ in 0..m_cycles {
		gameboy.tick_4();
	}

	drop(gameboy);

	let serial_content = sync_writer.into_inner();
	std::fs::write(expected_serial, &serial_content).map_err(DmgTestError::SerialOutputFileWrite)?;
	

	Ok(instant.elapsed())
}

fn run_test(rom: &Path, maximum_m_cycles: u64, expected_serial: &Path) -> Result<(u64, Duration), DmgTestError> {
	let rom = {
		if !rom.is_file() {
			return Err(DmgTestError::RomNotFound);
		}
		std::fs::read(rom).map_err(DmgTestError::RomRead)?
	};

	let expected_serial = {
		if !expected_serial.is_file() {
			return Err(DmgTestError::SerialOutputFileNotFound);
		}
		std::fs::read(expected_serial).map_err(DmgTestError::SerialOutputFileRead)?
	};

	let sync_writer = SyncWriter::new();

	let mut gameboy = Gameboy::new(None, sync_writer.clone());
	gameboy.load_cartridge(rom);

	let instant = Instant::now();

	let mut cycle_counter = 0;

	while cycle_counter < maximum_m_cycles {
		gameboy.tick_4();

		cycle_counter += 1;

		if sync_writer.compare(&expected_serial) {
			return Ok((cycle_counter, instant.elapsed()));
		}
	}

	drop(gameboy);

	match sync_writer.compare(&expected_serial) {
		true => Ok((cycle_counter, instant.elapsed())),
		false => Err(DmgTestError::SerialDifferent(expected_serial.into_iter().map(char::from).collect(), sync_writer.into_inner().into_iter().map(char::from).collect())),
	}
}

fn main() {
	let args = CliArgs::parse();

	match args.operation {
		Operation::Test { maximum_m_cycles, expected_serial } => match run_test(args.rom.as_path(), maximum_m_cycles, expected_serial.as_path()) {
			Ok((m_cycles, duration)) => {
				println!("Success! Ran {} M-Cycles in {}ms", m_cycles, duration.as_millis());
			},
			Err(why) => {
				eprintln!("{}", why);
				std::process::exit(1);
			}
		},
		Operation::GenerateOutput { m_cycles, expected_serial } => match generate_output(args.rom.as_path(), m_cycles, expected_serial.as_path()) {
			Ok(duration) => {
				println!("Successfully written serial output to {} in {} M-Cycles ({}ms), please verify it is correct", expected_serial.display(), m_cycles, duration.as_millis());
			},
			Err(why) => {
				eprintln!("{}", why);
				std::process::exit(1);
			}
		},
	}

	
}
