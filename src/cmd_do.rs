use gumdrop::Options;
use todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

#[derive(Debug, Options)]
pub struct Opts {
    #[options(help = "Id of todo to mark complete")]
    id: usize,
}

pub fn execute(opts: &Opts) {
  let todos = &mut parse_todos_from_default_file()
    .expect("Could not parse todos from default file");

  if let Some(elem) = todos.get_mut(opts.id - 1) {
    elem.is_complete = true;
  }

  write_todos_to_default_file(&todos)
    .expect("Could not write todos to default file");
}
