use std::fs::File;
use std::io::Read;
use std::path::Path;
use once_cell::sync::Lazy;
use async_lock::RwLock;
use serde::Deserialize;


pub static CONF: Lazy<RwLock<Conf>> = Lazy::new(|| {
	RwLock::new(Conf::new())
});


#[derive(Debug, Deserialize)]
pub struct Conf {
	pub vk_auth_url: String,
	pub github_auth_url: String,
}

impl Conf {
	pub fn new() -> Self {
		let path = Path::new("zenux.toml");
		let mut file = File::open(&path).unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		let conf: Conf = toml::from_str(&contents).unwrap();
		return conf;
	}
}
