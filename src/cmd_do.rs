use chrono::prelude::*;
use gumdrop::Options;
use todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Do not add a done key/value pair")]
	no_done: bool,
}

pub fn execute(opts: &Opts) {
	let now = Local::now();
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	for id in &opts.free {
		let iid = id.parse::<usize>().unwrap();
		if let Some(t) = todos.get_mut(iid - 1) {
			t.is_complete = !t.is_complete;

			if !opts.no_done {
				if t.is_complete {
					t.key_values.insert(
						"done".to_string(),
						format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day()),
					);
				} else {
					t.key_values.remove(&"done".to_string());
				}
			}
		}
	}

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
