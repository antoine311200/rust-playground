// pub struct Command {
//     pub name: String,
//     pub description: String,
//     pub run: fn(),
// }

// impl Command {
//     pub fn new(name: &str, description: &str, run: fn()) -> Command {
//         Command {
//             name: name.to_string(),
//             description: description.to_string(),
//             run,
//         }
//     }
// }

// pub mod commands {
//     use super::Command;
//     mod echo;

//     pub fn load_commands() -> Vec<Arc<Command>> {
//         let mut commands = Vec::new();

//         commands.extend(load()); // Use the imported load function
//         commands.extend(echo::load());

//         commands
//     }
// }