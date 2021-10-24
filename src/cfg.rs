use config::Config;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::RwLock;

use dirs;

lazy_static! {
	static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn read_config(config_filename: Option<&str>) -> Result<(), Box<dyn Error>> {
	let real_filename = match config_filename {
		Some(filename) => filename.to_string(),
		None => {
			let mut filename = dirs::config_dir().unwrap();
			filename.push("todo-txt");
			filename.push("todo-txt.toml");

			String::from(filename.to_str().unwrap())
		}
	};

	SETTINGS
		.write()?
		.merge(config::File::with_name(&real_filename))?;

	Ok(())
}

fn get_default_data_path() -> PathBuf {
	let mut config_dir = dirs::config_dir().unwrap();
	config_dir.push("todo-txt");

	config_dir
}

fn get_bool(name: &str) -> bool {
	match SETTINGS.read() {
		Ok(settings) => match settings.get_bool(name) {
			Ok(value) => value,
			_ => false,
		},
		_ => false,
	}
}

fn get_data_path() -> PathBuf {
	match SETTINGS.read() {
		Ok(settings) => {
			let setting = settings.get::<String>("data_path");

			if setting.is_ok() {
				return PathBuf::from(setting.unwrap());
			}

			get_default_data_path()
		}
		_ => get_default_data_path(),
	}
}

pub fn get_data_file(filename: &str) -> String {
	let mut pb = get_data_path();
	pb.push(filename);

	pb.to_str().unwrap().to_string()
}

pub fn get_mutually_exclusive_tags() -> Vec<Vec<String>> {
	match SETTINGS.read() {
		Ok(settings) => {
			let setting = settings.get::<Vec<Vec<String>>>("mutually_exclusive_tags");

			if setting.is_ok() {
				setting.unwrap()
			} else {
				vec![]
			}
		}
		_ => vec![],
	}
}

pub fn get_project_rules(project_name: &str) -> HashMap<String, String> {
	let key = format!("project_rules.{}", project_name);

	match SETTINGS.read() {
		Ok(settings) => {
			let setting = settings.get::<HashMap<String, String>>(&key);

			match setting {
				Ok(hm) => hm,
				_ => HashMap::new(),
			}
		}
		_ => HashMap::new(),
	}
}

pub fn get_auto_archive() -> bool {
	get_bool("auto_archive")
}

pub fn get_log_create_date() -> bool {
	get_bool("log_create_date")
}

pub fn get_log_complete_date() -> bool {
	get_bool("log_complete_date")
}
