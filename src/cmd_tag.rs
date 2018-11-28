use cfg::get_mutually_exclusive_tags;
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
	let tag_name = &opts.tag;
	let tag_name_formatted = format!(" #{}", opts.tag);
	let mutually_exclusive_tags: Vec<Vec<String>> = get_mutually_exclusive_tags()
		.into_iter()
		.filter(|t| t.contains(&tag_name))
		.collect();
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	for id in &opts.free {
		let iid = id.parse::<usize>().unwrap();
		if let Some(t) = todos.get_mut(iid - 1) {
			for tags in &mutually_exclusive_tags {
				for tag in tags {
					if tag == tag_name {
						continue;
					}

					let tnf = format!(" #{}", tag);

					if t.task.contains(&tnf) {
						t.task = t.task.replace(&tnf, "");
					}
				}
			}

			if t.task.contains(&tag_name_formatted) {
				t.task = t.task.replace(&tag_name_formatted, "");
			} else {
				t.task = format!("{}{}", t.task, tag_name_formatted);
			}
		}
	}

	write_todos_to_default_file(&todos).expect("Could not write todos to default file");
}
