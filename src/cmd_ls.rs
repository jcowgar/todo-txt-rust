use crate::todo_file;

use atty;
use gumdrop::Options;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Options)]
pub struct Opts {
	#[options(help = "Print help message")]
	help: bool,

	#[options(free)]
	free: Vec<String>,

	#[options(help = "Only todos at or above a given priority [A-Z]")]
	priority: char,

	#[options(help = "Only todos that are not yet complete")]
	incomplete: bool,

	#[options(help = "Only past due todos")]
	past_due: bool,

	#[options(help = "Order by title only")]
	title_order: bool,

	#[options(help = "Order by due date")]
	due_date_order: bool,

	#[options(help = "Limit to only the first N todo items", meta = "N")]
	limit: usize,

	#[options(help = "Enable color output (auto, always, never)")]
	color: String,
}

pub fn default_opts() -> Opts {
	Opts {
		help: false,
		free: [].to_vec(),
		priority: '\0',
		incomplete: false,
		past_due: false,
		title_order: false,
		due_date_order: false,
		limit: 0,
		color: String::from("auto"),
	}
}

pub fn execute(opts: &Opts) {
	let mut todo_list =
		todo_file::parse_todos_from_default_file().expect("Could not parse default todo.txt file");

	if opts.priority >= 'A' {
		let priority_ch = opts.priority.to_uppercase().next().unwrap();

		todo_list = todo_list.filter_by_priority(priority_ch);
	}

	if opts.incomplete {
		todo_list = todo_list.filter_by_complete(false);
	}

	if opts.past_due {
		todo_list = todo_list.filter_by_past_due(true);
	}

	for text in &opts.free {
		todo_list = todo_list.filter_by_text(text);
	}

	if opts.title_order {
		todo_list.sort_by_title();
	} else if opts.due_date_order {
		todo_list.sort_by_due_date();
	} else {
		todo_list.sort();
	}

	if opts.limit > 0 {
		todo_list.items = todo_list.items.into_iter().take(5).collect();
	}

	let color_choice = match opts.color.to_ascii_lowercase().as_str() {
		"always" => ColorChoice::Always,
		"never" => ColorChoice::Never,
		"auto" | "" | _ =>
			if atty::is(atty::Stream::Stdout) {
				ColorChoice::Auto
			} else {
				ColorChoice::Never
			},
	};

	let mut stdout = StandardStream::stdout(color_choice);

	for t in todo_list.items {
		let mut out = Vec::new();

		let color = match t.is_complete {
			true => Color::Green,
			false => match t.priority {
				Some('A') => Color::Red,
				Some('B') => Color::Cyan,
				Some('C') => Color::Magenta,
				Some(_) => Color::Yellow,
				None => Color::White,
			},
		};

		match t.is_complete {
			true => out.push(String::from("[X]")),
			_ => out.push(String::from("[ ]")),
		}

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

		stdout
			.set_color(ColorSpec::new().set_fg(Some(color)))
			.expect("Could not set foreground color");

		println!("{:3}: {}", t.index + 1, out.join(" "));
	}
}
