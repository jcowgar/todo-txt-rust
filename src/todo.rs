use chrono::{Local, NaiveDate, TimeZone};
use std::cmp::Ordering;
use std::collections::HashMap;

use uuid::Uuid;

use regex::Regex;

use crate::hms;

lazy_static! {
	static ref PARSE_RE:      Regex = Regex::new(r"^(?P<complete>x )?(?:\((?P<priority>[A-Z])\))?\s*(?P<date1>\d{4}-\d{2}-\d{2})?\s*(?P<date2>\d{4}-\d{2}-\d{2})?\s*(?P<task>.+$)").unwrap();
	static ref PROJECTS_RE:   Regex = Regex::new(r"\+([^\s]+)").unwrap();
	static ref CONTEXTS_RE:   Regex = Regex::new(r"@([^\s]+)").unwrap();
	static ref KEY_VALUES_RE: Regex = Regex::new(r"(?P<key>[^\s]+):(?P<value>[^\s]+)").unwrap();
}

fn serialize(
	is_complete: bool,
	created_at: Option<NaiveDate>,
	completed_at: Option<NaiveDate>,
	task: &str,
	priority: Option<char>,
) -> String {
	let mut out = Vec::new();

	if is_complete {
		out.push(String::from("x"));
	} else if let Some(p) = priority {
		out.push(format!("({})", p));
	}

	if let Some(d) = completed_at {
		out.push(d.format("%Y-%m-%d").to_string());
	}

	if let Some(d) = created_at {
		out.push(d.format("%Y-%m-%d").to_string());
	}

	out.push(String::from(task));

	out.join(" ")
}

#[derive(Debug)]
/// Todo item
///
/// See <TodoFile>
pub struct Todo {
	/// UUID uniquely identifying task
	pub id: Uuid,

	/// Index of position in the file
	pub index: u32,

	/// true if todo item is done.
	pub is_complete: bool,

	/// Date the task was created
	pub created_at: Option<NaiveDate>,

	/// Date the task was completed
	pub completed_at: Option<NaiveDate>,

	/// Task title
	pub task: String,

	/// Priority (if any), A-Z.
	pub priority: Option<char>,

	/// Project tags (+Project)
	pub projects: Vec<String>,

	/// Context tags (@context)
	pub contexts: Vec<String>,

	/// Key value attributes (key:value)
	pub key_values: HashMap<String, String>,
}

impl Todo {
	/// Create a new Todo structure from the given raw line.
	///
	/// The method may return None, if the line could not be parsed.
	pub fn parse(line: &str) -> Option<Todo> {
		let m = PARSE_RE.captures(line)?;
		let task = match m.name("task") {
			None => return None,
			Some(t) => String::from(t.as_str()),
		};
		let date1 = match m.name("date1") {
			None => None,
			Some(t) => match NaiveDate::parse_from_str(t.as_str(), "%Y-%m-%d") {
				Err(e) => {
					println!("error parsing date1: '{}', {}", t.as_str(), e);
					None
				}
				Ok(t) => Some(t),
			},
		};
		let date2 = match m.name("date2") {
			None => None,
			Some(t) => match NaiveDate::parse_from_str(t.as_str(), "%Y-%m-%d") {
				Err(e) => {
					println!("error parsing date2: '{}' {}", t.as_str(), e);
					None
				}
				Ok(t) => Some(t),
			},
		};

		let projects = PROJECTS_RE
			.captures_iter(&task)
			.map(|cap| String::from(&cap[0]))
			.collect();
		let contexts = CONTEXTS_RE
			.captures_iter(&task)
			.map(|cap| String::from(&cap[0]))
			.collect();
		let mut key_values: HashMap<String, String> = KEY_VALUES_RE
			.captures_iter(&task)
			.map(|cap| (String::from(&cap[1]), String::from(&cap[2])))
			.collect();
		let is_complete = m.name("complete").is_some();
		let mut priority = m
			.name("priority")
			.map(|p| p.as_str().chars().next().unwrap());
		let created_at = match date2 {
			None => date1,
			Some(_) => date2,
		};
		let completed_at = match date2 {
			None => None,
			Some(_) => date1,
		};
		let id = if let Some(v) = key_values.get("id") {
			match Uuid::parse_str(v) {
				Err(_) => Uuid::new_v4(),
				Ok(u) => u,
			}
		} else {
			Uuid::new_v4()
		};

		if priority == None {
			priority = match key_values.get("pri") {
				Some(v) => Some(v.chars().next().unwrap()),
				None => None,
			}
		}

		key_values.remove("id");
		key_values.remove("pri");

		let task = KEY_VALUES_RE.replace_all(&task, "").to_string();
		let task = task.trim().to_string();

		Some(Todo {
			index: 0,
			id,
			created_at,
			completed_at,
			is_complete,
			task,
			priority,
			projects,
			contexts,
			key_values,
		})
	}

