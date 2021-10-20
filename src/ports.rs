use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator)]
pub enum Subcommands {
    Pin,
    Unpin,
    Ban,
    Unban,
    List,
    Help,
    Exit,
}

pub trait Pinable {
    fn pin(&mut self, names: Vec<String>);
    fn unpin(&mut self, names: Vec<String>);
}

pub trait Banable {
    fn ban(&mut self, names: Vec<String>);
    fn unban(&mut self, names: Vec<String>);
}
