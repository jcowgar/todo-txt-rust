use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use dirs;

use todo::Todo;

fn get_data_path() -> PathBuf {
	let mut config_dir = dirs::config_dir().unwrap();
	config_dir.push("todo-txt");

	config_dir
}

fn get_todo_filename() -> String {
	let mut todo_txt_filename = get_data_path();
	todo_txt_filename.push("todo.txt");

	todo_txt_filename.to_str().unwrap().to_string()
}

fn get_archive_filename() -> String {
	let mut archive_filename = get_data_path();
	archive_filename.push("archive.txt");

	archive_filename.to_str().unwrap().to_string()
}

pub fn parse_todos(filename: &str) -> Result<Vec<Todo>, io::Error> {
	let f = try!(File::open(filename));
	let file = BufReader::new(&f);
	let todos = file
		.lines()
		.map(|line| {
			let lineu = line.unwrap();
			Todo::parse(&lineu)
		}).filter(|t| t.is_some())
		.enumerate()
		.map(|(i, t)| {
			let mut result = t.unwrap();
			result.index = i as u32;
			result
		}).collect();

	Ok(todos)
}

pub fn parse_todos_from_default_file() -> Result<Vec<Todo>, io::Error> {
	parse_todos(&get_todo_filename())
}

pub fn write_todos(filename: &str, todos: &Vec<Todo>) -> Result<(), io::Error> {
	let mut f = File::create(filename)?;

	for t in todos {
		f.write(&format!("{}\n", &t.serialize()).into_bytes())?;
	}

	f.flush()?;

	Ok(())
}

pub fn write_todos_to_default_file(todos: &Vec<Todo>) -> Result<(), io::Error> {
	write_todos(&get_todo_filename(), todos)
}

pub fn append_todos_to_archive_file(todos: &Vec<Todo>) -> Result<(), io::Error> {
	let mut f = OpenOptions::new().append(true).create(true).open(&get_archive_filename())?;

	for t in todos {
		f.write(&format!("{}\n", &t.serialize()).into_bytes())?;
	}

	f.flush()?;

	Ok(())
}
