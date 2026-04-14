mod backends;
mod strings;

pub use backends::{BatsignConfig, CommandConfig, PrintlnConfig, SlackConfig};
pub use strings::MessageStrings;

use serde::{Deserialize, Serialize};
use std::time;

use crate::settings;
use crate::source;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub monitor: MonitorConfig,
    pub gpio: GpioConfig,
    pub dummy_input: DumyInputSourceConfig,
    pub slack: SlackConfig,
    pub batsign: BatsignConfig,
    pub command: CommandConfig,
    pub println: PrintlnConfig,
}

impl From<&settings::Settings> for Config {
    fn from(settings: &settings::Settings) -> Self {
        Self {
            monitor: MonitorConfig {
                source: settings.monitor.source,
                loop_interval: Some(settings.monitor.loop_interval),
                required_time_for_startup: Some(settings.monitor.required_time_for_startup),
            },
            gpio: GpioConfig {
                pin: Some(settings.gpio.pin),
            },
            dummy_input: DumyInputSourceConfig {
                modulus: Some(settings.dummy_source.modulus),
                threshold: Some(settings.dummy_source.threshold),
            },
            slack: SlackConfig {
                strings: Some(settings.slack.strings.clone().into()),
                enabled: Some(settings.slack.enabled),
                urls: Some(settings.slack.urls.clone()),
                show_response: Some(settings.slack.show_response),
            },
            batsign: BatsignConfig {
                strings: Some(settings.batsign.strings.clone().into()),
                enabled: Some(settings.batsign.enabled),
                urls: Some(settings.batsign.urls.clone()),
                show_response: Some(settings.batsign.show_response),
            },
            command: CommandConfig {
                strings: Some(settings.command.strings.clone().into()),
                enabled: Some(settings.command.enabled),
                commands: Some(settings.command.commands.clone()),
                show_response: Some(settings.command.show_response),
            },
            println: PrintlnConfig {
                strings: Some(settings.println.strings.clone().into()),
                enabled: Some(settings.println.enabled),
            },
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonitorConfig {
    pub source: source::ChoiceOfInputSource,

    #[serde(with = "humantime_serde")]
    pub loop_interval: Option<time::Duration>,

    #[serde(with = "humantime_serde")]
    pub required_time_for_startup: Option<time::Duration>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GpioConfig {
    pub pin: Option<u8>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DumyInputSourceConfig {
    pub modulus: Option<u32>,
    pub threshold: Option<u32>,
}
