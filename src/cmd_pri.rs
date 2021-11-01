use crate::todo_file::{parse_todos_from_default_file, write_todos_to_default_file};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(help = "Priority [A-Z]")]
	priority: Option<char>,

	#[options(help = "Clear the priority")]
	clear: bool,

	#[options(free)]
	free: Vec<String>,
}

pub fn execute(opts: &Opts) {
	let priority = match opts.priority {
		None => None,
		Some(_) if opts.clear => None,
		Some(p) => p.to_uppercase().next(),
	};
	let todos = &mut parse_todos_from_default_file()
		.expect("Could not parse todos from default file")
		.items;

	for num in opts.free.iter().map(|i| i.parse::<usize>().unwrap() - 1) {
		match todos.get_mut(num) {
			None => println!("todo {} was not found", num + 1),
			Some(t) => t.priority = priority,
		}
	}

	write_todos_to_default_file(todos).expect("Could not write todos to default file");
}
