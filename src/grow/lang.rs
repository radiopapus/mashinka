#![allow(clippy::must_use_candidate)]

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Lang {
    Ru,
    En,
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::Ru => write!(f, "Ru"),
            Lang::En => write!(f, "En"),
        }
    }
}

// todo use derive_more
impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "ru" => Ok(Lang::Ru),
            "en" => Ok(Lang::En),
            _ => Err(String::from("Unknown language")),
        }
    }
}

impl Lang {
    pub fn to_lowercase(self) -> String {
        self.to_string().to_lowercase()
    }
}
