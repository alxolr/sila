use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    description: String,
    version: String,
    authors: Vec<String>,
}

pub struct Help;

impl Help {
    pub fn display() {
        let contents =
            fs::read_to_string("Cargo.toml").expect("Could not load the Cargo toml file.");
        let config: Config = toml::from_str(&contents).unwrap();

        println!("\n{}@{}", config.package.name, config.package.version);
        println!("{}", config.package.description);
        println!("created by {}", config.package.authors.join(""));
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
