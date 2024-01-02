use winit::keyboard::KeyCode;

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct DeemgeeConfig {
	pub bindings: Bindings,
}

impl DeemgeeConfig {
	pub fn from_file() -> Self {
		config::Config::builder()
			.add_source(config::File::with_name("config"))
			.build()
			.and_then(config::Config::try_deserialize)
			.expect("config")
	}
}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct Bindings {
	pub a: KeyCode,
	pub b: KeyCode,
	pub select: KeyCode,
	pub start: KeyCode,
	pub up: KeyCode,
	pub down: KeyCode,
	pub left: KeyCode,
	pub right: KeyCode,

	pub pause: KeyCode,
	pub exit: KeyCode,
	pub log_ops: KeyCode,
	pub dump_memory: KeyCode,
}
