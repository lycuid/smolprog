use super::{Block, ValueRunner};
use std::{process::Command, sync::Arc};

struct MemoryRunner;

impl ValueRunner for MemoryRunner {
    fn fmt_value(&mut self, string: String) -> String {
        format!(
            "<BtnL=notify_max_mem> {}  </BtnL><Box:Left=#171717:2> </Box>",
            string
        )
    }

    fn get_value(&mut self) -> Option<String> {
        let mem: Vec<u64> = Command::new("free")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())?
            .lines()
            .map(String::from)
            .find(|l| l.starts_with("Mem:"))?
            .split_whitespace()
            .skip(1)
            .map(|c| c.parse().unwrap())
            .collect();

        let used = mem[1];
        let shared = mem[3];

        let total: u64 = (used + shared) >> 10;
        match total {
            0 => None,
            1..=500 => Some(format!("  {:4} MiB", total)),
            501..=1000 => Some(format!("  <Fg=#ffdd59>{:4}</Fg> MiB", total)),
            1001.. => Some(format!(
                "  <Fg=#cc6666>{:.2}</Fg> GiB",
                total as f64 / 1024f64
            )),
        }
    }
}

pub fn create_memory_blk() -> Block {
    Block::Value {
        default_value: "mem: ?",
        interval_ms: 1000,
        create_runner: Arc::new(|| Box::new(MemoryRunner {})),
    }
}
