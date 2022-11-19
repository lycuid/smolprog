//! Logs network usage.
use super::{Logger, ValueRunner};
use std::{
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
};

struct NetworkRunner {
    previous_interface: String,
    previous_rx: f32,
    previous_tx: f32,
}

impl NetworkRunner {
    fn fmt_value(string: String) -> String {
        format!("<BtnL=xdotool key super+ctrl+n> {}  </BtnL><Box:Left=#171717:2> </Box>", string)
    }

    fn get_active_interface() -> Option<String> {
        fs::read_dir("/sys/class/net/")
            .ok()?
            .map(|entry| entry.unwrap().path())
            .filter(|dirpath| {
                if let Ok(mut file) = File::open(dirpath.join("operstate")) {
                    let mut buffer = String::new();
                    if let Ok(_) = file.read_to_string(&mut buffer) {
                        if buffer.trim_end() == "up" {
                            return true;
                        }
                    }
                }
                false
            })
            .map(|dir| dir.file_name().unwrap().to_str().unwrap().to_string())
            .next()
    }

    fn get_network_bytes(interface: &String) -> Option<(f32, f32)> {
        let stats = PathBuf::from("/sys/class/net/")
            .join(interface)
            .join("statistics");
        let (rxfile, txfile) = (stats.join("rx_bytes"), stats.join("tx_bytes"));

        Some((
            fs::read_to_string(rxfile).ok()?.trim_end().parse().ok()?,
            fs::read_to_string(txfile).ok()?.trim_end().parse().ok()?,
        ))
    }
}

impl ValueRunner for NetworkRunner {
    fn get_value(&mut self) -> Option<String> {
        let interface = Self::get_active_interface()?;
        let (new_rx, new_tx) = Self::get_network_bytes(&interface)?;

        if self.previous_interface != interface {
            self.previous_interface = interface.clone();
            self.previous_rx = new_rx;
            self.previous_tx = new_tx;
        }

        let rx = (new_rx - self.previous_rx) / 1024.;
        let tx = (new_tx - self.previous_tx) / 1024.;

        self.previous_rx = new_rx;
        self.previous_tx = new_tx;

        Some(Self::fmt_value(format!(
            "{}:  {:.2} KiB/s  {:.2} KiB/s",
            interface, rx, tx
        )))
    }
}

pub fn create_network_logger() -> Logger {
    Logger::ValueLogger {
        default_value: NetworkRunner::fmt_value("net: ?".into()),
        interval_ms: 1000,
        create_runner: Box::new(|| {
            let previous_interface =
                NetworkRunner::get_active_interface().unwrap();
            let (previous_rx, previous_tx) =
                match NetworkRunner::get_network_bytes(&previous_interface) {
                    Some(bytes) => bytes,
                    None => (0., 0.),
                };

            Box::new(NetworkRunner {
                previous_interface,
                previous_rx,
                previous_tx,
            })
        }),
    }
}
