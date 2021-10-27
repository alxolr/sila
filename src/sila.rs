use std::collections::HashSet;
use std::path::PathBuf;

use crate::{
    ports::{Banable, Pinable},
    terminal::{self, Terminal},
};

pub struct Sila {
    /// All terminals loaded from config file
    pub all_terminals: Vec<Terminal>,

    /// the set of pinned terminals has the highest priority
    /// if this one is empty it will look into excluded set
    pub pinned_terminals: HashSet<Terminal>,

    /// the set of excluded terminals
    /// all the terminals in excluded set will be removed
    pub banned_terminals: HashSet<Terminal>,
}

impl Sila {
    pub fn new(path: PathBuf) -> Self {
        let terminals = terminal::Config::load(path);

        Sila {
            all_terminals: terminals,
            pinned_terminals: HashSet::new(),
            banned_terminals: HashSet::new(),
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

        if self.banned_terminals.len() > 0 {
            return self
                .all_terminals
                .clone()
                .into_iter()
                .filter(|terminal| !self.banned_terminals.contains(terminal))
                .collect();
        }

        self.all_terminals.clone()
    }

    fn add_to_set(&mut self, set_type: SetType, names: Vec<String>) {
        // pretty sure ther should be a more elegant way to do it
        let set = match set_type {
            SetType::Banned => &mut self.banned_terminals,
            SetType::Pinned => &mut self.pinned_terminals,
        };

        for name in names {
            let terminal = self.all_terminals.iter().find(|term| term.name == name);

            if terminal.is_some() {
                set.insert(terminal.unwrap().clone());
            }
        }
    }

    fn remove_from_set(&mut self, set_type: SetType, names: Vec<String>) {
        let set = match set_type {
            SetType::Banned => &mut self.banned_terminals,
            SetType::Pinned => &mut self.pinned_terminals,
        };

        for name in names {
            // try to find the terminal in hashset
            let maybe_terminal = set.iter().find(|term| term.name == name);

            if maybe_terminal.is_some() {
                let terminal = maybe_terminal.unwrap().clone();
                set.remove(&terminal);
            }
        }
    }
}

impl Banable for Sila {
    fn ban(&mut self, terminal_names: Vec<String>) {
        self.add_to_set(SetType::Banned, terminal_names);
    }

    fn unban(&mut self, terminal_names: Vec<String>) {
        self.remove_from_set(SetType::Banned, terminal_names)
    }
}

impl Pinable for Sila {
    fn pin(&mut self, terminal_names: Vec<String>) {
        self.add_to_set(SetType::Pinned, terminal_names);
    }

    fn unpin(&mut self, terminal_names: Vec<String>) {
        self.remove_from_set(SetType::Pinned, terminal_names)
    }
}

enum SetType {
    Banned,
    Pinned,
}

#[cfg(test)]
mod tests {
    use crate::{
        ports::{Banable, Pinable},
        sila::Sila,
        terminal::Terminal,
    };
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

    // Cool thing that you can add special helper
    // implementation in unit tests to help you with the code
    impl Terminal {
        fn new(str: &str) -> Terminal {
            Terminal {
                name: str.to_string(),
                path: format!("/path/{}", str.to_lowercase()).to_string(),
            }
        }
    }

    fn to_hashset(vector: Vec<Terminal>) -> HashSet<Terminal> {
        vector.clone().into_iter().collect::<HashSet<Terminal>>()
    }

    #[test]
    fn test_pinned_terminals_works_as_expected() {
        let terminals_to_pin = vec![Terminal::new("T1")];
        let sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: to_hashset(terminals_to_pin.clone()),
            banned_terminals: HashSet::new(),
        };

        assert_eq!(sila.active_terminals(), terminals_to_pin);
    }

    #[test]
    fn test_banned_terminals_works_as_expected() {
        let banned_terminals = to_hashset(vec![Terminal::new("T3")]);

        let sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: HashSet::new(),
            banned_terminals,
        };

        assert_eq!(
            sila.active_terminals(),
            // expects the first two terminals to be available
            get_terminals().into_iter().take(2).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_pin_command_invalid_name() {
        let mut sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: HashSet::new(),
            banned_terminals: HashSet::new(),
        };

        sila.pin(vec!["T4".to_string()]);
        assert_eq!(sila.pinned_terminals, HashSet::new())
    }

    #[test]
    fn test_pin_command_valid_name() {
        let mut sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: HashSet::new(),
            banned_terminals: HashSet::new(),
        };

        let terminal = Terminal::new("T3");

        sila.pin(vec!["T3".to_string()]);
        let mut expected_set = HashSet::new();

        expected_set.insert(terminal);
        assert_eq!(sila.pinned_terminals, expected_set);
    }

    #[test]
    fn test_unpin_no_arguments() {
        let mut sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: HashSet::new(),
            banned_terminals: HashSet::new(),
        };
        sila.unpin(vec!["T3".to_string()]);

        assert_eq!(sila.pinned_terminals, HashSet::new());
    }

    #[test]
    fn test_unpin_with_arguments() {
        let mut sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: to_hashset(get_terminals().clone()),
            banned_terminals: HashSet::new(),
        };

        sila.unpin(vec!["T3".to_string()]);
        let expected_set = get_terminals()
            .into_iter()
            .take(2)
            .collect::<HashSet<Terminal>>();

        assert_eq!(sila.pinned_terminals, expected_set);
    }

    #[test]
    fn test_ban_one_terminal() {
        let mut sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: HashSet::new(),
            banned_terminals: HashSet::new(),
        };

        sila.ban(vec!["T3".to_string()]);

        assert_eq!(
            sila.active_terminals(),
            get_terminals()
                .clone()
                .into_iter()
                .take(2)
                .collect::<Vec<Terminal>>()
        );
    }

    #[test]
    fn test_unban_no_arguments() {
        let mut sila = Sila {
            all_terminals: get_terminals().clone(),
            pinned_terminals: HashSet::new(),
            banned_terminals: HashSet::new(),
        };

        sila.ban(vec!["T3".to_string()]);
        sila.unban(vec!["T3".to_string()]);

        assert_eq!(sila.active_terminals(), get_terminals().clone());
    }
}
