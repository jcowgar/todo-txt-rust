use crate::cfg::get_data_filename;
use crate::cfg::get_note_file_extension;
use crate::hms;
use crate::todo_file;
use crate::todo_list::TodoList;

use std::fs::File;
use std::io;
use std::io::Read;

use atty;
use gumdrop::Options;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Only todos that are not yet complete")]
	incomplete: bool,

	#[options(help = "Enable color output (auto, always, never)")]
	color: String,
}

fn read_file(filename: &str) -> io::Result<String> {
	let data_filename = get_data_filename(filename);
	let mut project_docs = String::new();

	let mut f = File::open(data_filename)?;
	f.read_to_string(&mut project_docs)?;

	Ok(project_docs)
}

fn print_todo_list(stream: &mut termcolor::StandardStream, list: TodoList) {
	for t in list.items {
		let mut out = Vec::new();

		match t.priority {
			Some(p) => out.push(format!("({})", p)),
			None => out.push(String::from("   ")),
		}

		out.push(t.task.clone());

		let kv_pairs: Vec<std::string::String> = t
			.key_values
			.iter()
			.map(|(k, v)| format!("{}:{}", k, v))
			.collect();
		let kv_pairs_str = kv_pairs.join(" ");

		if kv_pairs_str.len() > 0 {
			out.push(kv_pairs_str);
		}

		println!("  {:9} {}", t.elapsed_time(), out.join(" "));
	}
}

pub fn execute(opts: &Opts) {
	let project_name = &opts.free[0];
	if opts.free.len() != 1 {
		panic!("no project name given");
	}

	let project_documentation: String =
		match read_file(format!("files/{}.{}", project_name, get_note_file_extension()).as_str()) {
			Err(_) => String::new(),
			Ok(content) => content.split("\n").collect::<Vec<&str>>().join("\n  "),
		};

	let mut todo_list = todo_file::parse_todos_from_default_file()
		.expect("Could not parse default todo.txt file")
		.filter_by_project(&project_name);

	todo_list.sort();

	let time_spent = todo_list
		.items
		.iter()
		.fold(0, |sum, i| sum + i.elapsed_time_as_seconds());

	let (open_todos, closed_todos) = todo_list.split();

	let color_choice = match opts.color.to_ascii_lowercase().as_str() {
		"always" => ColorChoice::Always,
		"never" => ColorChoice::Never,
		"auto" | "" | _ => {
			if atty::is(atty::Stream::Stdout) {
				ColorChoice::Auto
			} else {
				ColorChoice::Never
			}
		}
	};

	let mut stdout = StandardStream::stdout(color_choice);
	let open_task_count = open_todos.items.len();
	let closed_task_count = closed_todos.items.len();
	let total_task_count = open_task_count + closed_task_count;

	println!("# {}\n", project_name);
	println!("  {}", project_documentation);

	println!("# Task Statistics\n");
	println!("  -       Open: {}", open_task_count);
	println!("  -     Closed: {}", closed_task_count);
	println!("  -      Total: {}", total_task_count);
	println!(
		"  - Completion: {:.0}%",
		(closed_task_count as f32 / total_task_count as f32) * 100.0
	);
	println!("  -       Time: {}", hms::from_seconds(time_spent));
	println!("");

	if open_todos.items.len() > 0 {
		println!("# Open Tasks");
		println!("");
		print_todo_list(&mut stdout, open_todos);
		println!("");
	}

	if closed_todos.items.len() > 0 {
		println!("# Closed Tasks");
		println!("");
		print_todo_list(&mut stdout, closed_todos);
		println!("");
	}
}
