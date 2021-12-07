use super::{Block, ValueRunner};
use std::{process::Command, sync::Arc};

struct TmuxlsRunner;

impl ValueRunner for TmuxlsRunner {
    fn fmt_value(&mut self, string: String) -> String {
        format!(
            "<BtnL=notify_tmux_ls> {}  </BtnL><Box:Left=#171717:2> </Box>",
            string
        )
    }

    fn get_value(&mut self) -> Option<String> {
        Command::new("tmux")
            .arg("ls")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|s| {
                Some(format!("<Fg=#9b59b6>  {}</Fg>", s.lines().count()))
            })
    }
}

pub fn create_tmuxls_blk() -> Block {
    Block::Value {
        default_value: "sessions: ?",
        interval_ms: 1000,
        create_runner: Arc::new(|| Box::new(TmuxlsRunner {})),
    }
}
