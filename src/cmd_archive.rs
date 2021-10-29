use crate::todo_file::{
	append_todos_to_archive_file, parse_todos_from_default_file, write_todos_to_default_file,
};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,
}

pub fn execute(_opts: &Opts) {
	let todo_list = parse_todos_from_default_file().expect("Could not parse todos from default file");
	let todos = todo_list.items;
	let mut keep_todos = Vec::new();
	let mut archive_todos = Vec::new();

	for t in todos {
		match t.is_complete {
			false => keep_todos.push(t),
			true => archive_todos.push(t),
		}
	}

	write_todos_to_default_file(&keep_todos).expect("Could not write todos to default file");
	append_todos_to_archive_file(&archive_todos).expect("Could not write todos to default file");
}
