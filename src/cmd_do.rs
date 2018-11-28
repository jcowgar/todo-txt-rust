use gumdrop::Options;
use todo_file::{parse_todos_from_default_file, write_todos_to_default_file};

#[derive(Debug, Options)]
pub struct Opts {
    #[options(free)]
    free: Vec<String>,
}

pub fn execute(opts: &Opts) {
  let todos = &mut parse_todos_from_default_file()
    .expect("Could not parse todos from default file");
  
  for id in &opts.free {
    let iid = id.parse::<usize>().unwrap();
    if let Some(t) = todos.get_mut(iid - 1) {
      t.is_complete = !t.is_complete;
    }
  }

  write_todos_to_default_file(&todos)
    .expect("Could not write todos to default file");
}
