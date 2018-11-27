#[macro_use] extern crate lazy_static;
extern crate regex;

mod todo;
mod todo_file;

#[derive(Debug, Options)]
struct MyOptions {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "Print help message")]
    help: bool,

    #[options(help = "Verbose output")]
    verbose: bool,
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();

    let parsed = examples.iter()
    	.map(|v| todo::Todo::parse(v))
    	.filter(|v| v.is_some())
    	.collect::<Vec<Option<todo::Todo>>>();

    for example in parsed.iter() {
        match example {
	        None => println!("Could not parse"),
	        Some(task) => println!("{:?}", task),
        };
    }

    let f = todo_file::TodoFile::parse("/home/jeremy/.todo-txt/todo.txt");
    match f {
        Ok(parsed_file) => for t in parsed_file.todos {
            println!("{:?}", t);
        },
        _ => println!("Couldn't parse file"),
    }
}

