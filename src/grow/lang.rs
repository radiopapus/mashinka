#![allow(clippy::must_use_candidate)]

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Lang {
    Ru,
    En,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::Ru
    }
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

    fn from_str(s: &str) -> Result<Lang, String> {
        match s.trim().to_lowercase().as_str() {
            "ru" => Ok(Lang::Ru),
            "en" => Ok(Lang::En),
            _ => Err(s.to_string())
        }
    }
}

impl Lang {
    pub fn to_lowercase(self) -> String {
        self.to_string().to_lowercase()
    }
}
