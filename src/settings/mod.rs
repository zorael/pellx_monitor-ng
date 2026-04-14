mod backends;
mod strings;

use std::path;
use std::process;
use std::time;

pub use backends::{BatsignSettings, CommandSettings, PrintlnSettings, SlackSettings};
pub use strings::MessageStrings;

use crate::cli;
use crate::config;
use crate::defaults;
use crate::logging;
use crate::source;

pub struct Settings {
    pub monitor: MonitorSettings,
    pub gpio: GpioSettings,
    pub println: PrintlnSettings,
    pub slack: SlackSettings,
    pub batsign: BatsignSettings,
    pub command: CommandSettings,

    pub config_file: path::PathBuf,
    pub disable_timestamps: bool,
    pub verbose: bool,
    pub debug: bool,
    pub dry_run: bool,
}

impl Settings {
    pub fn apply_config(&mut self, config: Option<&config::Config>) {
        let Some(config) = config else {
            return;
        };

        self.monitor.apply_config(&config.monitor);
        self.gpio.apply_config(&config.gpio);
        self.println.apply_config(&config.println);
        self.slack.apply_config(&config.slack);
        self.batsign.apply_config(&config.batsign);
        self.command.apply_config(&config.command);
    }

    pub fn apply_cli(&mut self, cli: &cli::Cli) {
        self.disable_timestamps = cli.disable_timestamps;
        self.verbose = cli.verbose;
        self.debug = cli.debug;
        self.dry_run = cli.dry_run;

        if let Some(source) = &cli.source {
            self.monitor.source = *source;
        }
    }

    pub fn sanity_check(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Err(e) = self.slack.sanity_check() {
            errors.push(format!("Slack backend: {}", e));
        }

        if let Err(e) = self.batsign.sanity_check() {
            errors.push(format!("Batsign backend: {}", e));
        }

        if let Err(e) = self.command.sanity_check() {
            errors.push(format!("Command backend: {}", e));
        }

        if let Err(e) = self.println.sanity_check() {
            errors.push(format!("Println backend: {}", e));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn save(&self, config_path: &path::Path) -> process::ExitCode {
        let config = config::Config::from(self);

        match confy::store_path(config_path, config) {
            Ok(()) => {
                logging::tsprintln!(
                    self.disable_timestamps,
                    "Config saved successfully to {}",
                    config_path.display()
                );
                process::ExitCode::SUCCESS
            }
            Err(err) => {
                logging::tseprintln!(
                    self.disable_timestamps,
                    "Failed to save configuration to {}: {err}",
                    config_path.display()
                );
                process::ExitCode::FAILURE
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            monitor: MonitorSettings::default(),
            gpio: GpioSettings::default(),
            println: PrintlnSettings::default(),
            slack: SlackSettings::default(),
            batsign: BatsignSettings::default(),
            command: CommandSettings::default(),

            config_file: path::PathBuf::from(defaults::program_metadata::CONFIG_FILENAME),
            disable_timestamps: false,
            verbose: false,
            debug: false,
            dry_run: false,
        }
    }
}

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
            source: source::ChoiceOfInputSource::Gpio,
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

pub struct GpioSettings {
    pub pin: u8,
}

impl GpioSettings {
    pub fn apply_config(&mut self, config: &config::GpioConfig) {
        if let Some(pin) = config.pin {
            self.pin = pin;
        }
    }
}

impl Default for GpioSettings {
    fn default() -> Self {
        Self {
            pin: defaults::gpio::PIN,
        }
    }
}

fn trim_vec_of_strings(vec: &[String]) -> Vec<String> {
    vec.iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
