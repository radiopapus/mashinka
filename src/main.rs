mod commands;
mod config;
mod grow;

use std::env::Args;
use std::{env, process};

use crate::commands::run;

use dotenv::dotenv;

fn main() {
    if dotenv().is_err() {
        eprintln!("Check file .env exists. See .env-example for details.");
        process::exit(1);
    }

    let mut args: Args = env::args();
    args.next(); // пропускаем первый параметр, так как это target path

    let result = run(args);

    match result {
        Err(e) => {
            eprintln!("Mashinka error: {e}");
            process::exit(1);
        }
        Ok(mut result) => {
            println!("{}", result.summarize());
        }
    }
}
