use super::{Logger, ValueRunner};
use std::fs::read_to_string;

const BAT_SYM_LEN: usize = 5;

struct BatteryRunner {
    batindex: usize,
    batsym: [&'static str; BAT_SYM_LEN],
}

impl BatteryRunner {
    fn fmt_value(string: String) -> String {
        format!(" {}  ", string)
    }
}

impl ValueRunner for BatteryRunner {
    fn get_value(&mut self) -> Option<String> {
        let capacity_str =
            read_to_string("/sys/class/power_supply/BAT0/capacity").ok()?;
        let capacity: usize = capacity_str.lines().next()?.parse().ok()?;

        let status_str =
            read_to_string("/sys/class/power_supply/BAT0/status").ok()?;
        let status = status_str.lines().next()?;

        let sym = match status {
            "Charging" => self.batsym[self.batindex],
            "Discharging" => self.batsym[capacity * BAT_SYM_LEN / 100],
            _ => self.batsym.iter().last()?,
        };
        self.batindex = (self.batindex + 1) % BAT_SYM_LEN;

        Some(Self::fmt_value(format!("{} {:3}%", sym, capacity)))
    }
}

pub fn create_battery_logger() -> Logger {
    Logger::ValueLogger {
        default_value: BatteryRunner::fmt_value("battery: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| {
            Box::new(BatteryRunner {
                batindex: 0,
                batsym: [" ", " ", " ", " ", " "],
            })
        }),
    }
}
