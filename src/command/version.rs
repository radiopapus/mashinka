#![allow(clippy::must_use_candidate)]

use crate::command::{Command, CommandResult, Error};
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
        println!("{}", VERSION);
        Ok(CommandResult::default())
    }
}
