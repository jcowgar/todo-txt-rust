use gumdrop::Options;
use todo::Todo;
use todo_file::TodoFile;

#[derive(Debug, Options)]
pub struct Opts {
  #[options(free)]
  free: Vec<String>,

  #[options(help = "Priority of the new todo")]
  priority: char,
}

pub fn execute(opts: &Opts) {
  let task = opts.free.join(" ");
  let priority = opts.priority.to_uppercase().next();
  let t = Todo::new(&task, false, priority);

  let f = TodoFile::parse_default();

  if f.is_err() {
      return ();
  }

  let mut f = f.unwrap();

  f.todos.push(t);

  f.write_default()
    .expect("Could not write todo.txt file");
}
