use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref PARSE_RE:      Regex = Regex::new(r"^(?P<complete>x )?(?:\((?P<priority>[A-Z])\))?\s*(?P<date1>\d{4}-\d{2}-\d{2})?\s*(?P<date2>\d{4}-\d{2}-\d{2})?\s*(?P<task>.+$)").unwrap();
    static ref PROJECTS_RE:   Regex = Regex::new(r"+(\w+)").unwrap();
    static ref CONTEXTS_RE:   Regex = Regex::new(r"@(\w+)").unwrap();
    static ref KEY_VALUES_RE: Regex = Regex::new(r"(?P<key>\w+):(?P<value>\w+)").unwrap();
}

#[derive(Debug)]
pub struct Todo {
	/// true if todo item is done.
  is_complete: bool,

  /// Task title
  task       : String,
  
  /// Priority (if any), A-Z.
  priority   : Option<char>,

  /// Project tags (+Project)
  projects   : Vec<String>,

  /// Context tags (@context)
  contexts   : Vec<String>,

  /// Key value attributes (key:value)
  key_values : HashMap<String, String>,
}

impl Todo {
  /// Create a new Todo structure from the given raw line.
  ///
  /// The method may return None, if the line could not be parsed.
  pub fn parse(line: &str) -> Option<Todo> {
    let m = PARSE_RE.captures(line)?;
      
  	let task = match m.name("task") {
  		None    => return None,
  		Some(t) => String::from(t.as_str()),
  	};

    let projects    = Vec::new();
    let contexts    = Vec::new();
    let key_values  = HashMap::new();
    let is_complete = m.name("complete").is_some();
    let priority    = m.name("priority").map(|p| p.as_str().chars().next().unwrap());

  	Some(Todo {
  	    is_complete,
  	    task,
  	    priority,
  	    projects,
  	    contexts,
  	    key_values,
  	})
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
}

