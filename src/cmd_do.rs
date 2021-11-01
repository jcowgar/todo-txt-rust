use crate::cfg::get_auto_archive;
use crate::cfg::get_log_complete_date;
use crate::repeat_time::next_date;
use crate::todo::Todo;
use crate::todo_file::{
	append_todo_to_archive_file, parse_todos_from_default_file, write_todos_to_default_file,
};

use chrono::Local;
use gumdrop::Options;
use uuid::Uuid;

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
	let todo_list =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");
	let todos = &mut todo_list.items;

	let mut new_todos: Vec<Todo> = [].to_vec();
	let mut marked_ids = Vec::new();

	for id in &opts.free {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			t.is_complete = !t.is_complete;
			if t.is_complete {
				if t.has_clock() {
					t.clock_out()
				}

				if get_log_complete_date() {
					t.completed_at = Some(Local::today().naive_local());
				}

				// If this is a repeating task, we always keep the
				// original intact. So duplicate parent task, clear it's
				// temporary state, and mark the duplicate as completed.
				if t.has_repeat() {
					let mut done_t = t.clone();
					done_t.id = Uuid::new_v4();

					if should_archive {
						append_todo_to_archive_file(&done_t).unwrap();
					} else {
						new_todos.push(done_t);
					}

					t.reset(true);

					// compute our new due date
					//
					// unwrap() -> we did check to see we have a
					// repeat pattern above.
					let repeat_pattern = t.key_values.get("rep").unwrap();

					if let Some(v) = next_date(repeat_pattern, None) {
						t.key_values
							.insert("due".to_string(), v.format("%Y-%m-%d").to_string());
					}
				} else if should_archive {
					marked_ids.push(iid - 1);

					append_todo_to_archive_file(t).unwrap();
				}
			} else {
				t.completed_at = None;
			}
		}
	}

	if should_archive {
		marked_ids.sort();

		for index in marked_ids.iter().rev() {
			todos.remove(*index);
		}
	}

	todos.append(&mut new_todos);

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
