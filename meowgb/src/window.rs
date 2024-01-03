pub mod events;
mod overlay;

use std::sync::{
	mpsc::{Receiver, Sender},
	Arc, RwLock,
};

use meowgb_core::gameboy::{serial::SerialWriter, Gameboy};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	event::Event,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use self::{
	events::{EmulatorWindowEvent, GameboyEvent, Keymap},
	overlay::Framework,
};
use crate::config::MeowGBConfig;

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

pub fn run_window(
	rom_name: &str,
	config: MeowGBConfig,
	gameboy: Arc<RwLock<Gameboy<impl SerialWriter + 'static>>>,
	rx: Receiver<GameboyEvent>,
	tx: Sender<EmulatorWindowEvent>,
) {
	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();

	let window = {
		WindowBuilder::new().with_title(format!("Meow - {}", rom_name)).build(&event_loop).unwrap()
	};

	let (mut pixels, mut framework) = {
		let window_size = window.inner_size();
		let scale_factor = window.scale_factor() as f32;
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		let pixels = Pixels::new(
			meowgb_core::gameboy::ppu::FB_WIDTH,
			meowgb_core::gameboy::ppu::FB_HEIGHT,
			surface_texture,
		)
		.unwrap();
		let framework = Framework::new(
			&event_loop,
			window_size.width,
			window_size.height,
			scale_factor,
			&pixels,
			&gameboy.read().unwrap(),
		);
		(pixels, framework)
	};

	let mut redraw_happened = true;
	let mut fb: Option<Vec<u8>> = None;

	let mut keymap = Keymap::default();

	event_loop.run(move |event, _, control_flow| {
		if input.update(&event) {
			if input.key_pressed(config.bindings.exit) || input.close_requested() {
				*control_flow = ControlFlow::Exit;
				tx.send(EmulatorWindowEvent::Exit).unwrap();
				return;
			}

			if input.key_pressed(config.bindings.pause) {
				tx.send(EmulatorWindowEvent::PauseToggle).unwrap();
			}

			if let Some(debug_menu) = config.bindings.debug_menu {
				if input.key_pressed(debug_menu) {
					framework.gui.window_open = !framework.gui.window_open;
				}
			}
			if input.key_pressed(config.bindings.pause) {}

			if let Some(scale_factor) = input.scale_factor() {
				framework.scale_factor(scale_factor);
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

		match event {
			Event::WindowEvent { event, .. } => {
				framework.handle_event(&event);
			}
			Event::RedrawRequested(_) => {
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

				framework.prepare(&window, &gameboy.read().unwrap());

				let render_result = pixels.render_with(|encoder, render_target, context| {
					// Render the world texture
					context.scaling_renderer.render(encoder, render_target);

					// Render egui
					framework.render(encoder, render_target, context);

					Ok(())
				});

				if let Err(why) = render_result {
					eprintln!("Pixels Error: {}", why);
					*control_flow = ControlFlow::Exit;
					tx.send(EmulatorWindowEvent::Exit).unwrap();
					return;
				}
			}
			_ => {}
		}

		if let Some(size) = input.window_resized() {
			pixels.resize_surface(size.width, size.height).unwrap();
			framework.resize(size.width, size.height);
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
