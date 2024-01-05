mod config;
mod window;

use std::{
	borrow::Cow,
	path::PathBuf,
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc, RwLock,
	},
};

use clap::Parser;
use config::MeowGBConfig;
use meowgb_core::gameboy::{
	bootrom::{verify_parse_bootrom, BootromParseError},
	serial::SerialWriter,
	Gameboy,
};
use window::events::{EmulatorWindowEvent, GameboyEvent};

#[derive(Debug, Parser)]
/// DMG Emulator
pub struct CliArgs {
	/// bootrom path
	#[clap(long)]
	pub bootrom: Option<PathBuf>,
	/// game path
	#[clap(long)]
	pub rom: Option<PathBuf>,
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

	let bootrom = match args.bootrom.as_deref() {
		Some(path) => Some(verify_parse_bootrom(path).map_err(MeowGBError::Bootrom)?),
		None => None,
	};

	let gameboy = Arc::new(RwLock::new(Gameboy::new(bootrom, std::io::stdout())));
	let gameboy_2 = gameboy.clone();

	let jh = std::thread::Builder::new()
		.name(String::from("mewmulator"))
		.spawn(move || run_gameboy(args, gameboy_2, gb_side_rx, gb_side_tx).unwrap())
		.unwrap();

	window::run_window(
		&rom_name.unwrap_or(Cow::Borrowed("NO GAME")),
		config,
		gameboy,
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
	gameboy_arc: Arc<RwLock<Gameboy<impl SerialWriter>>>,
	rx: Receiver<EmulatorWindowEvent>,
	tx: Sender<GameboyEvent>,
) -> Result<(), MeowGBError> {
	let mut gameboy = gameboy_arc.write().unwrap();

	if let Some(rom) = args.rom {
		if !rom.is_file() {
			return Err(MeowGBError::GameNotFound);
		}

		let rom = std::fs::read(rom.as_path())?;

		gameboy.load_cartridge(rom)
	}

	drop(gameboy);

	let mut goal = time::OffsetDateTime::now_utc() + time::Duration::milliseconds(1000 / 60);
	let mut frame_counter = 0;

	'outer: loop {
		let mut gameboy = gameboy_arc.write().unwrap();

		while let Ok(event) = rx.try_recv() {
			match event {
				EmulatorWindowEvent::AToggle => gameboy.joypad.invert_a(),
				EmulatorWindowEvent::BToggle => gameboy.joypad.invert_b(),
				EmulatorWindowEvent::SelectToggle => gameboy.joypad.invert_select(),
				EmulatorWindowEvent::StartToggle => gameboy.joypad.invert_start(),
				EmulatorWindowEvent::UpToggle => gameboy.joypad.invert_up(),
				EmulatorWindowEvent::DownToggle => gameboy.joypad.invert_down(),
				EmulatorWindowEvent::LeftToggle => gameboy.joypad.invert_left(),
				EmulatorWindowEvent::RightToggle => gameboy.joypad.invert_right(),
				EmulatorWindowEvent::PauseToggle => unimplemented!(),
				EmulatorWindowEvent::Exit => break 'outer,
			}
		}

		let redraw_needed = gameboy.tick_4();

		drop(gameboy);

		if redraw_needed {
			let now = time::OffsetDateTime::now_utc();
			frame_counter += 1;
			tx.send(GameboyEvent::Framebuffer(gameboy_arc.read().unwrap().ppu.write_fb())).unwrap();
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
