mod command;
mod config;
mod grow;

use std::env::Args;
use std::{env, process};
use colored::Colorize;

use crate::command::run;

use dotenv::dotenv;

fn main() {
    if dotenv::from_filename("~/.env").is_err() {
        println!("{}", "Consider to place env variables to ~/.env file".yellow());
    }

    let mut args: Args = env::args();
    args.next(); // пропускаем первый параметр, так как это target path

    let result = run(args);

    match result {
        Err(e) => {
            eprintln!("{}{e}", "Mashinka error:".red());
            process::exit(1);
        }
        Ok(mut result) => {
            println!("{}", result.summarize());
        }
    }
}
