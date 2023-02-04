#![allow(clippy::must_use_candidate)]

use colored::Colorize;
use crate::command::{Command, CommandResult, Error};

pub struct Help;

impl Help {
    pub fn new() -> Box<Help> {
        Box::new(Self)
    }
}

impl Command for Help {
    fn run(&self) -> Result<CommandResult, Error> {
        println!("{} {}", "Usage:".green().bold(), "mashinka CMD --arg-name --argv-name=value".blue());
        println!("{} {}", "Example:".green().bold(), "mashinka publish --dry-run".blue());
        println!();
        println!("{}", "Available commands:".green().bold());
        println!("{} - uses draft file as a source of content and create to post and translation
based on specified lang.", "publish".blue());
        println!("{} - builds data file for indexing system (elasticlunr) based on posts content.", "index".blue());
        println!("{} - uploads and extract data to cloud storage (selectel for now).", "deploy".blue());
        println!("{} - shows content from HELP.md file.", "help".blue());
        println!("{} - shows mashinka version.", "version".blue());
        println!();
        println!("{}", "Miscellaneous:".green().bold());
        println!("{} - runs command in so called dry-run mode( without side artifacts).", "--dry-run".blue());

        Ok(CommandResult::default())
    }
}
