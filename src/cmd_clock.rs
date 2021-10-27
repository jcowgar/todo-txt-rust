use crate::todo::Todo;
use crate::todo_file::{parse_todos_from_default_file, write_todos_to_default_file};
use crate::hms;

use chrono::{Local, TimeZone};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(help = "Clear the clock in state of a task")]
	clear: bool,

	#[options(help = "Clear the clocked time on a task")]
	clear_clocked: bool,

	#[options(help = "Set clocked time on a task")]
	set_clocked_time: String,

	#[options(free)]
	free: Vec<String>,
}

fn set_clocked(todos: &mut Vec<Todo>, ids: &Vec<String>, new_clock: &String) {
	for id in ids.iter() {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			t.key_values
				.insert("clocked".to_string(), new_clock.clone());
		}
	}
}

fn clear_clocked(todos: &mut Vec<Todo>, ids: &Vec<String>) {
	for id in ids.iter() {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			t.key_values.remove("clocked");
		}
	}
}

fn clear_clock(todos: &mut Vec<Todo>, ids: &Vec<String>) {
	for id in ids.iter() {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			t.key_values.remove("clock");
		}
	}
}

fn check_into_or_outof(todos: &mut Vec<Todo>, ids: &Vec<String>) {
	let now = Local::now();

	for id in ids.iter() {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			match t.key_values.get("clock") {
				Some(_) => {
					let clocked = match t.key_values.get("clocked") {
						Some(already_clocked) => hms::to_seconds(already_clocked),
						None => 0,
					};
					let clock_secs = match t.key_values.get("clock") {
						Some(v) => v.parse::<i64>().unwrap(),
						None => 0,
					};
					let todo_clock_in = Local.timestamp(clock_secs, 0);
					let current_clocked = now - todo_clock_in;

					t.key_values.remove("clock");
					t.key_values.insert(
						"clocked".to_string(),
						hms::from_seconds(clocked + current_clocked.num_seconds()),
					);
				}
				None => {
					t.key_values
						.insert("clock".to_string(), format!("{}", now.timestamp()));
				}
			}
		}
	}
}

fn display_clocked_todo_items(todos: &mut Vec<Todo>) {
	let now = Local::now();

	for t in todos.iter() {
		if let Some(clock) = t.key_values.get("clock") {
			let seconds = match clock.parse::<i64>() {
				Err(_) => 0,
				Ok(v) => v,
			};
			let todo_clock_in = Local.timestamp(seconds, 0);
			let time_diff = now - todo_clock_in;
			let hms = hms::from_seconds(time_diff.num_seconds());

			println!("{:4}: {:8} {}", t.index + 1, hms, t.task);
		}
	}
}

pub fn execute(opts: &Opts) {
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	if opts.free.len() > 0 {
		if opts.clear {
			clear_clock(todos, &opts.free);
		} else if opts.clear_clocked {
			clear_clocked(todos, &opts.free);
		} else if opts.set_clocked_time.is_empty() == false {
			set_clocked(todos, &opts.free, &opts.set_clocked_time);
		} else {
			check_into_or_outof(todos, &opts.free);
		}

		write_todos_to_default_file(&todos).expect("Could not write todos to default file");
	} else {
		display_clocked_todo_items(todos);
	}
}
