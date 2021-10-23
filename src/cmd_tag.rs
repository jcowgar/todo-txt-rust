use cfg::get_mutually_exclusive_tags;
use gumdrop::Options;
use todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,
}

pub fn execute(opts: &Opts) {
	let mut free_it = opts.free.iter();
	let tag_name = match free_it.next() {
		Some(v) => v,
		None => panic!("No tag name given!"),
	};
	let tag_name_formatted = format!(" #{}", tag_name);
	let mutually_exclusive_tags: Vec<Vec<String>> = get_mutually_exclusive_tags()
		.into_iter()
		.filter(|t| t.contains(&tag_name))
		.collect();
	let todos =
		&mut parse_todos_from_default_file().expect("Could not parse todos from default file");

	for id in free_it {
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
