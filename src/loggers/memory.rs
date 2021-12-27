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
}

impl ValueRunner for MemoryRunner {
    fn get_value(&mut self) -> Option<String> {
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

        // (used + shared) >> 10
        let total: u64 = (mem[1] + mem[3]) >> 10;
        let result = match total {
            0 => None,
            1..=500 => Some(format!("  {:4} MiB", total)),
            501..=1000 => Some(format!("  <Fg=#ffdd59>{:4}</Fg> MiB", total)),
            1001.. => Some(format!(
                "  <Fg=#cc6666>{:.2}</Fg> GiB",
                total as f64 / 1024f64
            )),
        };

        result.map(Self::fmt_value)
    }
}

pub fn create_memory_logger() -> Logger {
    Logger::ValueLogger {
        default_value: MemoryRunner::fmt_value("mem: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| Box::new(MemoryRunner {})),
    }
}
