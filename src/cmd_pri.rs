use crate::todo_file::{parse_todos_from_default_file, write_todos_to_default_file};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(help = "Priority [A-Z]")]
	priority: char,

	#[options(help = "Clear the priority")]
	clear: bool,

	#[options(free)]
	free: Vec<String>,
}

pub fn execute(opts: &Opts) {
	let free_it = opts.free.iter();
	let clear = opts.clear;
	let priority = if clear {
		None
	} else {
		opts.priority.to_uppercase().next()
	};

	if clear == false && priority == None {
		println!("No priority given, nor command to clear priority.");
		return;
	}

	let todo_list =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");
	let todos = &mut todo_list.items;

	for id in free_it {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			t.priority = priority;
		}
	}

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
