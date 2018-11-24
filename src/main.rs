#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use regex::Regex;

struct Todo {
    raw: String,
    is_complete: bool,
    task: String,
    priority: Option<char>,
    projects: Vec<String>,
    contexts: Vec<String>,
    key_values: HashMap<String, String>,
}

lazy_static! {
    static ref PARSE_RE: Regex = Regex::new(r"^(?P<complete>x )?(?:\((?P<priority>[A-Z])\))?\s*(?P<date1>\d{4}-\d{2}-\d{2})?\s*(?P<date2>\d{4}-\d{2}-\d{2})?\s*(?P<task>.*$)").unwrap();
}

fn parse(line: &str) -> Option<Todo> {
    let matches = PARSE_RE.captures(line);

    if matches.is_some() {
        let m = matches.unwrap();

        let is_complete = m.name("complete").is_some();
        let priority_match = m.name("priority");
        let task_match = m.name("task");

        let priority = if priority_match.is_some() {
            let p_str = priority_match.unwrap().as_str();
            let mut p_chars = p_str.chars();

            p_chars.next()
        } else {
            None
        };

        let task = if task_match.is_some() {
            String::from(task_match.unwrap().as_str())
        } else {
            String::new()
        };

        Some(Todo {
            raw: line.to_string(),
            is_complete,
            task,
            priority,
            projects: Vec::new(),
            contexts: Vec::new(),
            key_values: HashMap::new(),
        })
    } else {
        None
    }
}

fn main() {
    let examples = [
        "(A) Thank Mom for the meatballs @phone",
        "x (B) Get tires on the van @maintenance",
        "Learn Rust",
    ];

    for example in examples.iter() {
        let t = parse(example);

        if t.is_some() {
            let t = t.unwrap();
            let p = if t.priority.is_some() {
                t.priority.unwrap()
            } else {
                ' '
            };

            println!("Raw={}, IsComplete={}, Priority={}, Task={}", t.raw, t.is_complete, p, t.task);
        }
    }
}
