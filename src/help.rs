use enum_iterator::IntoEnumIterator;

use crate::{ports::Subcommands, ABOUT, AUTHOR, VERSION};

pub struct Help;

impl Help {
    pub fn display() {
        println!("\nsila@{}", VERSION);
        println!("{}", ABOUT);
        println!("created by {}", AUTHOR);
        println!("\nCOMMANDS:");

        let mut commands_help = vec![];
        for command in Subcommands::into_enum_iter() {
            let cmd = match command {
                Subcommands::Pin =>
                (
                    "pin",
                    "<term1> <term2>",
                    "Pin one or multiple terminals separated by space. Following commands will run on top of pinned ones only."
                ),
                Subcommands::Unpin =>
                (
                    "unpin",
                    "[term1]",
                    "Unpin all terminals if no argument is provided or the specific ones.",
                ),
                Subcommands::Ban => 
                (
                    "ban",
                    "<term1> <term2>",
                    "Ban one or multiple terminals separated by space. The following commands will not run in banned terminals"
                ),
                Subcommands::Unban => 
                (
                    "unban",
                    "[term2]",
                    "Unban the specificed terminals or all if no arguments provided."
                ),
                Subcommands::List => ("list","","List the active terminal names."),
                Subcommands::Help => ("help", "", "Displays help information."),
                Subcommands::Exit => ("exit", "", "Close the application.")
            };

            commands_help.push(cmd);
        }

        commands_help
            .iter()
            .for_each(|item| println!("{0: <7} {1: <17} {2: <10}", item.0, item.1, item.2));

        println!("");
    }
}
