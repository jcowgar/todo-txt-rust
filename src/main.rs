#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate gumdrop;
extern crate regex;
extern crate termcolor;
extern crate dirs;
extern crate config;

use gumdrop::Options;
use std::error::Error;

mod cfg;

mod cmd_add;
mod cmd_archive;
mod cmd_do;
mod cmd_help;
mod cmd_ls;
mod cmd_rm;
mod cmd_tag;

mod todo;
mod todo_file;

#[derive(Debug, Options)]
struct MyOptions {
	#[options(help = "Print help message")]
	help: bool,

	#[options(help = "Use an alternative configuration file")]
	config: String,

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

	#[options(help = "Archive completed todos")]
	Archive(cmd_archive::Opts),

	#[options(help = "Mark a todo as done")]
	Do(cmd_do::Opts),

	#[options(help = "Remove a todo")]
	Rm(cmd_rm::Opts),

	#[options(help = "Tag a todo")]
	Tag(cmd_tag::Opts),
}

fn try_main() -> Result<(), Box<Error>> {
	let opts = MyOptions::parse_args_default_or_exit();
	let config_file = if opts.config.len() > 0 {
		Some(opts.config.as_str())
	} else {
		None
	};

	cfg::read_config(config_file)?;

	match opts.command {
		Some(Command::Ls(copts)) => cmd_ls::execute(&copts),
		Some(Command::Do(copts)) => cmd_do::execute(&copts),
		Some(Command::Rm(copts)) => cmd_rm::execute(&copts),
		Some(Command::Add(copts)) => cmd_add::execute(&copts),
		Some(Command::Archive(copts)) => cmd_archive::execute(&copts),
		Some(Command::Help(copts)) => cmd_help::execute(&copts),
		Some(Command::Tag(copts)) => cmd_tag::execute(&copts),
		_ => println!("No command given: {:?}", opts),
	}

	Ok(())
}

fn main() {
	try_main().unwrap();
}
