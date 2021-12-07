use super::{Block, ValueRunner};
use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

struct CpuRunner {
    previous: [u64; 7],
}

impl ValueRunner for CpuRunner {
    fn fmt_value(&mut self, string: String) -> String {
        format!(
            "<BtnL=notify_max_cpu> {}  </BtnL><Box:Left=#171717:2> </Box>",
            string
        )
    }

    fn get_value(&mut self) -> Option<String> {
        let file = File::open("/proc/stat").ok()?;
        let mut line = String::new();
        BufReader::new(file).read_line(&mut line).ok()?;

        let cpu: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .take(7)
            .map(|n| n.parse().unwrap())
            .collect();

        let new_total: u64 = cpu.iter().sum();
        let prev_total: u64 = self.previous.iter().sum();
        let total = new_total - prev_total;

        let new_used: u64 = cpu.iter().take(3).sum();
        let prev_used: u64 = self.previous.iter().take(3).sum();
        let used = new_used - prev_used;

        self.previous = cpu.try_into().unwrap();

        let percentage = (100 * used) / total;
        match percentage {
            0..=25 => Some(format!("  {:2}%", percentage)),
            26..=65 => Some(format!("  <Fg=#ffdd59>{:2}</Fg>%", percentage)),
            66..=100 => Some(format!("  <Fg=#cc6666>{:2}</Fg>%", percentage)),
            _ => None,
        }
    }
}

pub fn create_cpu_blk() -> Block {
    Block::Value {
        default_value: "cpu: ?",
        interval_ms: 1000,
        create_runner: Arc::new(|| Box::new(CpuRunner { previous: [0; 7] })),
    }
}