	pub fn serialize(&self) -> String {
		let mut serialize_kv_pairs = self.key_values.clone();

		if self.is_complete {
			if let Some(t) = self.priority {
				serialize_kv_pairs.insert("pri".to_string(), format!("{}", t));
			}
		}

		serialize_kv_pairs.insert("id".to_string(), self.id.to_string());

		let kv_pairs: Vec<std::string::String> = serialize_kv_pairs
			.iter()
			.map(|(k, v)| format!("{}:{}", k, v))
			.collect();
		let kv_pairs_str = kv_pairs.join(" ");

		let result = format!(
			"{} {}",
			serialize(
				self.is_complete,
				self.created_at,
				self.completed_at,
				&self.task,
				self.priority
			),
			kv_pairs_str
		);
		let result = result.trim().to_string();

		result
	}

	pub fn is_past_due(&self) -> bool {
		let due_date = match self.key_values.get("due") {
			None => return false,
			Some(v) => v,
		};
		let now = Local::now().format("%Y-%m-%d").to_string();

		due_date <= &now
	}

	pub fn has_repeat(&self) -> bool {
		self.key_values.contains_key("rep")
	}

	pub fn has_clock(&self) -> bool {
		self.key_values.contains_key("clock")
	}

	fn add_to_clocked(&mut self, amount: i64) {
		let already_clocked = match self.key_values.get("clocked") {
			None => 0,
			Some(v) => hms::to_seconds(v),
		};
		let new_clocked = already_clocked + amount;

		self.key_values
			.insert("clocked".to_string(), hms::from_seconds(new_clocked));
	}

	pub fn clock_in(&mut self) {
		if self.has_clock() {
			return;
		}

		let now = Local::now();
		self.key_values
			.insert("clock".to_string(), format!("{}", now.timestamp()));
	}

	pub fn clock_out(&mut self) {
		if self.has_clock() == false {
			return;
		}

		let now = Local::now();
		let current_clock = self
			.key_values
			.get("clock")
			.unwrap()
			.parse::<i64>()
			.unwrap();
		let elapsed = now.timestamp() - current_clock;

		self.add_to_clocked(elapsed);
		self.key_values.remove("clock");
	}

	pub fn elapsed_time(&self) -> String {
		let clocked_time = match self.key_values.get("clocked") {
			None => 0,
			Some(t) => hms::to_seconds(t),
		};

		match self.key_values.get("clock") {
			None => "".to_string(),
			Some(clock) => {
				let now = Local::now();

				let seconds = match clock.parse::<i64>() {
					Err(_) => 0,
					Ok(v) => v,
				};

				let todo_clock_in = Local.timestamp(seconds, 0);
				let time_diff = now - todo_clock_in;

				hms::from_seconds(time_diff.num_seconds() + clocked_time)
			}
		}
	}

	pub fn reset(&mut self, dates: bool) {
		if dates {
			self.created_at = Some(Local::today().naive_local());
		}

		self.is_complete = false;
		self.completed_at = None;

		self.key_values.remove("clock");
		self.key_values.remove("clocked");
	}

	/// Compare two Todo structures by priority and task title
	pub fn cmp(&self, b: &Todo) -> Ordering {
		if self.is_complete == b.is_complete {
			if self.priority.is_none() && b.priority.is_none() {
				self.task.cmp(&b.task)
			} else if self.priority.is_none() {
				Ordering::Greater
			} else if b.priority.is_none() {
				Ordering::Less
			} else {
				let apri = self.priority.unwrap();
				let bpri = b.priority.unwrap();
				let priority_result = apri.cmp(&bpri);

				if priority_result == Ordering::Equal {
					if self.is_complete == b.is_complete {
						self.task.cmp(&b.task)
					} else if self.is_complete {
						Ordering::Greater
					} else {
						Ordering::Less
					}
				} else {
					priority_result
				}
			}
		} else if self.is_complete {
			Ordering::Greater
		} else {
			Ordering::Less
		}
	}

	/// Compare two Todo structures by title alone
	pub fn cmp_by_title(&self, b: &Todo) -> Ordering {
		self.task.cmp(&b.task)
	}

