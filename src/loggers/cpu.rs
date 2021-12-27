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
}

impl ValueRunner for CpuRunner {
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
        let result = match percentage {
            0..=25 => Some(format!("  {:3}%", percentage)),
            26..=65 => Some(format!("  <Fg=#ffdd59>{:3}</Fg>%", percentage)),
            66..=100 => Some(format!("  <Fg=#cc6666>{:3}</Fg>%", percentage)),
            _ => None,
        };

        result.map(Self::fmt_value)
    }
}

pub fn create_cpu_logger() -> Logger {
    Logger::ValueLogger {
        default_value: CpuRunner::fmt_value("cpu: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| Box::new(CpuRunner { previous: [0; 7] })),
    }
}
