use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::io::{stdin, stdout};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;

use crate::command_option::CommandOption;
use crate::help::Help;
use crate::ports::*;
use crate::sila::Sila;

mod command_option;
mod help;
mod ports;
mod sila;
mod terminal;

static VERSION: &str = "0.3.1";
static ABOUT: &str = "A command line multiplexer.";
static AUTHOR: &str = "Alexandru Olaru <alxolr@gmail.com>";

#[derive(StructOpt, Debug)]
#[structopt(
    version = VERSION,
    about = ABOUT,
    author = AUTHOR,
    rename_all = "kebab-case"
)]
struct Opt {
    #[structopt(short, default_value = "./sila_config.yaml")]
    /// Provide the config file path
    path: PathBuf,
}

#[derive(Debug)]
struct Output {
    terminal_name: String,
    output: Vec<u8>,
    command: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Opt::from_args();
    let mut sila = Sila::new(cli.path);

    // define the communication channels
    let (tx, rx) = mpsc::channel();

    loop {
        let terminals = sila.active_terminals();
        let terminal_len = terminals.len();

        // logic to handle the drawings of the shell terminal
        print!("> ");
        stdout().flush().unwrap();

        // read the input line
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // stdin().


        // logic to transform each line into CommandOptions if we have pipes then we will have an array of CommandOptions
        // otherwise we will have one input
        let commands = if input.contains('|') {
            input
                .split('|')
                .into_iter()
                .map(|cmd| CommandOption::new(cmd.to_string()))
                .collect::<Vec<CommandOption>>()
        } else {
            let mut vec = Vec::new();
            vec.push(CommandOption::new(input));

            vec
        };

        // we will get the first command from the array of commands and try to see if it's a helper command
        let command = commands.first().unwrap().clone();
        let command_type = Commands::to_enum(command.name.as_ref());

        if command_type.is_some() {
            match command_type.unwrap() {
                Commands::Help => Help::display(),
                Commands::Ban => {
                    if command.args.len() > 0 {
                        sila.ban(command.args);
                    }
                }
                Commands::Unban => {
                    if command.args.len() > 0 {
                        sila.unban(command.args);
                    } else {
                        sila.banned_terminals = HashSet::new();
                    }
                }
                Commands::Pin => {
                    if command.args.len() > 0 {
                        sila.pin(command.args);
                    }
                }
                Commands::Unpin => {
                    if command.args.len() > 0 {
                        sila.unban(command.args);
                    } else {
                        sila.pinned_terminals = HashSet::new();
                    }
                }
                Commands::List => {
                    for terminal in sila.active_terminals() {
                        println!("{}", terminal.name);
                    }
                }
                Commands::Exit => break,
            };
        } else {
            // handle the multithearded logic of running one or multiple commands into separate threads
            let arc_cmds = Arc::new(commands);

            for terminal in terminals {
                let txc = tx.clone();
                let cmds = Arc::clone(&arc_cmds);

                thread::spawn(move || {
                    let commands = cmds.clone();

                    let mut prev_command = None;
                    let mut errors = vec![];

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
                                errors.push(e.to_string().clone());
                            }
                        }
                    }

                    let output = if let Some(final_command) = prev_command {
                        final_command.wait_with_output().unwrap().stdout
                    } else {
                        errors.join(",").as_bytes().to_owned()
                    };

                    txc.send(Output {
                        terminal_name: terminal.name.clone(),
                        output: output.clone(),
                        command: commands
                            .iter()
                            .map(|c| c.clone().to_string())
                            .collect::<Vec<_>>()
                            .join(" | "),
                    })
                    .unwrap();
                });
            }

            for _ in 0..terminal_len {
                let received = rx.recv().unwrap();
                println!(
                    "[{}]> {}\n{}",
                    received.terminal_name,
                    received.command,
                    std::str::from_utf8(&received.output).unwrap()
                );
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Sila Experienced an Error: {}", e);
        std::process::exit(1);
    }
}
