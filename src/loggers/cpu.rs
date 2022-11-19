//! Logs cpu usage.
use super::{Logger, ValueRunner};
use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
};

struct CpuRunner {
    previous: [u64; 7],
}

impl CpuRunner {
    fn fmt_value(string: String) -> String {
        format!(
            "<BtnL=notify_max_cpu> {}  </BtnL><Box:Left=#171717:2> </Box>",
            string
        )
    }

    fn calculate(&mut self) -> Option<String> {
        let file = File::open("/proc/stat").ok()?;
        let mut line = String::new();
        BufReader::new(file).read_line(&mut line).ok()?;

        let cpu: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .take(7)
            .map(|n| n.parse().unwrap())
            .collect();

        let total = {
            let new_total: u64 = cpu.iter().sum();
            let prev_total: u64 = self.previous.iter().sum();
            new_total - prev_total
        };
        let used = {
            let new_used: u64 = cpu.iter().take(3).sum();
            let prev_used: u64 = self.previous.iter().take(3).sum();
            new_used - prev_used
        };
        self.previous = cpu.try_into().unwrap();

        Some((100 * used) / total)
            .and_then(|percent| match percent {
                0..=25 => Some(format!("  {:3}%", percent)),
                26..=65 => Some(format!("  <Fg=#ffdd59>{:3}</Fg>%", percent)),
                66..=100 => Some(format!("  <Fg=#cc6666>{:3}</Fg>%", percent)),
                _ => None,
            })
            .map(Self::fmt_value)
    }
}

impl ValueRunner for CpuRunner {
    fn get_value(&mut self) -> String {
        self.calculate()
            .or_else(|| Some(CpuRunner::fmt_value("cpu: ?".into())))
            .unwrap()
    }
}

pub fn create_cpu_logger() -> Logger {
    Logger::ValueLogger {
        interval_ms: 1000,
        runner: Box::new(CpuRunner { previous: [0; 7] }),
    }
}
