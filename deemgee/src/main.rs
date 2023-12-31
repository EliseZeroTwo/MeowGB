mod gameboy;
mod settings;
mod window;

use std::{
	borrow::Cow,
	path::PathBuf,
	sync::mpsc::{channel, Receiver, Sender},
};

use chrono::{Duration, Utc};
use clap::Parser;
use gameboy::Gameboy;
use settings::DeemgeeConfig;
use sha1::{Digest, Sha1};
use window::EmulatorWindowEvent;

use crate::window::GameboyEvent;

#[derive(Debug, Parser)]
/// DMG Emulator
pub struct CliArgs {
	/// bootrom path
	#[clap(long)]
	pub bootrom: Option<PathBuf>,
	/// game path
	#[clap(long)]
	pub rom: Option<PathBuf>,
	// enter in debu g mode
	#[clap(short, long)]
	pub debug: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum DmgError {
	#[error("Bootrom Not Found")]
	BootromNotFound,
	#[error("Bootrom Incorrect Size (expected 256 bytes, found {0} bytes)")]
	BootromInvalidSize(u64),
	#[error("Bootrom SHA1 failed (expected 4ed31ec6b0b175bb109c0eb5fd3d193da823339f)")]
	BootromInvalidHash,
	#[error("Game Not Found")]
	GameNotFound,
	#[error("IO Error: {0}")]
	IO(#[from] std::io::Error),
}

fn main() {
	env_logger::init();

	let args: CliArgs = CliArgs::parse();
	let config = DeemgeeConfig::from_file();

	let (window_side_tx, gb_side_rx) = channel::<EmulatorWindowEvent>();
	let (gb_side_tx, window_side_rx) = channel::<GameboyEvent>();

	let rom_name = args.rom.as_ref().and_then(|path| {
		path.file_name().and_then(|name| name.to_str().map(str::to_string).map(Cow::Owned))
	});

	let jh = std::thread::Builder::new()
		.name(String::from("mewmulator"))
		.spawn(move || run_gameboy(config, args, gb_side_rx, gb_side_tx).unwrap())
		.unwrap();

	window::run_window(
		&rom_name.unwrap_or(Cow::Borrowed("NO GAME")),
		config,
		window_side_rx,
		window_side_tx,
	);

	jh.join().unwrap();
}

pub fn run_gameboy(
	_config: DeemgeeConfig,
	args: CliArgs,
	rx: Receiver<EmulatorWindowEvent>,
	tx: Sender<GameboyEvent>,
) -> Result<(), DmgError> {
	let mut bootrom = None;
	if let Some(bootrom_path) = args.bootrom {
		if !bootrom_path.is_file() {
			return Err(DmgError::BootromNotFound);
		}

		let brom_md = std::fs::metadata(bootrom_path.as_path())?;

		if brom_md.len() != 256 {
			return Err(DmgError::BootromInvalidSize(brom_md.len()));
		}

		let mut bootrom_slice = [0u8; 0x100];

		let bootrom_vec = std::fs::read(bootrom_path)?;

		if bootrom_vec.len() != 256 {
			return Err(DmgError::BootromInvalidSize(bootrom_vec.len() as u64));
		}

		let mut hash_ctx = Sha1::new();
		hash_ctx.update(&bootrom_vec);
		let digest = hash_ctx.finalize();

		if digest.as_slice()
			!= b"\x4e\xd3\x1e\xc6\xb0\xb1\x75\xbb\x10\x9c\x0e\xb5\xfd\x3d\x19\x3d\xa8\x23\x33\x9f"
		{
			return Err(DmgError::BootromInvalidHash);
		}

		bootrom_slice.copy_from_slice(&bootrom_vec);

		bootrom = Some(bootrom_slice)
	}

	let mut gameboy = Gameboy::new(bootrom);

	if args.debug {
		gameboy.single_step = true;
		tx.send(GameboyEvent::Framebuffer(gameboy.ppu.write_fb())).unwrap();
	}

	if let Some(rom) = args.rom {
		if !rom.is_file() {
			return Err(DmgError::GameNotFound);
		}

		let rom = std::fs::read(rom.as_path())?;

		gameboy.load_cartridge(rom)
	}

	let mut goal = chrono::Utc::now() + Duration::milliseconds(1000 / 60);
	let mut frame_counter = 0;

	'outer: loop {
		while let Ok(event) = rx.try_recv() {
			match event {
				window::EmulatorWindowEvent::AToggle => gameboy.joypad.set_a(!gameboy.joypad.a),
				window::EmulatorWindowEvent::BToggle => gameboy.joypad.set_b(!gameboy.joypad.b),
				window::EmulatorWindowEvent::SelectToggle => {
					gameboy.joypad.set_select(!gameboy.joypad.select)
				}
				window::EmulatorWindowEvent::StartToggle => {
					gameboy.joypad.set_start(!gameboy.joypad.start)
				}
				window::EmulatorWindowEvent::UpToggle => gameboy.joypad.set_up(!gameboy.joypad.up),
				window::EmulatorWindowEvent::DownToggle => {
					gameboy.joypad.set_down(!gameboy.joypad.down)
				}
				window::EmulatorWindowEvent::LeftToggle => {
					gameboy.joypad.set_left(!gameboy.joypad.left)
				}
				window::EmulatorWindowEvent::RightToggle => {
					gameboy.joypad.set_right(!gameboy.joypad.right)
				}
				window::EmulatorWindowEvent::PauseToggle => {
					gameboy.single_step = !gameboy.single_step
				}
				window::EmulatorWindowEvent::LogToggle => {
					gameboy.log_instructions = !gameboy.log_instructions
				}
				window::EmulatorWindowEvent::Exit => break 'outer,
				window::EmulatorWindowEvent::DumpMemory => {
					let timestamp = Utc::now().timestamp();
					let contents = gameboy.dump_memory();
					std::fs::write(format!("./memdump-{}.bin", timestamp), contents)
						.expect("Failed to write memory dump");
				}
			}
		}

		let (redraw_needed, time_spent_debugging) = gameboy.tick();

		if let Some(diff) = time_spent_debugging {
			goal = goal + Duration::milliseconds(diff);
		}

		if redraw_needed {
			let now = chrono::Utc::now();
			frame_counter += 1;
			tx.send(GameboyEvent::Framebuffer(gameboy.ppu.write_fb())).unwrap();
			let delta = goal - now;
			let delta_ms = delta.num_milliseconds();
			if delta_ms > 0 {
				std::thread::sleep(std::time::Duration::from_millis(delta_ms as u64));
			}
			goal = goal + Duration::milliseconds(1000 / 60);

			if frame_counter == 60 {
				log::debug!("Rendered 60 frames");
				frame_counter = 0;
			}
		}
	}

	Ok(())
}
