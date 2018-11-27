#[macro_use]
extern crate lazy_static;
extern crate gumdrop;
extern crate regex;

use gumdrop::Options;

mod cmd_ls;
mod cmd_add;
mod cmd_do;
mod cmd_rm;
mod cmd_help;

mod todo;
mod todo_file;

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
    Help(cmd_help::Opts),

    #[options(help = "List todos")]
    Ls(cmd_ls::Opts),

    #[options(help = "Add a new todo")]
    Add(cmd_add::Opts),

    #[options(help = "Mark a todo as done")]
    Do(cmd_do::Opts),

    #[options(help = "Remove a todo")]
    Rm(cmd_rm::Opts),
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
        Some(Command::Do(copts)) => cmd_do::execute(&copts, &f),
        Some(Command::Rm(copts)) => cmd_rm::execute(&copts, &f),
        Some(Command::Add(copts)) => cmd_add::execute(&copts, &f),
        Some(Command::Help(copts)) => cmd_help::execute(&copts),
        _ => println!("No command given: {:?}", opts),
    }
}
