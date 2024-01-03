use std::path::Path;

use toml::Value;
use winit::keyboard::KeyCode;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	#[error("Config source not found")]
	SourceNotFound,
	#[error("Error reading source file: {0}")]
	SourceReadError(std::io::Error),
	#[error("TOML parse error: {0}")]
	Toml(toml::de::Error),
}

type Result<T> = std::result::Result<T, ConfigError>;

struct ConfigParser(toml::Table);

impl ConfigParser {
	pub fn new(default: Option<toml::Table>) -> Self {
		Self(default.unwrap_or_default())
	}

	fn parse_toml(path: &Path) -> Result<toml::Table> {
		let file_exists = std::fs::metadata(path).as_ref().map(std::fs::Metadata::is_file).unwrap_or_default();
		
		if !file_exists {
			return Err(ConfigError::SourceNotFound);
		}

		let source_string = std::fs::read_to_string(path).map_err(ConfigError::SourceReadError)?;

		toml::from_str(source_string.as_str()).map_err(ConfigError::Toml)
	}

	/// Merges two values, if both of the values aren't [Value::Table], `new` is returned.
	/// Otherwise contents of `new` are placed ontop of contents of `original`, recursively
	fn merge_toml(original: Value, new: Value) -> Value {
		match (original, new) {
			(Value::Table(mut original), Value::Table(new)) => {
				for (k, v) in new {
					let value = match original.remove(&k) {
						Some(original_subvalue) => Self::merge_toml(original_subvalue, v),
						None => v,
					};
					original.insert(k, value);
				}
				Value::Table(original)
			},
			(_, new) => new
		} 
	}

	/// If file at `path` exists, it merges with the existing config overriding any duplicate keys
	pub fn merge_exists(mut self, path: &Path) -> Result<Self> {
		let toml = match Self::parse_toml(path) {
			Ok(toml) => toml,
			Err(ConfigError::SourceNotFound) => return Ok(self),
			Err(why) => return Err(why)
		};

		self.0 = match Self::merge_toml(Value::Table(self.0), Value::Table(toml)) {
			Value::Table(table) => table,
			_ => unreachable!(),
		};

		Ok(self)
	}

	pub fn build<T: serde::de::DeserializeOwned>(self) -> Result<T> {
		toml::from_str(dbg!(self.0.to_string()).as_str()).map_err(ConfigError::Toml)
	}
}

#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct MeowGBConfig {
	pub bindings: Bindings,
}

impl MeowGBConfig {
	pub fn from_file() -> Result<Self> {
		let mut builder = ConfigParser::new(Some(toml::toml! {
			[bindings]
			a = "KeyA"
			b = "KeyS"
			select = "KeyQ"
			start = "KeyW"
			up = "ArrowUp"
			down = "ArrowDown"
			left = "ArrowLeft"
			right = "ArrowRight"
			pause = "KeyP"
			exit = "Escape"
			log_ops = "KeyL"
			dump_memory = "Comma"
		}));

		if let Some(path) = home::home_dir().and_then(|mut path| { path.push(".MeowGB/config.toml"); path.to_str().map(String::from) }) {
			builder = builder.merge_exists(path.as_ref())?;
		}

		builder.merge_exists(Path::new("config.toml"))?.build()
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
}
