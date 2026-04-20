//! Settings management.

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

/// Program-wide settings struct, housing all runtime settings.
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Settings {
    /// Settings related to the monitor loop.
    pub monitor: MonitorSettings,

    /// Settings related to the GPIO input source.
    pub gpio: GpioSettings,

    /// Settings related to the dummy input source.
    pub dummy_source: DummyInputSourceSettings,

    /// Settings related to the println notifier backend.
    pub println: PrintlnSettings,

    /// Settings related to the Slack notifier backend.
    pub slack: SlackSettings,

    /// Settings related to the Batsign notifier backend.
    pub batsign: BatsignSettings,

    /// Settings related to the external command notifier backend.
    pub command: CommandSettings,

    /// Whether to disable timestamps in terminal output.
    pub disable_timestamps: bool,

    /// Whether to print some additional information in terminal output.
    pub verbose: bool,

    /// Whether to print much more additional information in terminal output.
    pub debug: bool,

    /// Whether to perform a dry run, echoing what would be done instead of
    /// performing any action.
    pub dry_run: bool,
}

impl Settings {
    /// Applies configuration as read from disk.
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

    /// Applies command-line arguments, overriding any settings they specify.
    pub fn apply_cli(&mut self, cli: &cli::Cli) {
        self.disable_timestamps = cli.disable_timestamps;
        self.verbose = cli.verbose;
        self.debug = cli.debug;
        self.dry_run = cli.dry_run;

        if self.debug {
            self.verbose = true;
        }

        if let Some(source) = &cli.source {
            self.monitor.source = *source;
        }
    }

    /// Performs a sanity check on the settings, returning any errors found.
    ///
    /// # Returns
    /// - `Ok(())` if the settings are sane.
    /// - `Err(Vec<String>)` if the settings contain errors, with a vector of
    ///   strings describing the errors.
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

    /// Saves the current settings to the specified configuration file path.
    ///
    /// # Parameters
    /// - `config_path`: The path to which settings should be saved.
    ///
    /// # Returns
    /// - `Ok(())` if saving succeeded.
    /// - `Err(String)` if saving failed, with a string describing the error.
    pub fn save(&self, config_path: &path::Path) -> Result<(), String> {
        let config = config::Config::from(self);

        match confy::store_path(config_path, config) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

/// Settings related to the monitoring main loop of the program.
#[derive(Debug)]
pub struct MonitorSettings {
    /// The input source to use for monitoring.
    pub source: source::ChoiceOfInputSource,

    /// The interval at which to loop and check the input source for changes.
    pub loop_interval: time::Duration,

    /// How much time has to pass after a startup is detected before the
    /// monitor accepts the startup as successful.
    ///
    /// Any notifications will only be pushed after this time has passed.
    pub startup_window: time::Duration,
}

impl Default for MonitorSettings {
    /// Provides default values for all monitor settings.
    fn default() -> Self {
        Self {
            source: source::ChoiceOfInputSource::Gpio,
            loop_interval: defaults::monitor::LOOP_INTERVAL,
            startup_window: defaults::monitor::STARTUP_WINDOW,
        }
    }
}

impl MonitorSettings {
    /// Applies configuration to the monitor settings, as read from disk.
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

/// Settings related to the GPIO input source.
#[derive(Debug)]
pub struct GpioSettings {
    /// The GPIO pin number to which the burner is connected.
    pub pin: u8,
}

impl GpioSettings {
    /// Applies configuration to the GPIO settings, as read from disk.
    pub fn apply_config(&mut self, config: &config::GpioConfig) {
        if let Some(pin) = config.pin {
            self.pin = pin;
        }
    }
}

impl Default for GpioSettings {
    /// Provides default values for all GPIO settings.
    fn default() -> Self {
        Self {
            pin: defaults::gpio::PIN,
        }
    }
}

/// Settings related to the dummy input source.
#[derive(Debug)]
pub struct DummyInputSourceSettings {
    /// Modulus value to determine the cycle length of the readings.
    pub modulus: u32,

    /// Threshold value to determine the point in the cycle where readings
    /// transition from `Reading::Low` to `Reading::High`.
    pub threshold: u32,
}

impl DummyInputSourceSettings {
    /// Applies configuration to the dummy input source settings, as read from disk.
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
    /// Provides default values for all dummy input source settings.
    fn default() -> Self {
        Self {
            modulus: defaults::dummy::MODULUS,
            threshold: defaults::dummy::THRESHOLD,
        }
    }
}

/// Trims a vector of strings, removing leading and trailing whitespace from
/// each strings, and then filtering out any empty strings.
///
/// # Parameters
/// - `vec`: The slice of strings to trim.
///
/// # Returns
/// A vector of trimmed strings, with empty strings removed.
fn trim_vec_of_strings(vec: &[String]) -> Vec<String> {
    vec.iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
