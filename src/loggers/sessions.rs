//! Logs number of active tmux sessions.
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

    fn calculate(&mut self) -> Option<String> {
        Command::new("tmux")
            .arg("ls")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| format!("<Fg=#9b59b6>  {}</Fg>", s.lines().count()))
            .map(Self::fmt_value)
    }
}

impl ValueRunner for SessionRunner {
    fn get_value(&mut self) -> String {
        self.calculate()
            .or_else(|| Some(SessionRunner::fmt_value("sessions: ?".into())))
            .unwrap()
    }
}

pub fn create_session_logger() -> Logger {
    Logger::ValueLogger {
        interval_ms: 1000,
        runner: Box::new(SessionRunner {}),
    }
}
