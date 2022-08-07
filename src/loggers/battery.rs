//! Logs machine battery usage.
use super::{Logger, ValueRunner};
use std::fs::read_to_string;

struct BatteryRunner {
    index: usize,
}

impl BatteryRunner {
    const BAT_SYM: [&'static str; 5] = [" ", " ", " ", " ", " "];
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

        let bat_sym_len = Self::BAT_SYM.len();
        let sym = if bat_sym_len > 0 {
            self.index = (self.index + 1) % bat_sym_len;
            match status {
                "Charging" => Self::BAT_SYM[self.index],
                "Discharging" => Self::BAT_SYM[capacity * bat_sym_len / 100],
                _ => Self::BAT_SYM[bat_sym_len - 1],
            }
        } else {
            ""
        };

        Some(Self::fmt_value(format!("{} {:3}%", sym, capacity)))
    }
}

pub fn create_battery_logger() -> Logger {
    Logger::ValueLogger {
        default_value: BatteryRunner::fmt_value("battery: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| Box::new(BatteryRunner { index: 0 })),
    }
}
