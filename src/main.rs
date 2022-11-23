use std::{env, fs};
use mashinka::commands::{CommandResult, HELP_COMMAND_NAME};

use mashinka::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO add checks for args
    match args.len() {
        1 => run(HELP_COMMAND_NAME, &[]),
        2 => run(&args[1], &[]),
        _ => run(&args[1], &args[2..args.len()]),
    };
}
