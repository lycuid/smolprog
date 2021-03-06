#![allow(dead_code)]

#[cfg(feature = "battery")]
mod battery;
#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "date")]
mod date;
#[cfg(feature = "memory")]
mod memory;
#[cfg(feature = "network")]
mod network;
#[cfg(feature = "sessions")]
mod sessions;
#[cfg(feature = "volume")]
mod volume;

pub trait ValueRunner {
    fn get_value(&mut self) -> Option<String>;
}

pub trait FifoRunner {
    fn fmt_value(&mut self, string: &str) -> String;
}

pub enum Logger {
    ValueLogger {
        default_value: String,
        interval_ms: u64,
        create_runner: Box<dyn Fn() -> Box<dyn ValueRunner> + Send + Sync>,
    },
    FifoLogger {
        default_value: String,
        fifopath: String,
        create_runner: Box<dyn Fn() -> Box<dyn FifoRunner> + Send + Sync>,
    },
}

pub fn create_loggers() -> Vec<Logger> {
    // logging is done in this same order.
    vec![
        #[cfg(feature = "network")]
        network::create_network_logger(),
        #[cfg(feature = "cpu")]
        cpu::create_cpu_logger(),
        #[cfg(feature = "memory")]
        memory::create_memory_logger(),
        #[cfg(feature = "volume")]
        volume::create_volume_logger(),
        #[cfg(feature = "sessions")]
        sessions::create_session_logger(),
        #[cfg(feature = "battery")]
        battery::create_battery_logger(),
        #[cfg(feature = "date")]
        date::create_date_logger(),
    ]
}
