#[derive(Debug, PartialEq, Clone)]
pub struct InputLine {
    pub name: String,
    pub args: Vec<String>,
}

impl InputLine {
    pub fn to_string(self) -> String {
        format!("{} {}", self.name, self.args.join(" "))
    }

    pub fn from_input(input: String) -> Self {
        let things = {
            let mut iter = Vec::new();
            let mut buffer = Vec::new();

            for ch in input.trim().chars() {
                if ch == ' ' {
                    if !quotes_are_closed(&buffer) {
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
                iter.push(buffer.iter().collect::<String>());
            }

            iter
        };

        let name = things.first().unwrap().clone();
        let args: Vec<String> = things.into_iter().skip(1).collect();

        InputLine { name, args }
    }
}

fn quotes_are_closed(buffer: &Vec<char>) -> bool {
    if buffer.contains(&'\'') && buffer.contains(&'"') {
        return is_even(count_occurences(buffer, &'\'')) && is_even(count_occurences(buffer, &'"'));
    } else if buffer.contains(&'\'') {
        return is_even(count_occurences(buffer, &'\''));
    } else if buffer.contains(&'"') {
        return is_even(count_occurences(buffer, &'"'));
    }

    true
}

fn count_occurences(vec: &Vec<char>, c: &char) -> usize {
    vec.iter().filter(|ch| *ch == c).count()
}

fn is_even(nb: usize) -> bool {
    nb % 2 == 0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_with_arguments() {
        let scenarios = vec![
            (
                "git".to_string(),
                InputLine {
                    name: "git".to_string(),
                    args: Vec::new(),
                },
            ),
            (
                "git describe".to_string(),
                InputLine {
                    name: "git".to_string(),
                    args: vec!["describe".to_string()],
                },
            ),
            (
                "git tag -a -m 'Some test'".to_string(),
                InputLine {
                    name: "git".to_string(),
                    args: vec![
                        "tag".to_string(),
                        "-a".to_string(),
                        "-m".to_string(),
                        "'Some test'".to_string(),
                    ],
                },
            ),
            (
                "git tag -a -m 'Some test' --dry_run".to_string(),
                InputLine {
                    name: "git".to_string(),
                    args: vec![
                        "tag".to_string(),
                        "-a".to_string(),
                        "-m".to_string(),
                        "'Some test'".to_string(),
                        "--dry_run".to_string(),
                    ],
                },
            ),
            (
                r#"git tag -a -m "Some test" --dry_run"#.to_string(),
                InputLine {
                    name: "git".to_string(),
                    args: vec![
                        "tag".to_string(),
                        "-a".to_string(),
                        "-m".to_string(),
                        r#""Some test""#.to_string(),
                        "--dry_run".to_string(),
                    ],
                },
            ),
            (
                r#"git tag -a -m "Some test 'appears here'" --dry_run"#.to_string(),
                InputLine {
                    name: "git".to_string(),
                    args: vec![
                        "tag".to_string(),
                        "-a".to_string(),
                        "-m".to_string(),
                        r#""Some test 'appears here'""#.to_string(),
                        "--dry_run".to_string(),
                    ],
                },
            ),
            (
                r#"ncu "/@conform\/batch.*/""#.to_string(),
                InputLine {
                    name: "ncu".to_string(),
                    args: vec!["\"/@conform\\/batch.*/\"".to_string()],
                },
            ),
        ];
        for (input, output) in scenarios {
            assert_eq!(InputLine::from_input(input), output);
        }
    }

    #[test]
    fn check_should_continue_to_add() {
        let scenarios = vec![
            ("Some test '".chars().collect::<Vec<_>>(), false),
            ("Some test 'test'".chars().collect::<Vec<_>>(), true),
            (r#"Some test "test""#.chars().collect::<Vec<_>>(), true),
        ];

        for (buffer, expected) in scenarios {
            assert_eq!(quotes_are_closed(&buffer), expected);
        }
    }
}
