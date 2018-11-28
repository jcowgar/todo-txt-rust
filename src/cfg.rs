use config::Config;
use std::error::Error;
use std::path::PathBuf;
use std::sync::RwLock;

use dirs;

lazy_static! {
	static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn read_config() -> Result<(), Box<Error>> {
	let mut config_file = dirs::config_dir().unwrap();
	config_file.push("todo-txt");
	config_file.push("todo-txt.ini");

	SETTINGS
		.write()?
		.merge(config::File::with_name(config_file.to_str().unwrap()))?;

	Ok(())
}

fn get_default_data_path() -> PathBuf {
	let mut config_dir = dirs::config_dir().unwrap();
	config_dir.push("todo-txt");

	config_dir
}

fn get_data_path() -> PathBuf {
	match SETTINGS.read() {
		Ok(settings) => {
			let setting = settings.get::<String>("data_path");

			if setting.is_ok() {
				return PathBuf::from(setting.unwrap());
			}

			get_default_data_path()
		},
		_ => get_default_data_path()
	}
}

pub fn get_data_file(filename: &str) -> String {
	let mut pb = get_data_path();
	pb.push(filename);

	pb.to_str().unwrap().to_string()
}
