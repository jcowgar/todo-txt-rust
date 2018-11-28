use gumdrop::Options;
use todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

#[derive(Debug, Options)]
pub struct Opts {
	#[options(free)]
	free: Vec<String>,

	#[options(help = "Tag name")]
	tag: String,
}

pub fn execute(opts: &Opts) {
	let tag_name = format!(" #{}", opts.tag);
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	for id in &opts.free {
		let iid = id.parse::<usize>().unwrap();
		if let Some(t) = todos.get_mut(iid - 1) {
			if t.task.contains(&tag_name) {
				t.task = t.task.replace(&tag_name, "");
			} else {
				t.task = format!("{}{}", t.task, tag_name);
			}
		}
	}

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
