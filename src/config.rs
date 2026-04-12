use serde::{Deserialize, Serialize};
use std::time;

use crate::settings;
use crate::source;

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub monitor: MonitorConfig,
    pub gpio: GpioConfig,
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
                max_allowed_startup_time: Some(settings.monitor.max_allowed_startup_time),
                notification_retry_interval: Some(settings.monitor.notification_retry_interval),
                reminder_interval: Some(settings.monitor.reminder_interval),
            },
            gpio: GpioConfig {
                pin: Some(settings.gpio.pin),
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

#[derive(Default, Clone, Serialize, Deserialize)]
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

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GpioConfig {
    pub pin: Option<u8>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintlnConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SlackConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub urls: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BatsignConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub urls: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    pub strings: Option<MessageStrings>,
    pub enabled: Option<bool>,
    pub commands: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MessageStrings {
    pub alert_header: Option<String>,
    pub alert_body: Option<String>,
    pub reminder_header: Option<String>,
    pub reminder_body: Option<String>,
    pub startup_failed_header: Option<String>,
    pub startup_failed_body: Option<String>,
    pub startup_success_header: Option<String>,
    pub startup_success_body: Option<String>,
    pub footer: Option<String>,
}

impl From<settings::MessageStrings> for MessageStrings {
    fn from(settings: settings::MessageStrings) -> Self {
        Self {
            alert_header: Some(settings.alert_header),
            alert_body: Some(settings.alert_body),
            reminder_header: Some(settings.reminder_header),
            reminder_body: Some(settings.reminder_body),
            startup_failed_header: Some(settings.startup_failed_header),
            startup_failed_body: Some(settings.startup_failed_body),
            startup_success_header: Some(settings.startup_success_header),
            startup_success_body: Some(settings.startup_success_body),
            footer: Some(settings.footer),
        }
    }
}
