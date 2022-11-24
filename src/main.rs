use std::{env, fs};
use mashinka::commands::{CommandResult, HELP_COMMAND_NAME};

use mashinka::run;

fn main() -> Result<CommandResult, String> {
    let args: Vec<String> = env::args().collect();
    return match args.len() {
        1 => run(HELP_COMMAND_NAME, &[]),
        2 => run(&args[1], &[]),
        _ => run(&args[1], &args[2..args.len()]),
    };
}
