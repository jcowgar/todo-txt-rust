use std::fs::File;
use std::io;
use std::io::prelude::Write;
use std::io::BufRead;
use std::io::BufReader;

use todo::Todo;

#[cfg(target_family = "unix")]
const TODO_FILE: &str = "/Users/jeremy/.todo-txt/todo.txt";

#[cfg(target_family = "windows")]
const TODO_FILE: &str = "\\Users\\jerem\\.todo-txt\\todo.txt";

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
	parse_todos(TODO_FILE)
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
	write_todos(TODO_FILE, todos)
}
