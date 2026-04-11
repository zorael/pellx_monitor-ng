use crate::config;

#[derive(Debug, Default, Clone)]
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
            self.urls = urls.clone();
        }

        if let Some(show_response) = config.show_response {
            self.show_response = show_response;
        }
    }
}

#[derive(Debug, Default, Clone)]
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
            self.urls = urls.clone();
        }

        if let Some(show_response) = config.show_response {
            self.show_response = show_response;
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandSettings {
    pub strings: super::MessageStrings,
    pub enabled: bool,
    pub commands: Vec<String>,
    pub show_response: bool,
}

impl Default for CommandSettings {
    fn default() -> Self {
        Self {
            strings: super::MessageStrings::default(),
            enabled: true,
            commands: Vec::new(),
            show_response: false,
        }
    }
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
            self.commands = commands.clone();
        }

        if let Some(show_response) = config.show_response {
            self.show_response = show_response;
        }
    }
}

#[derive(Debug, Clone)]
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
}
