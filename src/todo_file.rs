use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs::File;

use todo;

#[derive(Debug)]
pub struct TodoFile {
    /// Filename of todo file
    pub filename: String,

    /// List of todos contained in the file
    pub todos: Vec<Option<todo::Todo>>,
}

impl TodoFile {
    /// Parse a todo file
    pub fn parse(filename: &str) -> Result<TodoFile, io::Error> {
        let f = try!(File::open(filename));
        let file = BufReader::new(&f);
        let todos = file.lines().map(|line| {
            let lineu = line.unwrap();
            todo::Todo::parse(&lineu)
        }).collect();

        Ok(TodoFile {
            filename: String::from(filename),
            todos,
        })
    }
}

