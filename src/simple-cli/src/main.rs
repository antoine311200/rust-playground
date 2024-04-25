
use std::io::Write;

use colored::Colorize;

struct Command {
    pub name: String,
    pub description: String,
    pub run: fn(Vec<&str>),
}

impl Command {
    fn new(name: &str, description: &str, run: fn(Vec<&str>)) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            run,
        }
    }
}

fn run_echo(args: Vec<&str>) {
    if args.len() < 1 {
        println!("echo: missing argument");
        return;
    }

    println!("{}", args.join(" "));
}

fn run_ls(_: Vec<&str>) {
    let current_dir = std::env::current_dir().unwrap();
    let entries = std::fs::read_dir(current_dir).unwrap();

    let filenames: Vec<String> = entries
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    for filename in filenames {
        println!("  {}", if filename.starts_with(".") { filename.blue().italic() } else { filename.normal() });

    }
}

fn get_git_branch() -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("branch")
        .output()
        .ok()?;

    let output = String::from_utf8(output.stdout).ok()?;
    let output = output.lines().find(|l| l.starts_with('*'))?;

    Some(output[2..].to_string())
}

fn main() {
    let all_commands: [Command; 3] = [
        Command::new("echo", "Prints the arguments", run_echo),
        Command::new("exit", "Exits the application", |_: Vec<&str>| {}),
        Command::new("ls", "Lists the files in the current directory", run_ls)
    ];

    println!("Welcome to the {} {} application", "Simple".cyan().italic(), "CLI".bold().purple());
    println!("Type 'help' to see the available commands");
    // Get the name of the folder not the full path
    let current_dir = std::env::current_dir().unwrap();
    let dir_name = current_dir.file_name().unwrap().to_str().unwrap();

    loop {
        print!("{}{}{} ", "[".green().bold(), dir_name.green().bold(), "]".green().bold());
        print!("{}{}{} ", "(".purple(), get_git_branch().unwrap_or("".to_string()).purple().italic(), ")".purple());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            println!("{} See you soon!", "Goodbye!".yellow().bold().italic());
            break;
        }

        if input == "help" {
            println!("Available commands:");
            for command in all_commands.iter() {
                println!("  {} - {}", command.name.green(), command.description);
            }
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command_name = parts[0];
        let command_args = &parts[1..];

        let command = all_commands.iter().find(|c| c.name == command_name);
        match command {
            Some(command) => {
                (command.run)(command_args.to_vec());
            }
            None => println!("{} {}", "Command not found: ".red(), command_name.bold().underline().yellow()),
        }
    }
}
