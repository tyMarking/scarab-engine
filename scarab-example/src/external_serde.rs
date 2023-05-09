use piston::EventSettings;
use serde::{Deserialize, Serialize};

/// A structure mirroring Piston's [EventSettings] for use with serde
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(remote = "EventSettings")]
pub struct EventSettingsDef {
    pub max_fps: u64,
    pub ups: u64,
    pub ups_reset: u64,
    pub swap_buffers: bool,
    pub bench_mode: bool,
    pub lazy: bool,
}
