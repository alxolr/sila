use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::io::{stdin, stdout};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::{io::BufReader, path::PathBuf};
use structopt::StructOpt;

use crate::command::Command as SilaCommand;

mod command;

#[derive(StructOpt, Debug)]
#[structopt(
    version = "0.2.0",
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

        let commands = if input.contains('|') {
            input
                .split('|')
                .into_iter()
                .map(|cmd| SilaCommand::from_input(cmd.to_string()))
                .collect::<Vec<SilaCommand>>()
        } else {
            let mut vec = Vec::new();
            vec.push(SilaCommand::from_input(input));

            vec
        };

        match commands.first().unwrap().name.as_ref() {
            "exit" => break,
            "count" => println!("{} terminals", len),
            _ => {
                let arc_cmds = Arc::new(commands);

                for terminal in clone_terminals {
                    let txc = tx.clone();
                    let cmds = Arc::clone(&arc_cmds);

                    thread::spawn(move || {
                        let commands = cmds.clone();

                        let mut prev_command = None;

                        for command in commands.iter() {
                            let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
                                if output.stdout.is_some() {
                                    Stdio::from(output.stdout.unwrap())
                                } else {
                                    Stdio::inherit()
                                }
                            });

                            let output = Command::new(command.name.clone())
                                .args(command.args.clone())
                                .stdin(stdin)
                                .stdout(Stdio::piped())
                                .current_dir(terminal.path.clone())
                                .spawn();

                            match output {
                                Ok(output) => {
                                    prev_command = Some(output);
                                }
                                Err(e) => {
                                    prev_command = None;
                                    eprintln!("{}", e)
                                }
                            }
                        }

                        if let Some(final_command) = prev_command {
                            let output = final_command.wait_with_output().unwrap();

                            txc.send(Output {
                                terminal_name: terminal.name.clone(),
                                output: output.stdout.clone(),
                                command: commands
                                    .iter()
                                    .map(|c| c.clone().to_string())
                                    .collect::<Vec<_>>()
                                    .join(" | "),
                            })
                            .unwrap();
                        }
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
