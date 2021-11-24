use winit::event::VirtualKeyCode;

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct DeemgeeConfig {
	pub bindings: Bindings,
}

impl DeemgeeConfig {
	pub fn from_file() -> Self {
		let mut settings = config::Config::default();
		settings.merge(config::File::with_name("config")).unwrap();
		settings.try_into().expect("Config Error")
	}
}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct Bindings {
	pub a: VirtualKeyCode,
	pub b: VirtualKeyCode,
	pub select: VirtualKeyCode,
	pub start: VirtualKeyCode,
	pub up: VirtualKeyCode,
	pub down: VirtualKeyCode,
	pub left: VirtualKeyCode,
	pub right: VirtualKeyCode,

	pub pause: VirtualKeyCode,
	pub exit: VirtualKeyCode,
}
