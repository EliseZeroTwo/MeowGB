use winit::event::VirtualKeyCode;

use crate::config::MeowGBConfig;

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
	Exit,
}

#[derive(Debug)]
pub enum GameboyEvent {
	Framebuffer(Vec<u8>),
}

#[derive(Debug, Default)]
pub(super) struct Keymap {
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
	pub fn idx(&mut self, config: &MeowGBConfig, kc: VirtualKeyCode) -> &mut bool {
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
