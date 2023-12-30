use std::sync::mpsc::{Receiver, Sender};

use pixels::{Pixels, SurfaceTexture};
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	keyboard::KeyCode,
	window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::settings::DeemgeeConfig;

macro_rules! define_keypress {
	($input:ident, $config:ident, $keymap:ident, $tx:ident, $key:ident, $event:ident) => {
		if $input.key_pressed($config.bindings.$key)
			&& !*$keymap.idx(&$config, $config.bindings.$key)
		{
			$tx.send(EmulatorWindowEvent::$event).unwrap();
			*$keymap.idx(&$config, $config.bindings.$key) = true;
		}

		if $input.key_released($config.bindings.$key)
			&& *$keymap.idx(&$config, $config.bindings.$key)
		{
			$tx.send(EmulatorWindowEvent::$event).unwrap();
			*$keymap.idx(&$config, $config.bindings.$key) = false;
		}
	};
}

#[derive(Debug, Clone, Copy)]
pub enum EmulatorWindowEvent {
	AToggle,
	BToggle,
	SelectToggle,
	StartToggle,
	UpToggle,
	DownToggle,
	LeftToggle,
	RightToggle,
	PauseToggle,
	LogToggle,
	Exit,
	DumpMemory,
}

#[derive(Debug)]
pub enum GameboyEvent {
	Framebuffer(Vec<u8>),
}

pub const FB_HEIGHT: u32 = 144;
pub const FB_WIDTH: u32 = 160;

#[derive(Debug, Default)]
struct Keymap {
	pub down: bool,
	pub up: bool,
	pub left: bool,
	pub right: bool,
	pub start: bool,
	pub select: bool,
	pub b: bool,
	pub a: bool,
	pub pause: bool,
}

impl Keymap {
	pub fn idx(&mut self, config: &DeemgeeConfig, kc: KeyCode) -> &mut bool {
		if kc == config.bindings.a {
			&mut self.a
		} else if kc == config.bindings.b {
			&mut self.b
		} else if kc == config.bindings.start {
			&mut self.start
		} else if kc == config.bindings.select {
			&mut self.select
		} else if kc == config.bindings.up {
			&mut self.up
		} else if kc == config.bindings.down {
			&mut self.down
		} else if kc == config.bindings.left {
			&mut self.left
		} else if kc == config.bindings.right {
			&mut self.right
		} else if kc == config.bindings.pause {
			&mut self.pause
		} else {
			unreachable!();
		}
	}
}

pub fn run_window(
	config: DeemgeeConfig,
	rx: Receiver<GameboyEvent>,
	tx: Sender<EmulatorWindowEvent>,
) {
	let event_loop = EventLoop::new().unwrap();
	let mut input = WinitInputHelper::new();

	let window = { WindowBuilder::new().with_title("OwO").build(&event_loop).unwrap() };

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(FB_WIDTH, FB_HEIGHT, surface_texture).unwrap()
	};

	let mut redraw_happened = true;
	let mut fb: Option<Vec<u8>> = None;

	let mut keymap = Keymap::default();

	let mut request_redraw = false;
	let mut close_requested = false;

	event_loop
		.run(move |event, target| {
			match &event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => close_requested = true,
					WindowEvent::RedrawRequested => {
						let frame = pixels.frame_mut();

						match fb.as_ref() {
							Some(fb) => {
								redraw_happened = true;
								frame.copy_from_slice(fb.as_slice());
							}
							None => {
								let x = vec![0xff; frame.len()];
								frame.copy_from_slice(x.as_slice())
							}
						}
						if let Err(why) = pixels.render() {
							log::error!("Pixels Error: {}", why);
							close_requested = true;
							tx.send(EmulatorWindowEvent::Exit).unwrap();
							return;
						}
					}
					_ => {}
				},
				Event::AboutToWait => {
					if request_redraw && !close_requested {
						window.request_redraw();
					}

					target.set_control_flow(ControlFlow::Poll);

					if close_requested {
						target.exit();
					}
				}
				_ => {}
			}

			if input.update(&event) {
				if input.key_pressed(config.bindings.exit)
					|| (input.close_requested() || input.destroyed())
				{
					tx.send(EmulatorWindowEvent::Exit).unwrap();
					return;
				}

				if input.key_pressed(config.bindings.pause) {
					tx.send(EmulatorWindowEvent::PauseToggle).unwrap();
				}

				if input.key_pressed(config.bindings.log_ops) {
					tx.send(EmulatorWindowEvent::LogToggle).unwrap();
				}

				if input.key_pressed(config.bindings.dump_memory) {
					tx.send(EmulatorWindowEvent::DumpMemory).unwrap();
				}

				define_keypress!(input, config, keymap, tx, a, AToggle);
				define_keypress!(input, config, keymap, tx, b, BToggle);
				define_keypress!(input, config, keymap, tx, start, StartToggle);
				define_keypress!(input, config, keymap, tx, select, SelectToggle);
				define_keypress!(input, config, keymap, tx, up, UpToggle);
				define_keypress!(input, config, keymap, tx, down, DownToggle);
				define_keypress!(input, config, keymap, tx, left, LeftToggle);
				define_keypress!(input, config, keymap, tx, right, RightToggle);
			}

			if let Some(size) = input.window_resized() {
				pixels.resize_surface(size.width, size.height).unwrap();
				window.request_redraw();
				redraw_happened = false;
			}

			while let Ok(event) = rx.try_recv() {
				match event {
					GameboyEvent::Framebuffer(buf) => {
						fb = Some(buf);

						if redraw_happened {
							request_redraw = true;
							redraw_happened = false;
						}
					}
				}
			}
		})
		.expect("event loop error");
}
