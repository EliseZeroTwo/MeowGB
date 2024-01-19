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
	serial::SerialWriter,
	Gameboy,
};
use window::events::{EmulatorDebugEvent, EmulatorWindowEvent, GameboyEvent};

#[cfg(feature = "debugger")]
#[derive(Debug, Parser)]
/// DMG Emulator
pub struct CliArgs {
	/// game path
	#[clap(long)]
	pub rom: Option<PathBuf>,
	/// start the emulator in debug mode
	#[clap(short, long)]
	pub debug: bool,
}

#[cfg(not(feature = "debugger"))]
#[derive(Debug, Parser)]
/// DMG Emulator
pub struct CliArgs {
	/// game path
	#[clap(long)]
	pub rom: Option<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
pub enum MeowGBError {
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

	let rom = match args.rom.as_deref() {
		Some(rom) => {
			if !rom.is_file() {
				return Err(MeowGBError::GameNotFound);
			}
	
			Some(std::fs::read(rom)?)
		},
		None => None
	};

	let mut gameboy = WrappedGameboy::new(Gameboy::new(std::io::stdout(), rom));
	#[cfg(feature = "debugger")]
	let dbg = args.debug;
	#[cfg(not(feature = "debugger"))]
	let dbg = false;
	gameboy.debugging = dbg;
	let gameboy = Arc::new(RwLock::new(gameboy));
	let gameboy_2 = gameboy.clone();

	let jh = std::thread::Builder::new()
		.name(String::from("mewmulator"))
		.spawn(move || run_gameboy(gameboy_2, gb_side_rx, gb_side_tx).unwrap())
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

pub struct WrappedGameboy<W: SerialWriter> {
	pub breakpoints: [[bool; 3]; 0x10000],
	pub debugging: bool,
	pub gameboy: Gameboy<W>,
}

impl<W: SerialWriter> WrappedGameboy<W> {
	pub fn new(gameboy: Gameboy<W>) -> Self {
		Self { breakpoints: [[false; 3]; 0x10000], debugging: false, gameboy }
	}
}

pub fn run_gameboy(
	gameboy_arc: Arc<RwLock<WrappedGameboy<impl SerialWriter>>>,
	rx: Receiver<EmulatorWindowEvent>,
	tx: Sender<GameboyEvent>,
) -> Result<(), MeowGBError> {
	let mut goal = time::OffsetDateTime::now_utc() + time::Duration::milliseconds(1000 / 60);
	let mut frame_counter = 0;
	let mut debugging_tbf = None;

	'outer: loop {
		let mut step = false;

		let mut gameboy = gameboy_arc.write().unwrap();
		while let Ok(event) = rx.try_recv() {
			match event {
				EmulatorWindowEvent::AToggle => gameboy.gameboy.joypad.invert_a(),
				EmulatorWindowEvent::BToggle => gameboy.gameboy.joypad.invert_b(),
				EmulatorWindowEvent::SelectToggle => gameboy.gameboy.joypad.invert_select(),
				EmulatorWindowEvent::StartToggle => gameboy.gameboy.joypad.invert_start(),
				EmulatorWindowEvent::UpToggle => gameboy.gameboy.joypad.invert_up(),
				EmulatorWindowEvent::DownToggle => gameboy.gameboy.joypad.invert_down(),
				EmulatorWindowEvent::LeftToggle => gameboy.gameboy.joypad.invert_left(),
				EmulatorWindowEvent::RightToggle => gameboy.gameboy.joypad.invert_right(),
				EmulatorWindowEvent::Exit => break 'outer,
				EmulatorWindowEvent::Debug(EmulatorDebugEvent::ToggleBreakpoint(addr, breaks)) => {
					gameboy.breakpoints[addr as usize] = breaks;
				}
				EmulatorWindowEvent::Debug(EmulatorDebugEvent::Continue) => {
					gameboy.debugging = false;
					if let Some(debugging_tbf) = debugging_tbf.take() {
						let delta = time::OffsetDateTime::now_utc() - debugging_tbf;
						goal += delta;
					}
				}
				EmulatorWindowEvent::Debug(EmulatorDebugEvent::Step) => {
					step = true;
				}
			}
		}

		if !gameboy.debugging || step {
			let needs_redraw = gameboy.gameboy.tick_4();
			let bp_triggered = gameboy
				.gameboy
				.last_read
				.map(|(addr, _)| gameboy.breakpoints[addr as usize][0])
				.unwrap_or_default()
				|| gameboy
					.gameboy
					.last_write
					.map(|(addr, _)| gameboy.breakpoints[addr as usize][1])
					.unwrap_or_default()
				|| gameboy.breakpoints[gameboy.gameboy.registers.pc as usize][2];
			gameboy.debugging |= bp_triggered;

			if bp_triggered || step {
				let now = time::OffsetDateTime::now_utc();

				if let Some(debugging_tbf) = debugging_tbf {
					let delta = now - debugging_tbf;
					goal += delta;
				}

				debugging_tbf = Some(now);
			}

			drop(gameboy);

			if needs_redraw {
				let now = time::OffsetDateTime::now_utc();
				frame_counter += 1;
				tx.send(GameboyEvent::Framebuffer(
					gameboy_arc.read().unwrap().gameboy.ppu.write_fb(),
				))
				.unwrap();
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
	}

	Ok(())
}
