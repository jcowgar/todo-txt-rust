/// Read and Write to todo.txt formatted files
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::Write;
use std::io::BufRead;
use std::io::BufReader;

use crate::cfg::get_data_file;
use crate::todo::Todo;

fn get_todo_filename() -> String {
	get_data_file("todo.txt")
}

fn get_archive_filename() -> String {
	get_data_file("archive.txt")
}

/// Read all todos from `filename`
pub fn parse_todos(filename: &str) -> Result<Vec<Todo>, io::Error> {
	let f = File::open(filename)?;
	let file = BufReader::new(&f);
	let todos = file
		.lines()
		.map(|line| {
			let lineu = line.unwrap();
			Todo::parse(&lineu)
		})
		.filter(|t| t.is_some())
		.enumerate()
		.map(|(i, t)| {
			let mut result = t.unwrap();
			result.index = i as u32;
			result
		})
		.collect();

	Ok(todos)
}

/// Read all todos from the user's default todo.txt file
pub fn parse_todos_from_default_file() -> Result<Vec<Todo>, io::Error> {
	parse_todos(&get_todo_filename())
}

/// Write all `todos` to `filename`
///
/// Warning: This will overwrite `filename`
pub fn write_todos(filename: &str, todos: &[Todo]) -> Result<(), io::Error> {
	let mut f = File::create(filename)?;

	for t in todos {
		f.write_all(&format!("{}\n", &t.serialize()).into_bytes())?;
	}

	f.flush()?;

	Ok(())
}

/// Write all `todos` to the user's default todo.txt file
pub fn write_todos_to_default_file(todos: &[Todo]) -> Result<(), io::Error> {
	write_todos(&get_todo_filename(), todos)
}

fn append_todos_to_file(todos: &[Todo], filename: &str) -> Result<(), io::Error> {
	let mut f = OpenOptions::new()
		.append(true)
		.create(true)
		.open(filename)?;

	for t in todos {
		f.write_all(&format!("{}\n", t.serialize()).into_bytes())?;
	}

	Ok(())
}

/// Append todos to the user's default archive.txt file
pub fn append_todos_to_archive_file(todos: &[Todo]) -> Result<(), io::Error> {
	append_todos_to_file(todos, &get_archive_filename())
}

fn write_todo_to_file(mut f: std::fs::File, todo: &Todo) -> Result<(), io::Error> {
	f.write_all(&format!("{}\n", todo.serialize()).into_bytes())?;

	Ok(())
}

/// Append a single todo to `filename`
pub fn append_todo_to_file(todo: &Todo, filename: &str) -> Result<(), io::Error> {
	let f = OpenOptions::new()
		.append(true)
		.create(true)
		.open(filename)?;

	write_todo_to_file(f, todo)?;

	Ok(())
}

/// Append a single todo to the user's default todo.txt file
pub fn append_todo_to_default_file(todo: &Todo) -> Result<(), io::Error> {
	append_todo_to_file(todo, &get_todo_filename())
}

/// Append a single todo to the user's default archive.txt file
pub fn append_todo_to_archive_file(todo: &Todo) -> Result<(), io::Error> {
	append_todo_to_file(todo, &get_archive_filename())
}
