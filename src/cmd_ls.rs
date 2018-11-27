use gumdrop::Options;
use todo_file::TodoFile;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Options)]
pub struct Opts {
    #[options(help = "Only todos for a given project")]
    project: String,

    #[options(help = "Only todos in a given context")]
    context: String,

    #[options(help = "Only todos at or above a given priority")]
    priority: char,

    #[options(help = "Order by title only")]
    title_order: bool,
}

pub fn execute(opts: &Opts) {
    let f = TodoFile::parse_default();

    if f.is_err() {
        return ();
    }

    let f = f.unwrap();

    let mut index = 0;
    let mut todos = f.todos.clone();

    if opts.project.len() > 0 {
        let project_filter = format!("+{}", opts.project);

        todos = todos
            .into_iter()
            .filter(|t| t.projects.contains(&project_filter))
            .collect();
    }

    if opts.context.len() > 0 {
        let context_filter = format!("@{}", opts.context);

        todos = todos
            .into_iter()
            .filter(|t| t.contexts.contains(&context_filter))
            .collect();
    }

    if opts.priority >= 'A' {
        let priority_ch = opts.priority.to_uppercase().next().unwrap();

        todos = todos
            .into_iter()
            .filter(|t| t.priority.is_some() && t.priority.unwrap() <= priority_ch)
            .collect();
    }

    if opts.title_order {
        todos.sort_by(|a, b| a.cmp_by_title(b));
    } else {
        todos.sort_by(|a, b| a.cmp(b));
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for t in todos {
        let mut out = Vec::new();

        index += 1;

        let color = match t.is_complete {
            true => Color::Green,
            false => match t.priority {
                Some('A') => Color::Red,
                Some('B') => Color::Cyan,
                Some('C') => Color::Magenta,
                Some(_) => Color::Yellow,
                None => Color::White,
            },
        };

        match t.is_complete {
            true => out.push(String::from("[X]")),
            _ => out.push(String::from("[ ]")),
        }

        match t.priority {
            Some(p) => out.push(format!("({})", p)),
            None => out.push(String::from("   ")),
        }

        out.push(t.task.clone());

        stdout.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Could not set foreground color");

        println!("{}: {}", index, out.join(" "));
    }
}
