//! Logs volume change.
use super::{FifoRunner, Logger};
use std::env;

struct VolumeRunner;

impl FifoRunner for VolumeRunner {
    fn fmt_value(&mut self, string: &str) -> String {
        format!(
            "<ScrlU:Shift=volume 5%+><ScrlD:Shift=volume 5%-><ScrlU=volume 1%+><ScrlD=volume 1%-><BtnL=volume toggle> {}  </BtnL></ScrlD></ScrlU></ScrlD></ScrlU><Box:Left=#171717:2> </Box>",
            string
        )
    }
}

pub fn create_volume_logger() -> Logger {
    Logger::FifoLogger {
        default_value: "vol: ?".into(),
        fifopath: env::var("XDG_RUNTIME_DIR")
            .map(|dir| format!("{}/pipe/volume", dir))
            .unwrap(),
        runner: Box::new(VolumeRunner {}),
    }
}
