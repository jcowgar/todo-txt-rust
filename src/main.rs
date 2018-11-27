#[macro_use]
extern crate lazy_static;
extern crate gumdrop;
extern crate regex;

use gumdrop::Options;

mod todo;
mod todo_file;
mod cmd_ls;

use todo_file::TodoFile;

#[cfg(target_family = "linux")]
const TODO_FILE: &str = "~/.todo-txt/todo.txt";

#[cfg(target_family = "windows")]
const TODO_FILE: &str = "\\Users\\jerem\\.todo-txt\\todo.txt";

#[derive(Debug, Options)]
struct MyOptions {
    #[options(help = "Print help message")]
    help: bool,

    #[options(help = "Verbose output")]
    verbose: bool,

    #[options(command)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "Show help for a command")]
    Help(HelpOpts),

    #[options(help = "List todos")]
    Ls(cmd_ls::Opts),

    #[options(help = "Add a new todo")]
    Add(AddOpts),

    #[options(help = "Mark a todo as done")]
    Do(DoOpts),

    #[options(help = "Remove a todo")]
    Rm(RmOpts),
}

#[derive(Debug, Options)]
struct HelpOpts {
    #[options(free)]
    free: Vec<String>,
}

#[derive(Debug, Options)]
struct AddOpts {
    #[options(help = "Priority of the new todo")]
    priority: char,
}

#[derive(Debug, Options)]
struct DoOpts {
    #[options(help = "Id of todo to mark complete")]
    id: i32,
}

#[derive(Debug, Options)]
struct RmOpts {
    #[options(help = "Id of todo to remove")]
    id: i32,
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();
    
    if opts.verbose {
        println!("File: {}", TODO_FILE);
        println!("");
    }

    let f = TodoFile::parse(TODO_FILE);

    if f.is_err() {
        return ();
    }

    let f = f.unwrap();

    match opts.command {
        Some(Command::Ls(copts)) => cmd_ls::execute(&copts, &f),
        _ => println!("No command given: {:?}", opts),
    }
}
