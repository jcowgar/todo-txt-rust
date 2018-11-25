#[macro_use] extern crate lazy_static;
extern crate regex;

mod todo;

fn main() {
    let examples = [
        "(A) Thank Mom for the meatballs @phone",
        "x (B) Get tires on the van @maintenance due:2018-12-01",
        "Learn Rust +Learn @computer",
    ];

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
}

