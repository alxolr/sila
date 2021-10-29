use std::error::Error;
use std::io::Write;
use std::io::{stdin, stdout};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::command_option::CommandOption;
use crate::ports::HelperCommand;

mod command_option;
mod help;
mod ports;
mod runner;
mod terminal;

static VERSION: &str = "0.3.2";
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

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Opt::from_args();
    let mut runner = runner::Runner::new(cli.path);

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let commands = CommandOption::from_input(input);

        let first_command = commands.first().unwrap().clone();
        let helper_command_maybe = HelperCommand::to_enum(&first_command.name);

        if helper_command_maybe.is_some() {
            runner.execute_helper_cmd(first_command)
        } else {
            runner.execute_cmds(commands);
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("Sila Experienced an Error: {}", e);
        std::process::exit(1);
    }
}
