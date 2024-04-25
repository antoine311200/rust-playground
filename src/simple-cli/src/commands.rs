pub mod ls;
pub mod echo;

pub struct Command {
    pub name: String,
    pub description: String,
    pub run: fn(Vec<&str>),
}

impl Command {
    pub fn new(name: &str, description: &str, run: fn(Vec<&str>)) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            run,
        }
    }
}