use crate::{ABOUT, AUTHOR, VERSION};

pub struct Help;

impl Help {
    pub fn display() {
        println!("\nsila@{}", VERSION);
        println!("{}", ABOUT);
        println!("created by {}", AUTHOR);
        println!("\nCOMMANDS:");
        vec![
            (
                "list",
                "",
                "List the terminal names."
            ),
            (
                "pin",
                "<term1> <term2>",
                "Pin one or multiple terminals separated by space. Following commands will be run on top of pinned ones."
            ),
            (
                "unpin",
                "[term1]",
                "Unpin all terminals if no argument is provided or the specific ones.",
            ),
                        (
                "count",
                "",
                "Count the number of terminals.",
            ),
            (
                "exit",
                "",
                "Close the application.",
            ),
        ].into_iter().for_each(|item| println!("{0: <7} {1: <17} {2: <10}", item.0, item.1, item.2));
        println!("");
    }
}
