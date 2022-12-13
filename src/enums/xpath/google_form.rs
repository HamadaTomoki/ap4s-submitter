use std::fmt;

// "//*[@id="mG61Hd"]/div[2]/div/div[2]/div[1]/div/div/div[2]/div/div[1]/div/div[1]/input"
// "//*[@id="mG61Hd"]/div[2]/div/div[2]/div[2]/div/div/div[2]/div/div[1]/div/div[1]/input"
//
// "//*[@id="i14"]"
// "//*[@id="i17"]/div[3]/div"
// "//*[@id="i20"]/div[3]/div"
// "//*[@id="i23"]/div[3]/div"
// "//*[@id="i26"]/div[3]/div"
//
// "//*[@id="i30"]"
// "//*[@id="i33"]/div[3]/div"
// "//*[@id="i36"]/div[3]/div"
// "//*[@id="i39"]/div[3]/div"
// "//*[@id="i42"]/div[3]/div"
//
// "//*[@id="i46"]"
// "//*[@id="i62"]"

// "//*[@id="i14"]/span"
// //*[@id="i30"]/span
pub enum Questions {
    Title(i32),
}
impl fmt::Display for Questions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"//*[@id="i{}"]/span"#,
            match *self {
                Self::Title(i) => i,
            }
        )
    }
}

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
