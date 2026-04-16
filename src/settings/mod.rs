mod backends;
mod strings;

pub use backends::{BatsignSettings, CommandSettings, PrintlnSettings, SlackSettings};
pub use strings::MessageStrings;

use std::path;
use std::time;

use crate::cli;
use crate::config;
use crate::defaults;
use crate::source;

#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Settings {
    pub monitor: MonitorSettings,
    pub gpio: GpioSettings,
    pub dummy_source: DummyInputSourceSettings,
    pub println: PrintlnSettings,
    pub slack: SlackSettings,
    pub batsign: BatsignSettings,
    pub command: CommandSettings,

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
        self.dummy_source.apply_config(&config.dummy_input);
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
            errors.push(format!("Slack backend: {e}"));
        }

        if let Err(e) = self.batsign.sanity_check() {
            errors.push(format!("Batsign backend: {e}"));
        }

        if let Err(e) = self.command.sanity_check() {
            errors.push(format!("Command backend: {e}"));
        }

        if let Err(e) = self.println.sanity_check() {
            errors.push(format!("Println backend: {e}"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn save(&self, config_path: &path::Path) -> Result<(), String> {
        let config = config::Config::from(self);

        match confy::store_path(config_path, config) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub struct MonitorSettings {
    pub source: source::ChoiceOfInputSource,
    pub loop_interval: time::Duration,
    pub startup_window: time::Duration,
}

impl Default for MonitorSettings {
    fn default() -> Self {
        Self {
            source: source::ChoiceOfInputSource::Gpio,
            loop_interval: defaults::monitor::LOOP_INTERVAL,
            startup_window: defaults::monitor::STARTUP_WINDOW,
        }
    }
}

impl MonitorSettings {
    pub fn apply_config(&mut self, config: &config::MonitorConfig) {
        if let Some(loop_interval) = config.loop_interval {
            self.loop_interval = loop_interval;
        }

        if let Some(startup_window) = config.startup_window {
            self.startup_window = startup_window;
        }

        self.source = config.source;
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

pub struct DummyInputSourceSettings {
    pub modulus: u32,
    pub threshold: u32,
}

impl DummyInputSourceSettings {
    pub fn apply_config(&mut self, config: &config::DummyInputSourceConfig) {
        if let Some(modulus) = config.modulus {
            self.modulus = modulus;
        }

        if let Some(divisor) = config.threshold {
            self.threshold = divisor;
        }
    }
}

impl Default for DummyInputSourceSettings {
    fn default() -> Self {
        Self {
            modulus: defaults::dummy::MODULUS,
            threshold: defaults::dummy::THRESHOLD,
        }
    }
}

fn trim_vec_of_strings(vec: &[String]) -> Vec<String> {
    vec.iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
