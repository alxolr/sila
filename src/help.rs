use enum_iterator::IntoEnumIterator;

use crate::{ports::Commands, ABOUT, AUTHOR, VERSION};

pub struct Help;

impl Help {
    pub fn display() -> String {
        let mut help_str = String::new();
        
        help_str.push_str(&format!("sila@{}\n", VERSION));
        help_str.push_str(&format!("{}\n", ABOUT));
        help_str.push_str(&format!("created by {}\n", AUTHOR));
        help_str.push_str("\nCOMMANDS:\n");


        let mut commands_help = vec![];
        for command in Commands::into_enum_iter() {
            let cmd = match command {
                Commands::Pin =>
                (
                    "pin",
                    "<term1> <term2>",
                    "Pin one or multiple terminals separated by space. Following commands will run on top of pinned ones only."
                ),
                Commands::Unpin =>
                (
                    "unpin",
                    "[term1]",
                    "Unpin all terminals if no argument is provided or the specific ones.",
                ),
                Commands::Ban => 
                (
                    "ban",
                    "<term1> <term2>",
                    "Ban one or multiple terminals separated by space. The following commands will not run in banned terminals"
                ),
                Commands::Unban => 
                (
                    "unban",
                    "[term2]",
                    "Unban the specificed terminals or all if no arguments provided."
                ),
                Commands::List => ("list","","List the active terminal names."),
                Commands::Help => ("help", "", "Displays help information."),
                Commands::Exit => ("exit", "", "Close the application.")
            };

            commands_help.push(cmd);
        }

        commands_help
            .iter()
            .for_each(|item| help_str.push_str(&format!("{0: <7} {1: <17} {2: <10}\n", item.0, item.1, item.2)));

        help_str
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_display_command_exists() {
        let display = Help::display();
        assert_eq!(display.contains("one or multiple"), true);
    }
}
