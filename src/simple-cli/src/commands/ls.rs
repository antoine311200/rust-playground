use colored::Colorize;
use std::io::Write;

use super::Command;
use super::super::utils::ellipsis;

fn colorize(filename: &str) -> String {
    if filename.starts_with(".") {
        filename.blue().italic().to_string()
    }
    else {
        filename.to_string()
    }
}

fn run_ls(_: Vec<&str>) {
    let current_dir = std::env::current_dir().unwrap();
    let entries = std::fs::read_dir(current_dir).unwrap();

    let n_cols = 3;
    let spacing = 40;
    let max_length: usize = 20;

    let filenames: Vec<String> = entries
    .map(|entry| entry.unwrap().file_name().into_string().unwrap())
    .map(|filename| ellipsis(&filename, max_length))
    .collect();

    std::io::stdout().flush().unwrap();
    for chunk in filenames.chunks(n_cols) {
        for (_, filename) in chunk.iter().enumerate() {
            print!("{:<width$}", colorize(filename), width = spacing);
        }
        println!();
    }
}

pub fn get_command() -> Command {
    Command::new("ls", "Lists the files in the current directory", run_ls)
}