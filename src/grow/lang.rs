use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Lang {
    Ru,
    En,
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Lang {
    pub fn to_str(self) -> &'static str {
        match self {
            Lang::Ru => "ru",
            Lang::En => "en"
        }
    }

    pub fn from_str(value: &str) -> Lang {
        match value.trim().to_lowercase().as_str() {
            "ru" => Lang::Ru,
            "en" => Lang::En,
            _ => panic!("Unknown language")
        }
    }
}