//! Configuration file definitions.

mod backends;
mod strings;

pub use backends::{BatsignConfig, CommandConfig, PrintlnConfig, SlackConfig};
pub use strings::MessageStrings;

use std::time;

use crate::settings;
use crate::source;

/// Configuration file structure.
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// Configuration related to the monitor loop.
    pub monitor: MonitorConfig,

    /// Configuration related to the GPIO input source.
    pub gpio: GpioConfig,

    /// Configuration related to the dummy input source.
    pub dummy_input: DummyInputSourceConfig,

    /// Configuration related to the Slack notifier backend.
    pub slack: SlackConfig,

    /// Configuration related to the Batsign notifier backend.
    pub batsign: BatsignConfig,

    /// Configuration related to the external command notifier backend.
    pub command: CommandConfig,

    /// Configuration related to the println notifier backend.
    pub println: PrintlnConfig,
}

impl From<&settings::Settings> for Config {
    /// Converts from the in-memory settings struct to this config struct, done
    /// prior to writing it to disk.
    fn from(settings: &settings::Settings) -> Self {
        Self {
            monitor: MonitorConfig {
                source: settings.monitor.source,
                loop_interval: Some(*settings.monitor.loop_interval),
                startup_window: Some(*settings.monitor.startup_window),
            },
            gpio: GpioConfig {
                pin: Some(settings.gpio.pin),
            },
            dummy_input: DummyInputSourceConfig {
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

/// Configuration related to the monitoring main loop of the program.
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MonitorConfig {
    /// The input source to use for monitoring.
    pub source: source::ChoiceOfInputSource,

    /// The interval at which to loop and check the input source for changes.
    #[serde(with = "humantime_serde")]
    pub loop_interval: Option<time::Duration>,

    /// How much time has to pass after a startup is detected before the
    /// monitor accepts the startup as successful.
    ///
    /// Any notifications will only be pushed after this time has passed.
    #[serde(with = "humantime_serde")]
    pub startup_window: Option<time::Duration>,
}

/// Configuration related to the GPIO input source.
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GpioConfig {
    /// The GPIO pin number to which the burner is connected.
    pub pin: Option<u8>,
}

/// Configuration related to the dummy input source.
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DummyInputSourceConfig {
    /// Modulus value to determine the cycle length of the readings.
    pub modulus: Option<u32>,

    /// Threshold value to determine the point in the cycle where readings
    /// transition from `Reading::Low` to `Reading::High`.
    pub threshold: Option<u32>,
}
