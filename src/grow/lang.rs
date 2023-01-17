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
    let slug: String = value.chars().fold(String::new(), |mut ss, c| {
            match c {
                ',' | '!' | '?' | '#' | '$' | '@' | '*' | '%' | '^' => ss.push_str(""),
                ' ' => ss.push('-'),
                'а' => ss.push('a'),
                'б' => ss.push('b'),
                'в' => ss.push('v'),
                'г' => ss.push('g'),
                'д' => ss.push('d'),
                'е' => ss.push('e'),
                'ё' => ss.push('e'),
                'ж' => ss.push_str("zh"),
                'з' => ss.push('z'),
                'и' => ss.push('i'),
                'й' => ss.push('y'),
                'к' => ss.push('k'),
                'л' => ss.push('l'),
                'м' => ss.push('m'),
                'н' => ss.push('n'),
                'о' => ss.push('o'),
                'п' => ss.push('p'),
                'р' => ss.push('r'),
                'с' => ss.push('s'),
                'т' => ss.push('t'),
                'у' => ss.push('u'),
                'ф' => ss.push('f'),
                'х' => ss.push('h'),
                'ц' => ss.push('c'),
                'ч' => ss.push_str("ch"),
                'ш' => ss.push_str("sh"),
                'щ' => ss.push_str("sh"),
                'ь' => ss.push_str(""),
                'ы' => ss.push('i'),
                'ъ' => ss.push_str(""),
                'э' => ss.push('e'),
                'ю' => ss.push_str("yu"),
                'я' => ss.push_str("ya"),
                'А' => ss.push('A'),
                'Б' => ss.push('B'),
                'В' => ss.push('V'),
                'Г' => ss.push('G'),
                'Д' => ss.push('D'),
                'Е' => ss.push('E'),
                'Ё' => ss.push('E'),
                'Ж' => ss.push_str("Zh"),
                'З' => ss.push('Z'),
                'И' => ss.push('I'),
                'Й' => ss.push('Y'),
                'К' => ss.push('K'),
                'Л' => ss.push('L'),
                'М' => ss.push('M'),
                'Н' => ss.push('N'),
                'О' => ss.push('O'),
                'П' => ss.push('P'),
                'Р' => ss.push('R'),
                'С' => ss.push('S'),
                'Т' => ss.push('T'),
                'У' => ss.push('U'),
                'Ф' => ss.push('F'),
                'Х' => ss.push('H'),
                'Ц' => ss.push('C'),
                'Ч' => ss.push_str("Ch"),
                'Ш' => ss.push_str("Sh"),
                'Щ' => ss.push_str("Sh"),
                'Ь' => ss.push_str(""),
                'Ы' => ss.push('i'),
                'Ъ' => ss.push_str(""),
                'Э' => ss.push('E'),
                'Ю' => ss.push_str("Yu"),
                'Я' => ss.push_str("Ya"),
                _ => ss.push(c)
            };
            ss
        });

    slug.trim().to_lowercase()
}
