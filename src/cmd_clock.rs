use crate::hms;
use crate::todo::Todo;
use crate::todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

use chrono::{Local, TimeZone};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(help = "Only show time for task #")]
	only_time: u32,

	#[options(help = "Clear the clock in state of a task")]
	clear: bool,

	#[options(help = "Clear the clocked time on a task")]
	clear_clocked: bool,

	#[options(help = "Clock out of all tasks")]
	out_of_all: bool,

	#[options(help = "Set clocked time on a task")]
	set_time: String,

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
	for id in ids.iter() {
		let iid = id.parse::<usize>().unwrap();

		if let Some(t) = todos.get_mut(iid - 1) {
			if t.has_clock() {
				t.clock_out();
			} else {
				t.clock_in();
			}
		}
	}
}

fn clock_out_of_all(todos: &mut Vec<Todo>){
	for t in todos.iter_mut() {
		if t.has_clock() {
			t.clock_out();
		}
	}
}

fn display_only_time(todos: Vec<Todo>, index: u32) {
	let todo = todos.iter().find(|v| v.index == index - 1);

	let display = match todo {
		None => "".to_string(),
		Some(t) => t.elapsed_time(),
	};

	println!("{}", display);
}

fn display_clocked_todo_items(todos: Vec<Todo>) {
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
	if opts.free.len() > 0 || opts.out_of_all {
		let todos =
			&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

		if opts.clear {
			clear_clock(todos, &opts.free);
		} else if opts.clear_clocked {
			clear_clocked(todos, &opts.free);
		} else if opts.set_time.is_empty() == false {
			set_clocked(todos, &opts.free, &opts.set_time);
		} else if opts.out_of_all {
			clock_out_of_all(todos);
		} else {
			check_into_or_outof(todos, &opts.free);
		}

		write_todos_to_default_file(&todos).expect("Could not write todos to default file");
	} else {
		let todos =
			parse_todos_from_default_file().expect("Could not parse todos from default file");

		if opts.only_time > 0 {
			display_only_time(todos, opts.only_time);
		} else {
			display_clocked_todo_items(todos);
		}
	}
}
