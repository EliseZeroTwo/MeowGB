mod gameboy;
mod settings;
mod window;

use std::{
	path::PathBuf,
	sync::mpsc::{channel, Receiver, Sender},
};

use argh::FromArgs;
use gameboy::Gameboy;
use settings::DeemgeeConfig;
use window::WindowEvent;

use crate::window::GameboyEvent;

#[derive(Debug, FromArgs)]
/// DMG Emulator
pub struct CliArgs {
	/// bootrom path
	#[argh(positional)]
	pub bootrom: PathBuf,
	/// game path
	#[argh(positional)]
	pub rom: Option<PathBuf>,
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

fn main() -> Result<(), DmgError> {
	env_logger::init();

	let args: CliArgs = argh::from_env();
	let config = DeemgeeConfig::from_file();

	let (window_side_tx, gb_side_rx) = channel::<WindowEvent>();
	let (gb_side_tx, window_side_rx) = channel::<GameboyEvent>();

	let jh = std::thread::spawn(move || run_gameboy(config, args, gb_side_rx, gb_side_tx));

	window::run_window(config, window_side_rx, window_side_tx);

	jh.join().unwrap()
}

pub fn run_gameboy(
	_config: DeemgeeConfig,
	args: CliArgs,
	rx: Receiver<WindowEvent>,
	tx: Sender<GameboyEvent>,
) -> Result<(), DmgError> {
	if !args.bootrom.is_file() {
		return Err(DmgError::BootromNotFound);
	}

	let brom_md = std::fs::metadata(args.bootrom.as_path())?;

	if brom_md.len() != 256 {
		return Err(DmgError::BootromInvalidSize(brom_md.len()));
	}

	let bootrom = std::fs::read(args.bootrom)?;

	if bootrom.len() != 256 {
		return Err(DmgError::BootromInvalidSize(bootrom.len() as u64));
	}

	if sha1::Sha1::from(bootrom.as_slice()).hexdigest().as_str()
		!= "4ed31ec6b0b175bb109c0eb5fd3d193da823339f"
	{
		return Err(DmgError::BootromInvalidHash);
	}

	let mut gameboy = Gameboy::new(bootrom.as_slice().try_into().unwrap());
	let mut last = chrono::Local::now();
	let mut paused = false;
	let mut frame_counter = 0;

	'outer: loop {
		while let Ok(event) = rx.try_recv() {
			match event {
				window::WindowEvent::AToggle => gameboy.joypad.a = !gameboy.joypad.a,
				window::WindowEvent::BToggle => gameboy.joypad.b = !gameboy.joypad.b,
				window::WindowEvent::SelectToggle => gameboy.joypad.select = !gameboy.joypad.select,
				window::WindowEvent::StartToggle => gameboy.joypad.start = !gameboy.joypad.start,
				window::WindowEvent::UpToggle => gameboy.joypad.up = !gameboy.joypad.up,
				window::WindowEvent::DownToggle => gameboy.joypad.down = !gameboy.joypad.down,
				window::WindowEvent::LeftToggle => gameboy.joypad.left = !gameboy.joypad.left,
				window::WindowEvent::RightToggle => gameboy.joypad.right = !gameboy.joypad.right,
				window::WindowEvent::PauseToggle => paused = !paused,
				window::WindowEvent::Exit => break 'outer,
			}
		}

		if !paused {
			let redraw_needed = gameboy.tick();
			if redraw_needed {
				frame_counter += 1;

				if frame_counter == 60 {
					let now = chrono::Local::now();
					log::info!("Rendered 60 frames in {}", now - last);
					last = now;
					frame_counter = 0;

					tx.send(GameboyEvent::Framebuffer(gameboy.ppu.write_fb())).unwrap();
				}
			}
		}
	}

	Ok(())
}
