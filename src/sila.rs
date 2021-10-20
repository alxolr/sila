use std::collections::HashSet;
use std::path::PathBuf;

use crate::terminal::{self, Terminal};

pub struct Sila {
    // All terminals loaded from config file
    pub all_terminals: Vec<Terminal>,

    // the set of pinned terminals has the highest priority
    // if this one is empty it will look into excluded set
    pub pinned_terminals: HashSet<Terminal>,

    // the set of excluded terminals
    // all the terminals in excluded set will be removed
    pub excluded_terminals: HashSet<Terminal>,
}

impl Sila {
    pub fn new(path: PathBuf) -> Self {
        let terminals = terminal::Config::load(path);

        Sila {
            all_terminals: terminals,
            pinned_terminals: HashSet::new(),
            excluded_terminals: HashSet::new(),
        }
    }

    pub fn pin(&mut self, terminal_names: Vec<String>) {
        for name in terminal_names {
            let terminal = self.all_terminals.iter().find(|t| t.name == name);

            if terminal.is_some() {
                self.pinned_terminals.insert(terminal.unwrap().clone());
            }
        }
    }

    pub fn unpin(&mut self, terminal_names: Vec<String>) {
        for name in terminal_names {
            // try to find the terminal in hashset
            let maybe_terminal = self.pinned_terminals.iter().find(|term| term.name == name);

            if maybe_terminal.is_some() {
                // weird hack to unwrap take ownership and then pass by reference
                // hope one day to understand what is going on there
                let terminal = maybe_terminal.unwrap().clone();
                self.pinned_terminals.remove(&terminal);
            }
        }
    }

    pub fn active_terminals(&self) -> Vec<Terminal> {
        // if we have pinned terminals than return the pinned terminals
        if self.pinned_terminals.len() > 0 {
            return self
                .pinned_terminals
                .clone()
                .into_iter()
                .collect::<Vec<Terminal>>();
        }

        if self.excluded_terminals.len() > 0 {
            return self
                .all_terminals
                .clone()
                .into_iter()
                .filter(|terminal| !self.excluded_terminals.contains(terminal))
                .collect();
        }

        self.all_terminals.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{sila::Sila, terminal::Terminal};
    use std::collections::HashSet;

    fn get_terminals() -> Vec<Terminal> {
        vec![
            Terminal {
                name: "T1".to_string(),
                path: "/path/t1".to_string(),
            },
            Terminal {
                name: "T2".to_string(),
                path: "/path/t2".to_string(),
            },
            Terminal {
                name: "T3".to_string(),
                path: "/path/t3".to_string(),
            },
        ]
    }

    #[test]
    fn test_pinned_terminals_works_as_expected() {
        let all_terminals = get_terminals().clone();

        let active_terminals: Vec<Terminal> = all_terminals
            .clone()
            .into_iter()
            .filter(|t| t.name == "T1".to_string())
            .collect();

        let pinned_terminals = active_terminals
            .clone()
            .into_iter()
            .collect::<HashSet<Terminal>>();

        let sila = Sila {
            all_terminals,
            pinned_terminals,
            excluded_terminals: HashSet::new(),
        };

        assert_eq!(sila.active_terminals(), active_terminals);
    }

    #[test]
    fn test_excluded_terminals_works_as_expected() {
        let all_terminals = get_terminals().clone();

        let expected_active_terminals: Vec<Terminal> = all_terminals
            .clone()
            .into_iter()
            .filter(|t| vec!["T1", "T2"].contains(&t.name.as_str()))
            .collect();

        let excluded_terminals = all_terminals
            .clone()
            .into_iter()
            .skip(2)
            .collect::<HashSet<Terminal>>();

        let sila = Sila {
            all_terminals,
            pinned_terminals: HashSet::new(),
            excluded_terminals,
        };

        assert_eq!(sila.active_terminals(), expected_active_terminals);
    }
}
