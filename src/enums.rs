pub mod xpath;

use std::fmt;

#[allow(dead_code)]
pub enum Answers {
    A,
    I,
    U,
    E,
}

impl fmt::Display for Answers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::A => "ア",
                Self::I => "イ",
                Self::U => "ウ",
                Self::E => "エ",
            }
        )
    }
}

#[allow(dead_code)]
pub enum Url<'a> {
    GoogleSearch(&'a str),
}

impl fmt::Display for Url<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Url::GoogleSearch(title) => write!(f, r#"https://www.google.com/search?q={}"#, title),
        }
    }
}
