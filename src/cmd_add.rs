use gumdrop::Options;
use todo_file::TodoFile;

#[derive(Debug, Options)]
pub struct Opts {
    #[options(help = "Priority of the new todo")]
    priority: char,
}

pub fn execute(opts: &Opts, f: &TodoFile) {
}
