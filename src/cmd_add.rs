use crate::cfg::{get_log_create_date, get_project_rules};
use crate::todo::Todo;
use crate::todo_file::append_todo_to_default_file;
use chrono::Local;
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Priority of the new todo [A-Z]")]
	priority: char,

	#[options(help = "Clock into newly created todo")]
	clock_in: bool,
}

pub fn execute(opts: &Opts) {
	let mut task = opts.free.join(" ");
	let priority = opts.priority.to_uppercase().next();
	match priority {
		None | Some('\0') => {}
		Some(t) => task = format!("({}) {}", t, task),
	};

	let mut t = Todo::parse(&task).unwrap();

	if get_log_create_date() {
		t.created_at = Some(Local::today().naive_local());
	}

	if opts.clock_in {
		t.clock_in();
	}

	for project in &t.projects {
		let project_name = project.replace("+", "");
		let project_rules = get_project_rules(&project_name);

		if let Some(append) = project_rules.get("append") {
			t.task = format!("{} {}", t.task, append)
		}
	}

	append_todo_to_default_file(&t).unwrap();
}
