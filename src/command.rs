#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn from_input(input: String) -> Self {
        let things = {
            let mut iter = Vec::new();
            let mut buffer = Vec::new();
            for ch in input.trim().chars() {
                if ch == ' ' {
                    // check if the space is inside the single or double slashes then include the space
                    if (buffer.contains(&'\'') && is_even(count_char(&buffer, &ch)))
                        || (buffer.contains(&'"') && is_even(count_char(&buffer, &ch)))
                    {
                        buffer.push(ch);
                    } else {
                        // push the the result in the iterator and flush the buffer
                        let word = buffer.iter().collect::<String>();
                        iter.push(word);
                        buffer = Vec::new();
                    }
                } else {
                    buffer.push(ch);
                }
            }

            if buffer.len() > 0 {
                iter.push(buffer.iter().collect::<String>())
            }

            iter
        };

        let name = things.first().unwrap().clone();
        let args: Vec<String> = things.into_iter().skip(1).collect();

        Command { name, args }
    }
}

fn count_char(vec: &Vec<char>, c: &char) -> usize {
    vec.iter().filter(|ch| *ch == c).count()
}

fn is_even(nb: usize) -> bool {
    nb % 2 == 0
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
        let input = "git tag -a -m 'Some test'".to_string();
        assert_eq!(
            Command::from_input(input),
            Command {
                name: "git".to_string(),
                args: vec![
                    "tag".to_string(),
                    "-a".to_string(),
                    "-m".to_string(),
                    "'Some test'".to_string()
                ]
            }
        )
    }
}
