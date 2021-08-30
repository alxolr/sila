use serde::{Deserialize, Serialize};
use std::io::Write;
use std::io::{stdin, stdout};
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::{io::BufReader, path::PathBuf, str::FromStr};

struct Cli {
    path: PathBuf,
}

impl Cli {
    fn new(path: PathBuf) -> Self {
        Cli { path }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Terminal {
    name: String,
    path: String,
}

#[derive(Debug)]
struct Output {
    terminal_name: String,
    output: Vec<u8>,
    command: String,
}

fn main() {
    let file_path = std::env::args().nth(1).expect("Could not parse file path");
    let cli = Cli::new(PathBuf::from_str(&file_path).unwrap());
    let input = std::fs::File::open(cli.path).unwrap();
    let rdr = BufReader::new(input);
    let terminals: Vec<Terminal> = serde_yaml::from_reader(rdr).unwrap();
    let len = terminals.len();
    let (tx, rx) = mpsc::channel();

    loop {
        let clone_terminals = terminals.clone();
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let (command, args) = {
            let parts = input.trim().split_whitespace().collect::<Vec<_>>();
            let command = parts.first().unwrap().clone().to_string();
            let mut args = Vec::new();

            for arg in parts.iter().skip(1) {
                args.push(arg.to_owned().to_string())
            }

            (command, args)
        };

        if &command == "exit" {
            break;
        }

        let arc_cmd = Arc::new(command);
        let arc_args = Arc::new(args);

        for terminal in clone_terminals {
            let txc = tx.clone();
            let cmd = arc_cmd.clone();
            let args = arc_args.clone();

            thread::spawn(move || {
                let child = Command::new(cmd.to_string())
                    .args(args.iter())
                    .current_dir(PathBuf::from(terminal.path))
                    .output()
                    .expect("Worked fine");

                txc.send(Output {
                    terminal_name: terminal.name.clone(),
                    output: child.stdout.clone(),
                    command: format!("{} {}", cmd.to_string(), args.join(" ").to_string()),
                })
                .unwrap();
            });
        }

        for _ in 0..len {
            let received = rx.recv().unwrap();
            println!(
                "{}> {}\n{}",
                received.terminal_name,
                received.command,
                std::str::from_utf8(&received.output).unwrap()
            );
        }
    }
}
