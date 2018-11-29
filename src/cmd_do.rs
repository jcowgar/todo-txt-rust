use chrono::prelude::*;
use gumdrop::Options;

use todo_file::{parse_todos_from_default_file, write_todos_to_default_file, append_todo_to_archive_file};
use cfg::get_auto_archive;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Do not add a done key/value pair")]
	no_done: bool,

	#[options(help = "Archive todo item once markd done")]
	archive: bool,
}

pub fn execute(opts: &Opts) {
	let now = Local::now();
	let should_archive = opts.archive || get_auto_archive();
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	let mut marked_ids = Vec::new();

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

			if t.is_complete && should_archive {
				marked_ids.push(iid - 1);

				append_todo_to_archive_file(t).unwrap();
			}
		}
	}

	if should_archive {
		marked_ids.sort();

		for index in marked_ids.iter().rev() {
			todos.remove(*index);
		}
	}

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
