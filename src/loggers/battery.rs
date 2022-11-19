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

    fn calculate(&mut self) -> Option<String> {
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

impl ValueRunner for BatteryRunner {
    fn get_value(&mut self) -> String {
        self.calculate()
            .or_else(|| Some(BatteryRunner::fmt_value("battery: ?".into())))
            .unwrap()
    }
}

pub fn create_battery_logger() -> Logger {
    Logger::ValueLogger {
        interval_ms: 1000,
        runner: Box::new(BatteryRunner { index: 0 }),
    }
}
