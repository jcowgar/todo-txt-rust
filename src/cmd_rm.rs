use crate::todo_file::{parse_todos_from_default_file, write_todos_to_default_file};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,
}

pub fn execute(opts: &Opts) {
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	let mut indexes_to_remove = Vec::new();

	for id in &opts.free {
		let iid = id.parse::<usize>().unwrap();
		if let Some(t) = todos.get_mut(iid - 1) {
			indexes_to_remove.push(t.index);
		}
	}

	indexes_to_remove.sort();

	for index in indexes_to_remove.iter().rev() {
		todos.remove(*index as usize);
	}

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
