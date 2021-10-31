use crate::cfg::{get_default_priority, get_log_create_date, get_project_rules};
use crate::repeat_time;
use crate::todo::Todo;
use crate::todo_file::{append_todo_to_default_file, last_inserted_todo_number};
use chrono::{Duration, Local, NaiveDate};
use gumdrop::Options;
use regex::Regex;

lazy_static! {
	static ref YYYYMMDD_RE: Regex = Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$").unwrap();
}


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

	#[options(help = "Only errors are displayed to console")]
	quiet: bool,
}

fn compute_relative_date(value: &str) -> NaiveDate {
	let today = Local::today().naive_local();

	let relative_date = match value.to_ascii_lowercase().as_str() {
		"today" => today,
		"tomorrow" => today + Duration::days(1),
		"monday" => repeat_time::next_weekday(chrono::Weekday::Mon, Some(today)),
		"tuesday" =>repeat_time::next_weekday(chrono::Weekday::Tue, Some(today)),
		"wednesday" => repeat_time::next_weekday(chrono::Weekday::Wed, Some(today)),
		"thursday" => repeat_time::next_weekday(chrono::Weekday::Thu, Some(today)),
		"friday" => repeat_time::next_weekday(chrono::Weekday::Fri, Some(today)),
		"saturday" => repeat_time::next_weekday(chrono::Weekday::Sat, Some(today)),
		"sunday" => repeat_time::next_weekday(chrono::Weekday::Sun, Some(today)),
		_ => {
			// try YYYY-MM-DD
			if YYYYMMDD_RE.is_match(value) {
				return NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap();
			}

			// try repeat_time parsing
			match repeat_time::next_date(value, Some(today)) {
				None => {},
				Some(v) => return v,
			}

			today
		},
	};

	relative_date
}

pub fn execute(opts: &Opts) {
	let mut task = opts.free.join(" ");
	let priority = match opts.priority.to_uppercase().next() {
		None | Some('\0') => get_default_priority(),
		Some(t) => Some(t),
	};
	match priority {
		None | Some('\0') => {}
		Some(t) => task = format!("({}) {}", t, task),
	};

	let mut t = Todo::try_from(task.as_str()).unwrap();

	if get_log_create_date() {
		t.created_at = Some(Local::today().naive_local());
	}

	if opts.clock_in {
		t.clock_in();
	}

	if t.key_values.contains_key("due") {
		let due_str = t.key_values.get("due").unwrap().as_str();
		let due_date = compute_relative_date(due_str);

		t.key_values.insert("due".to_string(), due_date.format("%Y-%m-%d").to_string());
	}

	for project in &t.projects {
		let project_name = project.replace("+", "");
		let project_rules = get_project_rules(&project_name);

		if let Some(append) = project_rules.get("append") {
			t.task = format!("{} {}", t.task, append)
		}
	}

	append_todo_to_default_file(&t).unwrap();

	if !opts.quiet {
		match last_inserted_todo_number() {
			Err(e) => println!("Could not get last inserted todo number: {}", e),
			Ok(count) => println!("{}", count),
		}
	}
}
