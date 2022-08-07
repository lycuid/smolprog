//! Logs date and time.
use super::{Logger, ValueRunner};
use std::process::Command;

const BORDER: &'static str = "#089CAC";
const BACKGROUND: &'static str = "#171717";

struct DateRunner;

impl DateRunner {
    fn fmt_value(string: String) -> String {
        format!(
            "<Box:Top|Bottom|Left|Right={border}><Bg={background}><Fn=1> {text} </Fn></Bg></Box>",
            border = BORDER,
            background = BACKGROUND,
            text = string
        )
    }
}

impl ValueRunner for DateRunner {
    fn get_value(&mut self) -> Option<String> {
        Command::new("date")
            .arg("+ %a, %b %d %H:%M:%S ")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|s| s.lines().map(|s| s.to_string()).next())
            .map(Self::fmt_value)
    }
}

pub fn create_date_logger() -> Logger {
    Logger::ValueLogger {
        default_value: DateRunner::fmt_value("date: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| Box::new(DateRunner {})),
    }
}
