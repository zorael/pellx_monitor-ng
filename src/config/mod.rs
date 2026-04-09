use serde::{Deserialize, Serialize};
use std::path;
use std::time;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub monitor: MonitorConfig,
    pub gpio: GpioConfig,
    pub println: PrintlnConfig,
}

impl Config {
    pub fn load(path: &path::Path) -> Result<Option<Self>, confy::ConfyError> {
        if !path.exists() {
            return Ok(None);
        }

        confy::load_path(path).map(Some)
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonitorConfig {
    #[serde(with = "humantime_serde")]
    pub loop_interval: Option<time::Duration>,

    #[serde(with = "humantime_serde")]
    pub max_allowed_startup_time: Option<time::Duration>,

    #[serde(with = "humantime_serde")]
    pub notification_retry_interval: Option<time::Duration>,

    #[serde(with = "humantime_serde")]
    pub reminder_interval: Option<time::Duration>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GpioConfig {
    pub enabled: Option<bool>,
    pub pin: Option<u8>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintlnConfig {
    pub enabled: Option<bool>,
}
