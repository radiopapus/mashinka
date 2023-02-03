#![allow(clippy::must_use_candidate)]

use crate::command::{Command, CommandResult, Details, Error};
use crate::config::VERSION;

pub struct Version;

impl Version {
    pub fn new() -> Box<Version> {
        Box::new(Self)
    }
}

/// Выводит версию утилиты
impl Command for Version {
    fn run(&self) -> Result<CommandResult, Error> {
        let command = String::new();

        let mut details = Details::new();
        details.push(String::from(""), VERSION.to_string());

        Ok(CommandResult { command, details })
    }
}
