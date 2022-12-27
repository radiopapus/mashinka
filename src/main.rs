mod commands;
mod config;
mod grow;

use std::env::Args;
use std::{env, process};

use crate::commands::run;

use dotenv::dotenv;

fn main() {
    if let Err(_) = dotenv() {
        eprintln!("Check file .env exists. See .env-example for details.");
        process::exit(1);
    }

    let mut args: Args = env::args().into_iter();
    args.next(); // пропускаем первый параметр, так как это target path

    let result = run(args);

    match result {
        Err(e) => {
            eprintln!("Mashinka error: {e}");
            process::exit(1);
        }
        Ok(result) => {
            println!("{}", result.summarize());
        }
    }
}
