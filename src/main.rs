use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::io::{stdin, stdout};
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::{io::BufReader, path::PathBuf};
use structopt::StructOpt;

mod command;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.1.2",
    about = "Terminal multiplexer",
    author = "Alexandru Olaru <alxolr@gmail.com>",
    rename_all = "kebab-case"
)]
struct Cli {
    #[structopt(help = "Provide configuration yaml file")]
    path: PathBuf,
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

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();
    let input = std::fs::File::open(cli.path).unwrap();
    let rdr = BufReader::new(input);
    let terminals: Vec<Terminal> = serde_yaml::from_reader(rdr).unwrap();
    let len = terminals.len();
    let (tx, rx) = mpsc::channel();

    loop {
        let clone_terminals = terminals.clone();
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = command::Command::from_input(input);

        match command.name.as_ref() {
            "exit" => break,
            "count" => println!("{} terminals", len),
            _ => {
                let arc_cmd = Arc::new(command);

                for terminal in clone_terminals {
                    let txc = tx.clone();
                    let cmd = Arc::clone(&arc_cmd);

                    thread::spawn(move || {
                        let child = Command::new(cmd.name.clone())
                            .args(cmd.args.clone())
                            .current_dir(PathBuf::from(terminal.path))
                            .output()
                            .expect("Worked fine");

                        txc.send(Output {
                            terminal_name: terminal.name.clone(),
                            output: child.stdout.clone(),
                            command: format!("{} {}", cmd.name, cmd.args.join(" ").to_string()),
                        })
                        .unwrap();
                    });
                }

                for _ in 0..len {
                    let received = rx.recv().unwrap();
                    println!(
                        "[{}]> {}\n{}",
                        received.terminal_name,
                        received.command,
                        std::str::from_utf8(&received.output).unwrap()
                    );
                }
            }
        };
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Sila Experienced an Error: {}", e);
        std::process::exit(1);
    }
}
