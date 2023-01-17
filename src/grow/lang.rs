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

pub fn slugify(value: &str) -> String {
    let slug: String = value.chars().map(|c|
        match c {
            ' ' => "-",
            'а' => "a",
            'б' => "b",
            'в' => "v",
            'г' => "g",
            'д' => "d",
            'е' => "e",
            'ё' => "e",
            'ж' => "zh",
            'з' => "z",
            'и' => "i",
            'й' => "y",
            'к' => "k",
            'л' => "l",
            'м' => "m",
            'н' => "n",
            'о' => "o",
            'п' => "p",
            'р' => "r",
            'с' => "s",
            'т' => "t",
            'у' => "u",
            'ф' => "f",
            'х' => "h",
            'ц' => "c",
            'ч' => "ch",
            'ш' => "sh",
            'щ' => "sh",
            'ь' => "",
            'ы' => "i",
            'ъ' => "",
            'э' => "e",
            'ю' => "yu",
            'я' => "ya",
            'А' => "A",
            'Б' => "B",
            'В' => "V",
            'Г' => "G",
            'Д' => "D",
            'Е' => "E",
            'Ё' => "E",
            'Ж' => "Zh",
            'З' => "Z",
            'И' => "I",
            'Й' => "Y",
            'К' => "K",
            'Л' => "L",
            'М' => "M",
            'Н' => "N",
            'О' => "O",
            'П' => "P",
            'Р' => "R",
            'С' => "S",
            'Т' => "T",
            'У' => "U",
            'Ф' => "F",
            'Х' => "H",
            'Ц' => "C",
            'Ч' => "Ch",
            'Ш' => "Sh",
            'Щ' => "Sh",
            'Ь' => "",
            'Ы' => "i",
            'Ъ' => "",
            'Э' => "E",
            'Ю' => "Yu",
            'Я' => "Ya",
            _ => ""
        }
    ).collect();

    slug.trim().to_lowercase()
}
