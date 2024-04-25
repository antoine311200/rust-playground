use super::Command;

fn run_echo(args: Vec<&str>) {
    if args.len() < 1 {
        println!("echo: missing argument");
        return;
    }

    println!("{}", args.join(" "));
}

pub fn get_command() -> Command {
    Command::new("echo", "Prints the arguments", run_echo)
}