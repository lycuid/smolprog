mod battery;
mod cpu;
mod date;
mod memory;
mod network;
mod tmuxls;
mod volume;

use std::sync::Arc;

pub trait ValueRunner {
    fn get_value(&mut self) -> Option<String>;
    fn fmt_value(&mut self, string: String) -> String;
}

pub trait FifoRunner {
    fn fmt_value(&mut self, string: String) -> String;
}

pub enum Block {
    Value {
        default_value: &'static str,
        interval_ms: u64,
        create_runner: Arc<dyn Fn() -> Box<dyn ValueRunner> + Send + Sync>,
    },
    Fifo {
        default_value: &'static str,
        fifopath: Option<String>,
        create_runner: Arc<dyn Fn() -> Box<dyn FifoRunner> + Send + Sync>,
    },
}

pub fn createblks() -> Vec<Block> {
    // renders in the same order.
    vec![
        network::create_network_blk(),
        cpu::create_cpu_blk(),
        memory::create_memory_blk(),
        volume::create_volume_blk(),
        tmuxls::create_tmuxls_blk(),
        battery::create_battery_blk(),
        date::create_date_blk(),
    ]
}
