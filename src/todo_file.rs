use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

use todo::Todo;

#[derive(Debug)]
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
}
