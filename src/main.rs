#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Todo {
    is_complete: bool,
    task       : String,
    priority   : Option<char>,
    projects   : Vec<String>,
    contexts   : Vec<String>,
    key_values : HashMap<String, String>,
}

lazy_static! {
    static ref PARSE_RE:      Regex = Regex::new(r"^(?P<complete>x )?(?:\((?P<priority>[A-Z])\))?\s*(?P<date1>\d{4}-\d{2}-\d{2})?\s*(?P<date2>\d{4}-\d{2}-\d{2})?\s*(?P<task>.+$)").unwrap();
    static ref PROJECTS_RE:   Regex = Regex::new(r"+(\w+)").unwrap();
    static ref CONTEXTS_RE:   Regex = Regex::new(r"@(\w+)").unwrap();
    static ref KEY_VALUES_RE: Regex = Regex::new(r"(?P<key>\w+):(?P<value>\w+)").unwrap();
}

fn parse(line: &str) -> Option<Todo> {
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

fn main() {
    let examples = [
        "(A) Thank Mom for the meatballs @phone",
        "x (B) Get tires on the van @maintenance",
        "Learn Rust",
    ];

    for example in examples.iter() {
        let t = parse(example);

        match t {
	        None => println!("Could not parse: {}", example),
	        Some(task) => println!("{:?}", task),
        };
    }
}

