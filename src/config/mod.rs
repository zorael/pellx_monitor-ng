use serde::{Deserialize, Serialize};
use std::path;
use std::time;

use crate::source;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub monitor: MonitorConfig,
    pub gpio: GpioConfig,
    pub println: PrintlnConfig,
    pub slack: SlackConfig,
    pub batsign: BatsignConfig,
    pub command: CommandConfig,
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
    pub source: source::ChoiceOfInputSource,

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
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub pin: Option<u8>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintlnConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SlackConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub urls: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BatsignConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub urls: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub commands: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MessageStrings {
    pub alert_header: Option<String>,
    pub alert_body: Option<String>,
    pub reminder_header: Option<String>,
    pub reminder_body: Option<String>,
    pub startup_failed_header: Option<String>,
    pub startup_failed_body: Option<String>,
    pub footer: Option<String>,
}
