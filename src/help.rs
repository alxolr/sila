use enum_iterator::IntoEnumIterator;

use crate::{ports::HelperCommand, ABOUT, AUTHOR, VERSION};

pub struct Help;

impl Help {
    pub fn display() -> String {
        let mut help_str = String::new();
        
        help_str.push_str(&format!("sila@{}\n", VERSION));
        help_str.push_str(&format!("{}\n", ABOUT));
        help_str.push_str(&format!("created by {}\n", AUTHOR));
        help_str.push_str("\nCOMMANDS:\n");


        let mut commands_help = vec![];
        for command in HelperCommand::into_enum_iter() {
            let cmd = match command {
                HelperCommand::Pin =>
                (
                    "pin",
                    "<term1> <term2>",
                    "Pin one or multiple terminals separated by space. Following commands will run on top of pinned ones only."
                ),
                HelperCommand::Unpin =>
                (
                    "unpin",
                    "[term1]",
                    "Unpin all terminals if no argument is provided or the specific ones.",
                ),
                HelperCommand::Ban => 
                (
                    "ban",
                    "<term1> <term2>",
                    "Ban one or multiple terminals separated by space. The following commands will not run in banned terminals"
                ),
                HelperCommand::Unban => 
                (
                    "unban",
                    "[term2]",
                    "Unban the specificed terminals or all if no arguments provided."
                ),
                HelperCommand::List => ("list","","List the active terminal names."),
                HelperCommand::Help => ("help", "", "Displays help information."),
                HelperCommand::Exit => ("exit", "", "Close the application.")
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
    use crate::{help::Help};

    #[test]
    fn test_display_command_exists() {
        assert!(Help::display().contains("one or multiple"));
    }
}
