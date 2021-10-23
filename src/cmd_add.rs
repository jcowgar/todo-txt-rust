use gumdrop::Options;
use todo::Todo;
use todo_file::append_todo_to_default_file;

use cfg::get_project_rules;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Priority of the new todo [A-Z]")]
	priority: char,
}

pub fn execute(opts: &Opts) {
	let task = opts.free.join(" ");
	let priority = opts.priority.to_uppercase().next();
	let priority = match priority {
		Some('\0') => None,
		_ => priority,
	};
	let mut t = Todo::new(&task, false, priority);

	for project in &t.projects {
		let project_name = project.replace("+", "");
		let project_rules = get_project_rules(&project_name);

		match project_rules.get("append") {
			Some(append) => t.task = format!("{} {}", t.task, append),
			_ => (),
		}
	}

	append_todo_to_default_file(&t).unwrap();
}
