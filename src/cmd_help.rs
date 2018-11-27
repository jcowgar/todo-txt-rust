use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Opts {
    #[options(free)]
    free: Vec<String>,
}

pub fn execute(_opts: &Opts) {
}
