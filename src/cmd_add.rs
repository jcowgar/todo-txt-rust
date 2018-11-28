use gumdrop::Options;
use todo::Todo;
use todo_file;

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
	let t = Todo::new(&task, false, priority);

	let mut todos =
		todo_file::parse_todos_from_default_file().expect("Couldn't parse default todo.txt file");

	todos.push(t);

	todo_file::write_todos_to_default_file(&todos)
		.expect("Couldn't write todos to default todo.txt file");
}
