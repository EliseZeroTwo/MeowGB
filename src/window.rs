use std::sync::mpsc::{Receiver, Sender};

use pixels::{Pixels, SurfaceTexture};
use winit::{
	event::{Event, VirtualKeyCode},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::settings::DeemgeeConfig;

macro_rules! define_keypress {
	($input:ident, $config:ident, $keymap:ident, $tx:ident, $key:ident, $event:ident) => {
		if $input.key_pressed($config.bindings.$key)
			&& !*$keymap.idx(&$config, $config.bindings.$key)
		{
			$tx.send(WindowEvent::$event).unwrap();
			*$keymap.idx(&$config, $config.bindings.$key) = true;
		}

		if $input.key_released($config.bindings.$key)
			&& *$keymap.idx(&$config, $config.bindings.$key)
		{
			$tx.send(WindowEvent::$event).unwrap();
			*$keymap.idx(&$config, $config.bindings.$key) = false;
		}
	};
}

#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
	AToggle,
	BToggle,
	SelectToggle,
	StartToggle,
	UpToggle,
	DownToggle,
	LeftToggle,
	RightToggle,
	PauseToggle,
	Exit,
}

#[derive(Debug)]
pub enum GameboyEvent {
	Framebuffer(Vec<u8>),
}

pub const FB_HEIGHT: u32 = 144;
pub const FB_WIDTH: u32 = 160;

#[derive(Debug, Default)]
pub struct Keymap {
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
	pub fn idx(&mut self, config: &DeemgeeConfig, kc: VirtualKeyCode) -> &mut bool {
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

pub fn run_window(config: DeemgeeConfig, rx: Receiver<GameboyEvent>, tx: Sender<WindowEvent>) {
	let event_loop = EventLoop::new();
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

	event_loop.run(move |event, _, control_flow| {
		if let Event::RedrawRequested(_) = event {
			let frame = pixels.get_frame();

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
				*control_flow = ControlFlow::Exit;
				tx.send(WindowEvent::Exit).unwrap();
				return;
			}
		}

		if input.update(&event) {
			if input.key_pressed(config.bindings.exit) || input.quit() {
				*control_flow = ControlFlow::Exit;
				tx.send(WindowEvent::Exit).unwrap();
				return;
			}

			if input.key_pressed(config.bindings.pause) {
				tx.send(WindowEvent::PauseToggle).unwrap();
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
			pixels.resize_surface(size.width, size.height);
			window.request_redraw();
			redraw_happened = false;
		}

		while let Ok(event) = rx.try_recv() {
			match event {
				GameboyEvent::Framebuffer(buf) => {
					fb = Some(buf);

					if redraw_happened {
						window.request_redraw();
						redraw_happened = false;
					}
				}
			}
		}
	});
}
