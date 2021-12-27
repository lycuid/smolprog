use super::{Logger, ValueRunner};
use std::process::Command;

struct SessionRunner;

impl SessionRunner {
    fn fmt_value(string: String) -> String {
        format!(
            "<BtnL=notify_tmux_ls> {}  </BtnL><Box:Left=#171717:2> </Box>",
            string
        )
    }
}

impl ValueRunner for SessionRunner {
    fn get_value(&mut self) -> Option<String> {
        Command::new("tmux")
            .arg("ls")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| format!("<Fg=#9b59b6>  {}</Fg>", s.lines().count()))
            .map(Self::fmt_value)
    }
}

pub fn create_session_logger() -> Logger {
    Logger::ValueLogger {
        default_value: SessionRunner::fmt_value("sessions: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| Box::new(SessionRunner {})),
    }
}
