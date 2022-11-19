use std::fmt;

#[allow(dead_code)]
pub const QUESTIONS: &str = r#"//*[@class="M7eMe"]/span"#;

#[allow(dead_code)]
pub enum Student {
    Id,
    Name,
    ClassId,
}
impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[{}]/div/div/div[2]/div/div[1]/div/div[1]/input"#,
            match &self {
                Self::ClassId => 1,
                Self::Id => 2,
                Self::Name => 3,
            }
        )
    }
}

#[allow(dead_code)]
pub enum Answers {
    A(i32),
    I(i32),
    U(i32),
    E(i32),
}
impl fmt::Display for Answers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = match *self {
            Self::A(x) => (x, 1),
            Self::I(x) => (x, 2),
            Self::U(x) => (x, 3),
            Self::E(x) => (x, 4),
        };
        write!(
            f,
            r#"//*[@id="mG61Hd"]/div[2]/div/div[2]/div[{}]/div/div/div[2]/div[1]/div/span/div/div[{}]/label"#,
            i.0 + 4,
            i.1
        )
    }
}
