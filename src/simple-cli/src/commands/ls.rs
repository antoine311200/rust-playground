use colored::Colorize;
use std::fs::DirEntry;
use std::io::Write;

use super::Command;
use super::super::utils::ellipsis;

enum FileType {
    HiddenFile,
    File,
    Directory,
}

fn get_type(filename: &Result<DirEntry, std::io::Error>) -> FileType {
    match filename {
        Err(_) => FileType::File,
        Ok(filename) => {
            let filename = filename.file_name().into_string().unwrap();
            if filename.starts_with(".") {
                FileType::HiddenFile
            }
            else {
                let metadata = std::fs::metadata(filename).unwrap();
                if metadata.is_dir() {
                    FileType::Directory
                }
                else {
                    FileType::File
                }
            }
        }
    }
}

fn colorize(filename: &str, filetype: &FileType) -> String {
    match filetype {
        FileType::HiddenFile => filename.blue().italic().to_string(),
        FileType::File => filename.to_string(),
        FileType::Directory => filename.yellow().bold().to_string(),
    }
}

fn run_ls(_: Vec<&str>) {
    let current_dir = std::env::current_dir().unwrap();
    let entries = std::fs::read_dir(current_dir).unwrap();

    let n_cols = 3;
    let spacing = 40;
    let max_length: usize = 20;

    let filenames: Vec<(String, FileType)> = entries
    .filter_map(|entry| {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                Some((ellipsis(&file_name.to_string(), max_length), get_type(&Ok(entry))))
            }
            else {
                None
            }
        }
        else {
            None
        }
    })
    .collect();

    std::io::stdout().flush().unwrap();
    for chunk in filenames.chunks(n_cols) {
        for (_, (filename, filetype)) in chunk.iter().enumerate() {
            print!("{:<width$}", colorize(filename, filetype), width = spacing);
        }
        println!();
    }
}

pub fn get_command() -> Command {
    Command::new("ls", "Lists the files in the current directory", run_ls)
}