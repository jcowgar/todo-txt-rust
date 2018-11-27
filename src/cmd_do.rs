use gumdrop::Options;
use todo_file::TodoFile;

#[derive(Debug, Options)]
pub struct Opts {
    #[options(help = "Id of todo to mark complete")]
    id: i32,
}

pub fn execute(opts: &Opts) {
}
