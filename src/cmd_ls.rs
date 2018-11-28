use gumdrop::Options;
use todo_file;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Options)]
pub struct Opts {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "Only todos at or above a given priority")]
    priority: char,

    #[options(help = "Order by title only")]
    title_order: bool,

    #[options(help = "Limit to only the first X todo items")]
    limit: usize,
}

pub fn execute(opts: &Opts) {
    let mut todos = todo_file::parse_todos_from_default_file()
        .expect("Could not parse default todo.txt file");

    if opts.priority >= 'A' {
        let priority_ch = opts.priority.to_uppercase().next().unwrap();

        todos = todos
            .into_iter()
            .filter(|t| t.priority.is_some() && t.priority.unwrap() <= priority_ch)
            .collect();
    }

    for text in &opts.free {
        todos = todos.into_iter()
            .filter(|t| t.task.contains(text))
            .collect();        
    }

    if opts.title_order {
        todos.sort_by(|a, b| a.cmp_by_title(b));
    } else {
        todos.sort_by(|a, b| a.cmp(b));
    }

    if opts.limit > 0 {
        todos = todos
            .into_iter()
            .take(opts.limit)
            .collect();
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for t in todos {
        let mut out = Vec::new();

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

        println!("{:4}: {}", t.index + 1, out.join(" "));
    }
}
