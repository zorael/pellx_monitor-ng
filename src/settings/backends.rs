use crate::config;

#[derive(Default)]
pub struct SlackSettings {
    pub strings: super::MessageStrings,
    pub enabled: bool,
    pub urls: Vec<String>,
    pub show_response: bool,
}

impl SlackSettings {
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

#[derive(Default)]
pub struct BatsignSettings {
    pub strings: super::MessageStrings,
    pub enabled: bool,
    pub urls: Vec<String>,
    pub show_response: bool,
}

impl BatsignSettings {
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

#[derive(Default)]
pub struct CommandSettings {
    pub strings: super::MessageStrings,
    pub enabled: bool,
    pub commands: Vec<String>,
    pub show_response: bool,
}

impl CommandSettings {
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

pub struct PrintlnSettings {
    pub strings: super::MessageStrings,
    pub enabled: bool,
}

impl Default for PrintlnSettings {
    fn default() -> Self {
        Self {
            strings: super::MessageStrings::default(),
            enabled: true,
        }
    }
}

impl PrintlnSettings {
    pub fn apply_config(&mut self, config: &config::PrintlnConfig) {
        if let Some(strings) = &config.strings {
            self.strings.apply_config(strings);
        }

        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }
    }

    pub fn sanity_check(&self) -> Result<(), String> {
        Ok(())
    }
}
