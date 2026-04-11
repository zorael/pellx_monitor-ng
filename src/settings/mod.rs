mod backends;
mod strings;

use std::time;

pub use backends::{BatsignSettings, CommandSettings, PrintlnSettings, SlackSettings};
pub use strings::MessageStrings;

use crate::cli;
use crate::config;
use crate::defaults;
use crate::source;

#[derive(Debug, Default)]
pub struct Settings {
    pub monitor: MonitorSettings,
    pub gpio: GpioSettings,
    pub println: PrintlnSettings,
    pub slack: SlackSettings,
    pub batsign: BatsignSettings,
    pub command: CommandSettings,

    pub verbose: bool,
    pub debug: bool,
    pub dry_run: bool,
}

impl Settings {
    pub fn apply_config(&mut self, config: &config::Config) {
        self.monitor.apply_config(&config.monitor);
        self.gpio.apply_config(&config.gpio);
        self.println.apply_config(&config.println);
        self.slack.apply_config(&config.slack);
        self.batsign.apply_config(&config.batsign);
        self.command.apply_config(&config.command);
    }

    pub fn apply_cli(&mut self, cli: &cli::Cli) {
        self.verbose = cli.verbose;
        self.debug = cli.debug;
        self.dry_run = cli.dry_run;

        if let Some(source) = &cli.source {
            self.monitor.source = *source;
        }
    }
}

#[derive(Debug)]
pub struct MonitorSettings {
    pub source: source::ChoiceOfInputSource,
    pub loop_interval: time::Duration,
    pub max_allowed_startup_time: time::Duration,
    pub notification_retry_interval: time::Duration,
    pub reminder_interval: time::Duration,
}

impl Default for MonitorSettings {
    fn default() -> Self {
        Self {
            source: source::ChoiceOfInputSource::Dummy,
            loop_interval: defaults::monitor::LOOP_INTERVAL,
            max_allowed_startup_time: defaults::monitor::MAX_ALLOWED_STARTUP_TIME,
            notification_retry_interval: defaults::monitor::NOTIFICATION_RETRY_INTERVAL,
            reminder_interval: defaults::monitor::REMINDER_INTERVAL,
        }
    }
}

impl MonitorSettings {
    pub fn apply_config(&mut self, config: &config::MonitorConfig) {
        if let Some(loop_interval) = config.loop_interval {
            self.loop_interval = loop_interval;
        }

        if let Some(max_allowed_startup_time) = config.max_allowed_startup_time {
            self.max_allowed_startup_time = max_allowed_startup_time;
        }

        if let Some(notification_retry_interval) = config.notification_retry_interval {
            self.notification_retry_interval = notification_retry_interval;
        }

        if let Some(reminder_interval) = config.reminder_interval {
            self.reminder_interval = reminder_interval;
        }
    }
}

#[derive(Debug)]
pub struct GpioSettings {
    pub strings: strings::MessageStrings,
    pub enabled: bool,
    pub pin: u8,
}

impl GpioSettings {
    pub fn apply_config(&mut self, config: &config::GpioConfig) {
        if let Some(strings) = &config.strings {
            self.strings.apply_config(strings);
        }

        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }

        if let Some(pin) = config.pin {
            self.pin = pin;
        }
    }
}

impl Default for GpioSettings {
    fn default() -> Self {
        Self {
            strings: strings::MessageStrings::default(),
            enabled: true,
            pin: defaults::gpio::PIN,
        }
    }
}
