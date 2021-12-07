use super::{Block, ValueRunner};
use std::{process::Command, sync::Arc};

static BORDER: &'static str = "#089CAC";
static SEPERATOR_COLOR: &'static str = "#171717";

struct DateRunner;

impl ValueRunner for DateRunner {
    fn fmt_value(&mut self, string: String) -> String {
        format!("<Box:Top={border}><Box:Bottom={border}><Box:Left={border}><Box:Right={border}><Bg={seperator_color}><Fn=1> {text} </Fn></Bg></Box></Box></Box></Box>", border = BORDER, seperator_color = SEPERATOR_COLOR, text = string)
    }

    fn get_value(&mut self) -> Option<String> {
        Command::new("date")
            .arg("+ %a, %b %d %H:%M:%S ")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|s| s.lines().map(|s| s.to_string()).next())
    }
}

pub fn create_date_blk() -> Block {
    Block::Value {
        default_value: "date: ?",
        interval_ms: 1000,
        create_runner: Arc::new(|| Box::new(DateRunner {})),
    }
}
