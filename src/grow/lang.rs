use std::fmt::{Display, Formatter};

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

impl Lang {
    pub fn to_lowercase(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn from_str(value: &str) -> Lang {
        match value.trim().to_lowercase().as_str() {
            "ru" => Lang::Ru,
            "en" => Lang::En,
            _ => panic!("Unknown language {}", value),
        }
    }
}
