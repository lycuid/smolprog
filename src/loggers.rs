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
    fn get_value(&mut self) -> String;
}

pub trait FifoRunner {
    fn fmt_value(&mut self, string: &str) -> String;
}

pub enum Logger {
    /// Used for interval based logging (log on every interval).
    ValueLogger {
        interval_ms: u64,
        runner: Box<dyn ValueRunner + Send + Sync>,
    },
    /// Used for logging from 'fifo' file (log as soon as content updates).
    FifoLogger {
        default_value: String,
        fifopath: String,
        runner: Box<dyn FifoRunner + Send + Sync>,
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
