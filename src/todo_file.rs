use std::fs::File;
use std::io;
use std::io::prelude::Write;
use std::io::BufRead;
use std::io::BufReader;

use todo::Todo;

#[cfg(target_family = "unix")]
const TODO_FILE: &str = "~/.todo-txt/todo.txt";

#[cfg(target_family = "windows")]
const TODO_FILE: &str = "\\Users\\jerem\\.todo-txt\\todo.txt";

#[derive(Debug)]
/// File containing many Todo's
///
/// See <Todo>
pub struct TodoFile {
    /// Filename of todo file
    pub filename: String,

    /// List of todos contained in the file
    pub todos: Vec<Todo>,
}

impl TodoFile {
    /// Parse a todo file
    pub fn parse(filename: &str) -> Result<TodoFile, io::Error> {
        let f = try!(File::open(filename));
        let file = BufReader::new(&f);
        let todos = file
            .lines()
            .map(|line| {
                let lineu = line.unwrap();
                Todo::parse(&lineu)
            }).filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect();

        Ok(TodoFile {
            filename: String::from(filename),
            todos,
        })
    }

    pub fn parse_default() -> Result<TodoFile, io::Error> {
        TodoFile::parse(TODO_FILE)
    }

    pub fn write(&self, filename: &str) -> std::io::Result<()> {
        let mut f = File::create(filename)?;

        for t in &self.todos {
            f.write(&format!("{}\n", &t.serialize()).into_bytes())?;
        }

        f.flush()?;

        Ok(())
    }

    pub fn write_default(&self) -> std::io::Result<()> {
        self.write(TODO_FILE)
    }
}
