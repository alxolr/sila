use std::{io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]

pub struct Terminal {
    pub name: String,
    pub path: String,
}

pub struct Config;

impl Config {
    // Load the terminals from config yaml file
    pub fn load(path: PathBuf) -> Vec<Terminal> {
        let input = std::fs::File::open(path).expect("Provided file does not exist");
        let rdr = BufReader::new(input);
        let terminals: Vec<Terminal> =
            serde_yaml::from_reader(rdr).expect("Could not process the config.yaml file");

        terminals
    }
}
