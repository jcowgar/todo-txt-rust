use std::cmp::Ordering;
use std::collections::HashMap;

use regex::Regex;

lazy_static! {
		static ref PARSE_RE:      Regex = Regex::new(r"^(?P<complete>x )?(?:\((?P<priority>[A-Z])\))?\s*(?P<date1>\d{4}-\d{2}-\d{2})?\s*(?P<date2>\d{4}-\d{2}-\d{2})?\s*(?P<task>.+$)").unwrap();
		static ref PROJECTS_RE:   Regex = Regex::new(r"\+(\w+)").unwrap();
		static ref CONTEXTS_RE:   Regex = Regex::new(r"@(\w+)").unwrap();
		static ref KEY_VALUES_RE: Regex = Regex::new(r"(?P<key>\w+):(?P<value>[^\s]+)").unwrap();
}

fn serialize(is_complete: bool, task: &str, priority: Option<char>) -> String {
	let mut out = Vec::new();

	if is_complete {
		out.push(String::from("x"));
	}

	if let Some(p) = priority {
		out.push(format!("({})", p));
	}

	out.push(String::from(task));

	out.join(" ").to_string()
}

#[derive(Debug)]
/// Todo item
///
/// See <TodoFile>
pub struct Todo {
	/// Index of position in the file
	pub index: u32,

	/// true if todo item is done.
	pub is_complete: bool,

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
	pub fn new(task: &str, is_complete: bool, priority: Option<char>) -> Todo {
		let serialized = serialize(is_complete, task, priority);
		let result = Todo::parse(&serialized);

		result.unwrap()
	}

	/// Create a new Todo structure from the given raw line.
	///
	/// The method may return None, if the line could not be parsed.
	pub fn parse(line: &str) -> Option<Todo> {
		let m = PARSE_RE.captures(line)?;
		let task = match m.name("task") {
			None => return None,
			Some(t) => String::from(t.as_str()),
		};
		let projects = PROJECTS_RE
			.captures_iter(&task)
			.map(|cap| String::from(&cap[0]))
			.collect();
		let contexts = CONTEXTS_RE
			.captures_iter(&task)
			.map(|cap| String::from(&cap[0]))
			.collect();
		let key_values = KEY_VALUES_RE
			.captures_iter(&task)
			.map(|cap| (String::from(&cap[1]), String::from(&cap[2])))
			.collect();
		let is_complete = m.name("complete").is_some();
		let priority = m
			.name("priority")
			.map(|p| p.as_str().chars().next().unwrap());

		let task = KEY_VALUES_RE.replace_all(&task, "").to_string();
		let task = task.trim().to_string();

		Some(Todo {
			index: 0,
			is_complete,
			task,
			priority,
			projects,
			contexts,
			key_values,
		})
	}

	pub fn serialize(&self) -> String {
		let kv_pairs: Vec<std::string::String> = self
			.key_values
			.iter()
			.map(|(k, v)| format!("{}:{}", k, v))
			.collect();
		let kv_pairs_str = kv_pairs.join(" ");

		let result = format!(
			"{} {}",
			serialize(self.is_complete, &self.task, self.priority),
			kv_pairs_str
		);
		let result = result.trim().to_string();

		result
	}

	/// Compare two Todo structures by priority and task title
	pub fn cmp(&self, b: &Todo) -> Ordering {
		if self.is_complete == b.is_complete {
			if self.priority.is_none() && b.priority.is_none() {
				return self.task.cmp(&b.task);
			} else if self.priority.is_none() {
				return Ordering::Greater;
			} else if b.priority.is_none() {
				return Ordering::Less;
			} else {
				let apri = self.priority.unwrap();
				let bpri = b.priority.unwrap();
				let priority_result = apri.cmp(&bpri);

				if priority_result == Ordering::Equal {
					if self.is_complete == b.is_complete {
						return self.task.cmp(&b.task);
					} else if self.is_complete {
						return Ordering::Greater;
					} else {
						return Ordering::Less;
					}
				} else {
					return priority_result;
				}
			}
		} else if self.is_complete {
			return Ordering::Greater;
		} else {
			return Ordering::Less;
		}
	}

	/// Compare two Todo structures by title alone
	pub fn cmp_by_title(&self, b: &Todo) -> Ordering {
		self.task.cmp(&b.task)
	}
}

impl Clone for Todo {
	/// Clone a Todo structure
	fn clone(&self) -> Todo {
		Todo {
			index: self.index,
			is_complete: self.is_complete,
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
		assert_eq!(t.is_complete, false);
		assert_eq!(t.priority.is_none(), true);
		assert_eq!(t.projects.len(), 0);
		assert_eq!(t.contexts.len(), 0);
	}

	#[test]
	fn parse_completed_todo() {
		let t = Todo::parse("x Say hello to mom").unwrap();

		assert_eq!(t.is_complete, true);
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

		assert_eq!(t.key_values.contains_key("due"), true);
		assert_eq!(t.key_values.get("due"), Some(&String::from("2018-12-25")));

		assert_eq!(t.key_values.contains_key("time"), true);
		assert_eq!(t.key_values.get("time"), Some(&String::from("1am")));
	}
}