	pub fn cmp_by_due_date(&self, b: &Todo) -> Ordering {
		let duea = self.key_values.get("due");
		let dueb = b.key_values.get("due");

		if self.is_complete && !b.is_complete {
			Ordering::Greater
		} else if !self.is_complete && b.is_complete {
			Ordering::Less
		} else if duea.is_some() && dueb.is_none() {
			Ordering::Less
		} else if duea.is_none() && dueb.is_some() {
			Ordering::Greater
		} else if duea == dueb {
			self.cmp(b)
		} else if duea > dueb {
			Ordering::Greater
		} else {
			Ordering::Less
		}
	}
}

impl Clone for Todo {
	/// Clone a Todo structure
	fn clone(&self) -> Todo {
		Todo {
			index: self.index,
			id: self.id.clone(),
			is_complete: self.is_complete,
			created_at: self.created_at.clone(),
			completed_at: self.completed_at.clone(),
			task: self.task.clone(),
			priority: self.priority,
			projects: self.projects.clone(),
			contexts: self.contexts.clone(),
			key_values: self.key_values.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_simple_todo() {
		let t = Todo::parse("Say hello to mom").unwrap();

		assert_eq!(t.task, "Say hello to mom");
		assert!(!t.is_complete, "should not be completed");
		assert!(t.priority.is_none(), "should not have a priority");
		assert_eq!(t.projects.len(), 0);
		assert_eq!(t.contexts.len(), 0);
	}

	#[test]
	fn parse_completed_todo() {
		let t = Todo::parse("x Say hello to mom").unwrap();

		assert!(t.is_complete, "should be complete");
	}

	#[test]
	fn parse_todo_with_priority() {
		let t = Todo::parse("(A) Say hello to mom").unwrap();

		assert_eq!(t.priority.unwrap(), 'A');
	}

	#[test]
	fn parse_todo_with_projects() {
		let t = Todo::parse("Say hello to mom +Family").unwrap();

		assert_eq!(t.projects.len(), 1);
		assert_eq!(t.projects[0], "+Family");
	}

	#[test]
	fn parse_todo_with_contexts() {
		let t = Todo::parse("Say hello to mom @phone").unwrap();

		assert_eq!(t.contexts.len(), 1);
		assert_eq!(t.contexts[0], "@phone");
	}

	#[test]
	fn parse_todo_with_key_value_pairs() {
		let t = Todo::parse("Say hello to mom due:2018-12-25 time:1am").unwrap();

		assert!(t.key_values.contains_key("due"), "should contain a due key");
		assert_eq!(t.key_values.get("due"), Some(&String::from("2018-12-25")));

		assert!(
			t.key_values.contains_key("time"),
			"should contain a time key"
		);
		assert_eq!(t.key_values.get("time"), Some(&String::from("1am")));
	}

	#[test]
	fn parse_todo_with_create_date() {
		let t = Todo::parse("2021-01-01 happy new year!").unwrap();

		assert_eq!(
			t.created_at.unwrap().format("%Y-%m-%d").to_string(),
			"2021-01-01"
		);
		assert_eq!(t.completed_at, None, "completed_at should be None");
	}

	#[test]
	fn parse_todo_with_create_and_complete_date() {
		let t = Todo::parse("x 2021-01-02 2021-01-01 happy new year!").unwrap();

		assert_eq!(
			t.created_at.unwrap().format("%Y-%m-%d").to_string(),
			"2021-01-01"
		);
		assert_eq!(
			t.completed_at.unwrap().format("%Y-%m-%d").to_string(),
			"2021-01-02"
		);
	}

	fn serialize_test(val: &str) {
		let t = Todo::parse(val).unwrap();

		// Need to remove the automatically added id:xyz before
		// comparing to the original source. Since these are random
		// ids, it's hard to test against with the content being
		// part of the task string.
		let remove_id_re = Regex::new(r"\sid:[^\s]+").unwrap();
		let serialized = t.serialize();
		let serialized_id_removed = remove_id_re.replace_all(&serialized, "");

		assert_eq!(serialized_id_removed, val);
	}

	#[test]
	fn serialize_simple() {
		serialize_test("hello world");
	}

	#[test]
	fn serialize_completed_todo() {
		serialize_test("x hello world");
	}

	#[test]
	fn serialize_todo_with_create_date() {
		serialize_test("2021-01-01 hello world");
	}

	#[test]
	fn serialize_todo_with_priority_and_create_date() {
		serialize_test("(A) 2021-01-01 hello world");
	}

	#[test]
	fn serialize_todo_with_create_and_complete_date() {
		serialize_test("x 2021-01-02 2021-01-01 hello world");
	}
}
