use todo_file::TodoFile;
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
    #[options(help = "Only todos for a given project")]
    project: String,

    #[options(help = "Only todos in a given context")]
    context: String,

    #[options(help = "Only todos at or above a given priority")]
    priority: char,
}

pub fn execute(opts: &Opts, f: &TodoFile) {
    let mut index = 0;

    for t in &f.todos {
        if let Some(ti) = t {
            let mut out = Vec::new();

            index += 1;

            match ti.priority {
                Some(p) => out.push(format!("({})", p)),
                None => out.push(String::from("   ")),
            }

            out.push(ti.task.clone());

            println!("{}: {}", index, out.join(" "));
        }
    }
}
