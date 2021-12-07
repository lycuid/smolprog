use super::{Block, FifoRunner};
use std::{env, sync::Arc};

struct VolumeRunner;

impl FifoRunner for VolumeRunner {
    fn fmt_value(&mut self, string: String) -> String {
        format!("<ScrlU:Shift=volume 5%+><ScrlD:Shift=volume 5%-><ScrlU=volume 1%+><ScrlD=volume 1%-><BtnL=volume toggle> {}  </BtnL></ScrlD></ScrlU></ScrlD></ScrlU><Box:Left=#171717:2> </Box>", string)
    }
}

pub fn create_volume_blk() -> Block {
    Block::Fifo {
        default_value: "vol: ?",
        fifopath: env::var("XDG_RUNTIME_DIR")
            .ok()
            .and_then(|dir| Some(format!("{}/pipe/volume", dir))),
        create_runner: Arc::new(|| Box::new(VolumeRunner {})),
    }
}
