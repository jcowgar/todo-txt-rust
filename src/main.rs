#[macro_use] extern crate lazy_static;
extern crate gumdrop;
extern crate regex;

use gumdrop::Options;

mod todo;
mod todo_file;

use todo_file::TodoFile;

#[cfg(target_family = "linux")]
const TODO_FILE: &str = "~/.todo-txt/todo.txt";

#[cfg(target_family = "windows")]
const TODO_FILE: &str = "\\Users\\jerem\\.todo-txt\\todo.txt";

#[derive(Debug, Options)]
struct MyOptions {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "Print help message")]
    help: bool,

    #[options(help = "Verbose output")]
    verbose: bool,
}

fn ls(tfile: &TodoFile) {
    let mut index = 0;

    for t in &tfile.todos {
        if let Some(ti) = t {
            let mut out = Vec::new();

            index += 1;

            match ti.priority {
                Some(p) => out.push(format!("({})", p)),
                None => out.push(String::from("   ")),
            }

            out.push(ti.task.clone());

            println!("{}: {}", index, out.join(" "));
        }
    }
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();
    
    if opts.verbose {
        println!("File: {}", TODO_FILE);
        println!("");
    }

    let f = TodoFile::parse(TODO_FILE);
    match f {
        Ok(parsed_file) => ls(&parsed_file),
        _ => println!("Couldn't parse file"),
    }
}
