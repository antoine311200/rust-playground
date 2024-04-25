mod utils;
mod commands;

use std::io::Write;
use colored::Colorize;

use commands::Command;
use utils::get_git_branch;


fn get_commands() -> Vec<Command> {
    vec![
        commands::ls::get_command(),
        commands::echo::get_command(),
    ]
}

fn main() {

    let all_commands = get_commands();

    println!("Welcome to the {} {} application", "Simple".cyan().italic(), "CLI".bold().purple());
    println!("Type 'help' to see the available commands");

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
                println!("  {} - {}", command.name.yellow().bold(), command.description);
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
