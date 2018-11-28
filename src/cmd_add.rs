use gumdrop::Options;
use todo::Todo;
use todo_file;

use cfg::get_project_rules;

#[derive(Debug, Options)]
pub struct Opts {
	#[options(free)]
	free: Vec<String>,

	#[options(help = "Priority of the new todo")]
	priority: char,
}

pub fn execute(opts: &Opts) {
	let task = opts.free.join(" ");
	let priority = opts.priority.to_uppercase().next();
	let priority = match priority {
		Some('\0') => None,
		_ => priority
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

	let mut todos =
		todo_file::parse_todos_from_default_file().expect("Couldn't parse default todo.txt file");

	todos.push(t);

	todo_file::write_todos_to_default_file(&todos)
		.expect("Couldn't write todos to default todo.txt file");
}
