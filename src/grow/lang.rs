#![allow(clippy::must_use_candidate)]

use std::collections::HashMap;
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

pub fn slugify(value: &str, mapping: &str) -> String {
    let value = value.trim().to_lowercase();

    if value.is_empty() {
        return value;
    }

    let value = value.replace(' ', "-");

    let mut map = HashMap::new();

    for line in mapping.lines() {
        let (char_ru, char_en) = line.split_once(';').unwrap();
        map.insert(char_ru, char_en);
    }

    let mut slug = String::new();

    for char in value.chars() {
        let char_as_string = String::from(char);
        let char = char_as_string.as_str();
        slug.push_str(
            map.get(char).unwrap_or(&char)
        );
    }

    slug.trim().to_lowercase()
}