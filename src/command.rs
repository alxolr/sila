#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn from_input(input: String) -> Self {
        let iter: Vec<String> = input
            .trim()
            .split_whitespace()
            .into_iter()
            .map(|c| c.clone().to_string())
            .collect();

        let name = iter.first().unwrap().clone();
        let args: Vec<String> = iter.into_iter().skip(1).collect();

        Command { name, args }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one_word_command() {
        let input = "git".to_string();

        assert_eq!(
            Command::from_input(input),
            Command {
                name: "git".to_string(),
                args: Vec::new()
            }
        )
    }

    #[test]
    fn parse_command_with_arguments() {
        let input = "git describe".to_string();
        assert_eq!(
            Command::from_input(input),
            Command {
                name: "git".to_string(),
                args: vec!["describe".to_string()]
            }
        )
    }

    #[test]
    fn parse_commands_with_single_slashes() {
        let input = "git tag -a -m 'Some interesting tag name'".to_string();
        assert_eq!(
            Command::from_input(input),
            Command {
                name: "git".to_string(),
                args: vec![
                    "tag".to_string(),
                    "-a".to_string(),
                    "-m".to_string(),
                    "'Some interesting tag name'".to_string()
                ]
            }
        )
    }
}
