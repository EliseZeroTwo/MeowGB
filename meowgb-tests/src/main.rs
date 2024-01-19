use std::{
	path::{Path, PathBuf},
	sync::{Arc, RwLock},
	time::{Duration, Instant},
};

use clap::{Parser, Subcommand};
use meowgb_core::gameboy::{serial::SerialWriter, Gameboy};

#[derive(Debug, Parser)]
/// DMG Emulator
pub struct CliArgs {
	/// game path
	pub rom: PathBuf,
	#[clap(subcommand)]
	pub operation: Operation,
}

#[derive(Debug, Subcommand)]
pub enum Operation {
	TestFramebuffer {
		/// maximum M-cycles
		#[clap(short = 'm', long)]
		maximum_m_cycles: u64,
		/// path to expected framebuffer (RGBA)
		#[clap(short = 's', long)]
		expected_framebuffer: PathBuf,
	},
	TestSerial {
		/// maximum M-cycles
		#[clap(short = 'm', long)]
		maximum_m_cycles: u64,
		/// path to expected serial output
		#[clap(short = 's', long)]
		expected_serial: PathBuf,
	},
	GenerateOutputSerial {
		/// M-cycles to run for
		#[clap(short = 'm', long)]
		m_cycles: u64,
		/// path to expected serial output
		#[clap(short = 's', long)]
		expected_serial: PathBuf,
	},
	GenerateOutputFramebuffer {
		/// M-cycles to run for
		#[clap(short = 'm', long)]
		m_cycles: u64,
		/// path to expected framebuffer output
		#[clap(short = 's', long)]
		expected_framebuffer: PathBuf,
	},
}

#[derive(Debug, thiserror::Error)]
pub enum DmgTestError {
	#[error("ROM not found")]
	RomNotFound,
	#[error("ROM reading error: {0}")]
	RomRead(std::io::Error),
	#[error("Missing output file")]
	OutputFileNotFound,
	#[error("Error reading output file: {0}")]
	OutputFileRead(std::io::Error),
	#[error("Error writing output file: {0}")]
	OutputFileWrite(std::io::Error),
	#[error("Serial mismatch\nExpected: {0}\nFound: {1}")]
	SerialDifferent(String, String),
	#[error("Framebuffer mismatch")]
	FramebufferDifferent,
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

fn generate_output<const FRAMEBUFFER: bool>(
	rom: &Path,
	m_cycles: u64,
	expected: &Path,
) -> Result<Duration, DmgTestError> {
	let rom = {
		if !rom.is_file() {
			return Err(DmgTestError::RomNotFound);
		}
		std::fs::read(rom).map_err(DmgTestError::RomRead)?
	};

	let sync_writer = SyncWriter::new();
	let mut fb = None;

	let mut gameboy = Gameboy::new(sync_writer.clone(), Some(rom));

	let instant = std::time::Instant::now();

	for _ in 0..m_cycles {
		let new_fb = gameboy.tick_4();

		if FRAMEBUFFER && new_fb {
			fb = Some(gameboy.ppu.write_fb());
		}
	}

	drop(gameboy);

	if FRAMEBUFFER {
		std::fs::write(expected, &fb.unwrap())
			.map_err(DmgTestError::OutputFileWrite)?;
	} else {
		let serial_content = sync_writer.into_inner();
		std::fs::write(expected, &serial_content)
			.map_err(DmgTestError::OutputFileWrite)?;
	}

	Ok(instant.elapsed())
}


fn run_test<const FRAMEBUFFER: bool>(
	rom: &Path,
	maximum_m_cycles: u64,
	expected: &Path,
) -> Result<(u64, Duration), DmgTestError> {
	let rom = {
		if !rom.is_file() {
			return Err(DmgTestError::RomNotFound);
		}
		std::fs::read(rom).map_err(DmgTestError::RomRead)?
	};

	let expected = {
		if !expected.is_file() {
			return Err(DmgTestError::OutputFileNotFound);
		}
		std::fs::read(expected).map_err(DmgTestError::OutputFileRead)?
	};

	if FRAMEBUFFER {
		assert_eq!(expected.len(), (meowgb_core::gameboy::ppu::FB_WIDTH as usize * meowgb_core::gameboy::ppu::FB_HEIGHT as usize) * meowgb_core::gameboy::ppu::PIXEL_SIZE as usize);
	}

	let sync_writer = SyncWriter::new();

	let mut gameboy = Gameboy::new(sync_writer.clone(), Some(rom));

	let instant = Instant::now();

	let mut cycle_counter = 0;

	while cycle_counter < maximum_m_cycles {
		if FRAMEBUFFER {
			let redraw = gameboy.tick_4();

			cycle_counter += 1;

			if redraw && expected == gameboy.ppu.write_fb() {
				return Ok((cycle_counter, instant.elapsed()));
			}
		} else {
			gameboy.tick_4();

			cycle_counter += 1;

			if sync_writer.compare(&expected) {
				return Ok((cycle_counter, instant.elapsed()));
			}
		}
	}

	drop(gameboy);

	match sync_writer.compare(&expected) {
		true => Ok((cycle_counter, instant.elapsed())),
		false => Err(DmgTestError::SerialDifferent(
			expected.into_iter().map(char::from).collect(),
			sync_writer.into_inner().into_iter().map(char::from).collect(),
		)),
	}
}

fn main() {
	let args = CliArgs::parse();

	match args.operation {
		Operation::TestSerial { maximum_m_cycles, expected_serial } => {
			match run_test::<false>(args.rom.as_path(), maximum_m_cycles, expected_serial.as_path()) {
				Ok((m_cycles, duration)) => {
					println!("Success! Ran {} M-Cycles in {}ms", m_cycles, duration.as_millis());
				}
				Err(why) => {
					eprintln!("{}", why);
					std::process::exit(1);
				}
			}
		}
		Operation::GenerateOutputSerial { m_cycles, expected_serial } => {
			match generate_output::<false>(args.rom.as_path(), m_cycles, expected_serial.as_path()) {
				Ok(duration) => {
					println!("Successfully written serial output to {} in {} M-Cycles ({}ms), please verify it is correct", expected_serial.display(), m_cycles, duration.as_millis());
				}
				Err(why) => {
					eprintln!("{}", why);
					std::process::exit(1);
				}
			}
		}
		Operation::TestFramebuffer { maximum_m_cycles, expected_framebuffer } => {
			match run_test::<true>(args.rom.as_path(), maximum_m_cycles, expected_framebuffer.as_path()) {
				Ok((m_cycles, duration)) => {
					println!("Success! Ran {} M-Cycles in {}ms", m_cycles, duration.as_millis());
				}
				Err(why) => {
					eprintln!("{}", why);
					std::process::exit(1);
				}
			}
		},
	    Operation::GenerateOutputFramebuffer { m_cycles, expected_framebuffer } => {
			match generate_output::<true>(args.rom.as_path(), m_cycles, expected_framebuffer.as_path()) {
				Ok(duration) => {
					println!("Successfully written framebuffer output to {} in {} M-Cycles ({}ms), please verify it is correct", expected_framebuffer.display(), m_cycles, duration.as_millis());
				}
				Err(why) => {
					eprintln!("{}", why);
					std::process::exit(1);
				}
			}
		},
	}
}
