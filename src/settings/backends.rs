//! Settings related to the various backends that can be used to send notifications.

use crate::config;

/// Settings related to the Slack backend.
#[derive(Default)]
pub struct SlackSettings {
    /// Custom message strings, used to compose notifications.
    pub strings: super::MessageStrings,

    /// Whether the Slack backend is enabled.
    pub enabled: bool,

    /// Webhook URLs to which notifications should be sent.
    pub urls: Vec<String>,

    /// Whether to show the HTTP response from the Slack API in terminal output.
    pub show_response: bool,
}

impl SlackSettings {
    /// Applies configuration to the Slack settings, as read from disk.
    pub fn apply_config(&mut self, config: &config::SlackConfig) {
        if let Some(strings) = &config.strings {
            self.strings.apply_config(strings);
        }

        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }

        if let Some(urls) = &config.urls {
            self.urls = super::trim_vec_of_strings(urls);
        }

        if let Some(show_response) = config.show_response {
            self.show_response = show_response;
        }
    }

    /// Performs a sanity check on the Slack settings, returning an error if
    /// any are found.
    ///
    /// # Returns
    /// - `Ok(())` if the settings are sane.
    /// - `Err(String)` if the settings contain an error, with a string
    ///   describing the error.
    pub fn sanity_check(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.urls.is_empty() {
            Err("URL must not be empty".to_string())
        } else {
            Ok(())
        }
    }
}

/// Settings related to the Batsign backend.
#[derive(Default)]
pub struct BatsignSettings {
    /// Custom message strings, used to compose notifications.
    pub strings: super::MessageStrings,

    /// Whether the Batsign backend is enabled.
    pub enabled: bool,

    /// URLs to which notifications should be sent.
    pub urls: Vec<String>,

    /// Whether to show the HTTP response from the Batsign API in terminal output.
    pub show_response: bool,
}

impl BatsignSettings {
    /// Applies configuration to the Batsign settings, as read from disk.
    pub fn apply_config(&mut self, config: &config::BatsignConfig) {
        if let Some(strings) = &config.strings {
            self.strings.apply_config(strings);
        }

        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }

        if let Some(urls) = &config.urls {
            self.urls = super::trim_vec_of_strings(urls);
        }

        if let Some(show_response) = config.show_response {
            self.show_response = show_response;
        }
    }

    /// Performs a sanity check on the Batsign settings, returning an error if
    /// any are found.
    ///
    /// # Returns
    /// - `Ok(())` if the settings are sane.
    /// - `Err(String)` if the settings contain an error, with a string
    ///   describing the error.
    pub fn sanity_check(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.urls.is_empty() {
            Err("URL must not be empty".to_string())
        } else {
            Ok(())
        }
    }
}

/// Settings related to the external command backend.
#[derive(Default)]
pub struct CommandSettings {
    /// Custom message strings, used to compose notifications.
    pub strings: super::MessageStrings,

    /// Whether the external command backend is enabled.
    pub enabled: bool,

    /// Commands to execute when sending notifications.
    pub commands: Vec<String>,

    /// Whether to show the output from the executed commands in terminal output.
    pub show_response: bool,
}

impl CommandSettings {
    /// Applies configuration to the external command settings, as read from disk.
    pub fn apply_config(&mut self, config: &config::CommandConfig) {
        if let Some(strings) = &config.strings {
            self.strings.apply_config(strings);
        }

        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }

        if let Some(commands) = &config.commands {
            self.commands = super::trim_vec_of_strings(commands);
        }

        if let Some(show_response) = config.show_response {
            self.show_response = show_response;
        }
    }

    /// Performs a sanity check on the external command settings, returning
    /// an error if any are found.
    ///
    /// # Returns
    /// - `Ok(())` if the settings are sane.
    /// - `Err(String)` if the settings contain an error, with a string
    ///   describing the error.
    pub fn sanity_check(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.commands.is_empty() {
            Err("Commands must not be empty".to_string())
        } else {
            Ok(())
        }
    }
}

/// Settings related to the println backend.
pub struct PrintlnSettings {
    /// Custom message strings, used to compose notifications.
    pub strings: super::MessageStrings,

    /// Whether the println backend is enabled.
    pub enabled: bool,
}

impl Default for PrintlnSettings {
    /// Provides default values for all println settings.
    fn default() -> Self {
        Self {
            strings: super::MessageStrings::default(),
            enabled: true,
        }
    }
}

impl PrintlnSettings {
    /// Applies configuration to the println settings, as read from disk.
    pub fn apply_config(&mut self, config: &config::PrintlnConfig) {
        if let Some(strings) = &config.strings {
            self.strings.apply_config(strings);
        }

        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }
    }

    /// Performs a sanity check on the println settings, returning an error if any
    /// are found.
    ///
    /// In the case of the println backend, there are no settings, so this
    /// simply returns `Ok(())`.
    #[allow(clippy::unused_self)]
    #[allow(clippy::unnecessary_wraps)]
    pub fn sanity_check(&self) -> Result<(), String> {
        Ok(())
    }
}
