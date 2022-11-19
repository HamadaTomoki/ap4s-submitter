use std::fmt;

#[allow(dead_code)]
pub enum Question {
    Answer,
    Collect,
}
impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Answer =>
                    r#"//*[@id="mainCol"]/div[2]/div[2] | //*[@id="mainCol"]/div[2]/section | //*[@id="mainCol"]/div[2]/p[1] | //*[@id="mainCol"]/div[2]/article"#,
                Self::Collect => r#"//*[@id="answerChar"]"#,
            }
        )
    }
}
