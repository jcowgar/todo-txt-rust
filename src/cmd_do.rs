use chrono::Local;
use gumdrop::Options;

use cfg::get_auto_archive;
use cfg::get_log_complete_date;
use todo_file::{
	append_todo_to_archive_file, parse_todos_from_default_file, write_todos_to_default_file,
};

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Archive todo item once markd done")]
	archive: bool,
}

pub fn execute(opts: &Opts) {
	let should_archive = opts.archive || get_auto_archive();
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	let mut marked_ids = Vec::new();

	for id in &opts.free {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			t.is_complete = !t.is_complete;

			if t.is_complete && get_log_complete_date() {
				t.completed_at = Some(Local::today().naive_local());
			} else {
				t.completed_at = None;
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
