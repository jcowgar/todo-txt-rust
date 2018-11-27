use gumdrop::Options;
use todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

#[derive(Debug, Options)]
pub struct Opts {
    #[options(help = "Id of todo to remove")]
    id: usize,
}

pub fn execute(opts: &Opts) {
  let todos = &mut parse_todos_from_default_file()
    .expect("Could not parse todos from default file");

  todos.remove(opts.id - 1);

  write_todos_to_default_file(&todos)
    .expect("Could not write todos to default file");
}
