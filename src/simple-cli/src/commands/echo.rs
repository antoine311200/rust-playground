use super::Command;
use std::sync::Arc;

fn run() {
    println!("Hello from the echo command");
}

pub fn load() -> Vec<Arc<Command>> {
    vec![Arc::new(Command::new("echo", "Prints the given text", run))]
}