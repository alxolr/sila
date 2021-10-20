use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator)]
pub enum Commands {
    Pin,
    Unpin,
    Ban,
    Unban,
    List,
    Help,
    Exit,
}

impl Commands {
    pub fn to_enum(str: &str) -> Option<Commands> {
        match str {
            "pin" => Some(Commands::Pin),
            "unpin" => Some(Commands::Unpin),
            "ban" => Some(Commands::Ban),
            "unban" => Some(Commands::Unban),
            "list" => Some(Commands::List),
            "help" => Some(Commands::Help),
            "exit" => Some(Commands::Exit),

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
