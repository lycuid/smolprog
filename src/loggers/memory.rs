//! Logs memory usage.
use super::{Logger, ValueRunner};
use std::process::Command;

struct MemoryRunner;

impl MemoryRunner {
    fn fmt_value(string: String) -> String {
        format!(
            "<BtnL=notify_max_mem> {}  </BtnL><Box:Left=#171717:2> </Box>",
            string
        )
    }

    fn calculate(&mut self) -> Option<String> {
        let mem: Vec<u64> = Command::new("free")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())?
            .lines()
            .find(|l| l.starts_with("Mem:"))?
            .split_whitespace()
            .skip(1)
            .map(|c| c.parse().unwrap())
            .collect();

        // total = (used + shared) / 1024
        Some((mem[1] + mem[3]) >> 10)
            .and_then(|t| match t {
                0 => None,
                1..=500 => Some(format!("  {:4} MiB", t)),
                501..=1000 => Some(format!("  <Fg=#ffdd59>{:4}</Fg> MiB", t)),
                1001.. => Some(format!(
                    "  <Fg=#cc6666>{:.2}</Fg> GiB",
                    t as f64 / 1024.
                )),
            })
            .map(Self::fmt_value)
    }
}

impl ValueRunner for MemoryRunner {
    fn get_value(&mut self) -> String {
        self.calculate()
            .or_else(|| Some(MemoryRunner::fmt_value("mem: ?".into())))
            .unwrap()
    }
}

pub fn create_memory_logger() -> Logger {
    Logger::ValueLogger {
        interval_ms: 1000,
        runner: Box::new(MemoryRunner {}),
    }
}
