mod commands;
mod grow;


use std::{env, process};
use std::env::Args;
use dotenv::dotenv;
use crate::commands::{HELP_COMMAND_NAME, run, run_with_params};

fn main() {
    dotenv().ok();

    let mut args: Args = env::args().into_iter();

    let command = match args.next() {
        Some(v) => v,
        None => HELP_COMMAND_NAME.to_string(),
    };

    let result = match args.len() {
        1 | 2 => run(command.as_str()),
        _ => run_with_params(command.as_str(), args.into_iter().skip(2)),
    };

    if let Err(e) = result {
        eprintln!("Mashinka error: {e}");
        process::exit(1);
    }
}