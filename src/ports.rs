use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, Eq, PartialEq)]
pub enum HelperCommand {
    Pin,
    Unpin,
    Ban,
    Unban,
    List,
    Help,
    Exit,
}

impl HelperCommand {
    pub fn to_enum(str: &str) -> Option<HelperCommand> {
        match str {
            "pin" => Some(HelperCommand::Pin),
            "unpin" => Some(HelperCommand::Unpin),
            "ban" => Some(HelperCommand::Ban),
            "unban" => Some(HelperCommand::Unban),
            "list" => Some(HelperCommand::List),
            "help" => Some(HelperCommand::Help),
            "exit" => Some(HelperCommand::Exit),

            _ => None,
        }
    }
}

pub trait Pinable {
    fn pin(&mut self, names: Vec<String>);
    fn unpin(&mut self, names: Vec<String>);
}

pub trait Banable {
    fn ban(&mut self, names: Vec<String>);
    fn unban(&mut self, names: Vec<String>);
}

#[cfg(test)]
mod tests {
    use crate::ports::HelperCommand;

    #[test]
    fn test_to_enum_returns_good_part() {
        let scenarios = vec!["pin", "unpin", "ban", "unban", "list", "help", "exit"];

        for scenario in scenarios {
            assert!(HelperCommand::to_enum(scenario).is_some())
        }

        assert_eq!(HelperCommand::to_enum("test"), None);
    }
}
