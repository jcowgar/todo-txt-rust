use crate::todo::Todo;
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

fn print_todo(stream: &mut termcolor::StandardStream, todo: &Todo) {
	let priority_color = match todo.priority {
		Some('A') => Color::Red,
		Some('B') => Color::Cyan,
		Some('C') => Color::Magenta,
		Some(_) => Color::Yellow,
		None => Color::White,
	};

	stream
		.set_color(ColorSpec::new().set_fg(Some(Color::White)))
		.expect("Could not set foreground color");

	print!("  {:3}: ", todo.index + 1);

	print!("[");

	stream
		.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
		.expect("Could not set foreground color");

	print!("{}", if todo.is_complete { "X" } else { " " });

	stream
		.set_color(ColorSpec::new().set_fg(Some(Color::White)))
		.expect("Could not set foreground color");

	print!("] (");

	stream
		.set_color(ColorSpec::new().set_fg(Some(priority_color)))
		.expect("Could not set foreground color");

	print!(
		"{}",
		match todo.priority {
			None => ' ',
			Some(v) => v,
		}
	);

	stream
		.set_color(ColorSpec::new().set_fg(Some(Color::White)))
		.expect("Could not set foreground color");

	print!(") ");

	let words = todo.task.split_whitespace();

	for word in words {
		let color = match word.chars().next() {
			Some('+') => Color::Blue,
			Some('@') => Color::Magenta,
			Some('#') => Color::Cyan,
			_ => Color::White,
		};

		stream
			.set_color(ColorSpec::new().set_fg(Some(color)))
			.expect("Could not set foreground color");
		print!("{} ", word);
	}

	stream
		.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
		.expect("Could not set foreground color");

	println!(" {}", todo.elapsed_time());

	stream
		.set_color(ColorSpec::new().set_fg(Some(Color::White)))
		.expect("Could not set foreground color");
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
		"auto" | "" | _ => {
			if atty::is(atty::Stream::Stdout) {
				ColorChoice::Auto
			} else {
				ColorChoice::Never
			}
		}
	};

	let mut stdout = StandardStream::stdout(color_choice);

	for t in todo_list.items {
		print_todo(&mut stdout, &t);
	}
}
