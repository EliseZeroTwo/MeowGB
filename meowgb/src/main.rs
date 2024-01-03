mod config;
mod window;

use std::{
	borrow::Cow,
	path::PathBuf,
	sync::mpsc::{channel, Receiver, Sender},
};

use clap::Parser;
use meowgb_core::gameboy::{
	bootrom::{verify_parse_bootrom, BootromParseError},
	Gameboy,
};
use config::MeowGBConfig;
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
pub enum MeowGBError {
	#[error(transparent)]
	Bootrom(BootromParseError),
	#[error("Game Not Found")]
	GameNotFound,
	#[error("IO Error: {0}")]
	IO(#[from] std::io::Error),
	#[error(transparent)]
	Config(#[from] config::ConfigError),
}

fn real_main() -> Result<(), MeowGBError> {
	env_logger::init();

	let args: CliArgs = CliArgs::parse();
	let config = MeowGBConfig::from_file()?;

	let (window_side_tx, gb_side_rx) = channel::<EmulatorWindowEvent>();
	let (gb_side_tx, window_side_rx) = channel::<GameboyEvent>();

	let rom_name = args.rom.as_ref().and_then(|path| {
		path.file_name().and_then(|name| name.to_str().map(str::to_string).map(Cow::Owned))
	});

	let jh = std::thread::Builder::new()
		.name(String::from("mewmulator"))
		.spawn(move || run_gameboy(args, gb_side_rx, gb_side_tx).unwrap())
		.unwrap();

	window::run_window(
		&rom_name.unwrap_or(Cow::Borrowed("NO GAME")),
		config,
		window_side_rx,
		window_side_tx,
	);

	jh.join().unwrap();

	Ok(())
}

fn main() {
	if let Err(why) = real_main() {
		eprintln!("{}", why);
		std::process::exit(1);
	}
}

pub fn run_gameboy(
	args: CliArgs,
	rx: Receiver<EmulatorWindowEvent>,
	tx: Sender<GameboyEvent>,
) -> Result<(), MeowGBError> {
	let bootrom = match args.bootrom.as_deref() {
		Some(path) => Some(verify_parse_bootrom(path).map_err(MeowGBError::Bootrom)?),
		None => None,
	};

	let mut gameboy = Gameboy::new(bootrom, std::io::stdout());

	if args.debug {
		gameboy.single_step = true;
		tx.send(GameboyEvent::Framebuffer(gameboy.ppu.write_fb())).unwrap();
	}

	if let Some(rom) = args.rom {
		if !rom.is_file() {
			return Err(MeowGBError::GameNotFound);
		}

		let rom = std::fs::read(rom.as_path())?;

		gameboy.load_cartridge(rom)
	}

	let mut goal = time::OffsetDateTime::now_utc() + time::Duration::milliseconds(1000 / 60);
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
				window::EmulatorWindowEvent::Exit => break 'outer,
			}
		}

		let (redraw_needed, time_spent_debugging) = gameboy.tick_4();

		if let Some(diff) = time_spent_debugging {
			goal = goal + diff;
		}

		if redraw_needed {
			let now = time::OffsetDateTime::now_utc();
			frame_counter += 1;
			tx.send(GameboyEvent::Framebuffer(gameboy.ppu.write_fb())).unwrap();
			let delta = goal - now;
			let delta_ms = delta.whole_milliseconds();
			if delta_ms > 0 {
				std::thread::sleep(std::time::Duration::from_millis(delta_ms as u64));
			}
			goal = goal + time::Duration::milliseconds(1000 / 60);

			if frame_counter == 60 {
				log::debug!("Rendered 60 frames");
				frame_counter = 0;
			}
		}
	}

	Ok(())
}
