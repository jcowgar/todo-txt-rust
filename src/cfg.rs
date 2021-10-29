use config::Config;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::RwLock;

use dirs;

lazy_static! {
	static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
	static ref CONFIG_FILE: Option<PathBuf> = find_config_file();
}

fn find_config_file() -> Option<PathBuf> {
	//
	// Walk the directory tree up looking for a .todo-txt.toml
	//
	let cwd = std::env::current_dir();
	let cwd_pathbuf = match cwd {
		Err(_) => return None,
		Ok(p) => p,
	};

	let mut config_file = cwd_pathbuf;

	loop {
		let cur_dir_file = config_file.join(".todo-txt.toml");

		if cur_dir_file.exists() {
			return Some(cur_dir_file);
		}

		config_file = match config_file.parent() {
			None => break,
			Some(p) => p.to_path_buf(),
		};
	}

	//
	// Look for a XDG_CONFIG_PATH/todo-txt/config.toml file
	//

	let mut filename = dirs::config_dir().unwrap();
	filename.push("todo-txt");
	filename.push("config.toml");

	let std_config_file = filename.to_path_buf();

	if std_config_file.exists() {
		return Some(std_config_file);
	}

	None
}

pub fn read_config(config_filename: Option<&str>) -> Result<(), Box<dyn Error>> {
	let real_filename = match config_filename {
		Some(filename) => filename.to_string(),
		None => {
			let found_filename = &CONFIG_FILE;

			if found_filename.is_none() {
				// We do not *have* to have a configuration file.
				return Ok(())
			}

			let pathbuf = found_filename.as_ref().unwrap();
			String::from(pathbuf.to_str().unwrap())
		}
	};

	SETTINGS
		.write()?
		.merge(config::File::with_name(&real_filename))?;

	Ok(())
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

fn get_char(name: &str) -> Option<char> {
	match SETTINGS.read() {
		Ok(settings) => match settings.get_str(name) {
			Ok(value) => value.chars().next(),
			_ => None,
		},
		_ => None,
	}
}

fn get_default_data_path() -> PathBuf {
	let mut config_dir = dirs::config_dir().unwrap();
	config_dir.push("todo-txt");

	config_dir
}

fn get_data_path() -> PathBuf {
	//
	// Find the data path
	//

	let data_pathbuf = match SETTINGS.read() {
		Ok(settings) => {
			let setting = settings.get::<String>("data_path");

			if setting.is_ok() {
				PathBuf::from(setting.unwrap())
			} else {
				get_default_data_path()
			}
		}
		_ => get_default_data_path(),
	};

	//
	// Create the path if it does not exist
	//

	if !data_pathbuf.exists() {
		let data_path = data_pathbuf.as_path();

		match std::fs::create_dir_all(data_path) {
			Err(_) => (),
			Ok(_) => (),
		};
	}

	//
	// Return the found path
	//

	data_pathbuf
}

fn relative_to_config_file(pb: PathBuf) -> PathBuf {
	if pb.is_relative() {
		match CONFIG_FILE.as_ref() {
			None => pb,
			Some(p) => p.parent().unwrap().join(pb),
		}
	} else {
		pb
	}
}

pub fn get_data_file(filename: &str) -> String {
	let mut pb = relative_to_config_file(get_data_path());
	pb.push(filename);
	pb.to_str().unwrap().to_string()
}

fn get_filename(filename_key: &str, default_filename: &str) -> String {
	let todo_file_pathbuf = match SETTINGS.read() {
		Err(_) => None,
		Ok(settings) => match settings.get::<String>(filename_key) {
			Err(_) => None,
			Ok(p) => Some(PathBuf::from(p)),
		},
	};

	match todo_file_pathbuf {
		None => get_data_file(default_filename),
		Some(p) => relative_to_config_file(p).to_str().unwrap().to_string(),
	}
}

pub fn get_todo_filename() -> String {
	get_filename("todo_filename", "todo.txt")
}

pub fn get_archive_filename() -> String {
	get_filename("archive_filename", "archive.txt")
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

pub fn get_auto_ls() -> bool {
	get_bool("auto_ls")
}

pub fn get_default_priority() -> Option<char> {
	get_char("default_priority")
}
