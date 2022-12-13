use std::fmt;

pub enum Question {
    Answer,
    Collect,
}
impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = r#"//*[@id="mainCol"]/div[2]"#;
        match *self {
            Self::Answer => write!(
                f,
                r#"{base}/div[2] | {base}/section | {base}/p[1] | {base}/article"#,
                base = base
            ),
            Self::Collect => write!(f, "{}", r#"//*[@id="answerChar"]"#),
        }
    }
}
