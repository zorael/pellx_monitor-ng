//! User-configurable strings used in composing notifications.

use crate::config;

/// Configurable strings used in composing notifications.
///
/// These have default values but can be overridden in the configuration file.
///
/// A message has a header and a body, with an optional trailing footer.
#[derive(Clone)]
pub struct MessageStrings {
    /// Header for alert notifications.
    pub alert_header: String,

    /// Body for alert notifications.
    pub alert_body: String,

    /// Header for reminder notifications.
    pub reminder_header: String,

    /// Body for reminder notifications.
    pub reminder_body: String,

    /// Header for startup failure notifications.
    pub startup_failed_header: String,

    /// Body for startup failure notifications.
    pub startup_failed_body: String,

    /// Header for startup success notifications.
    pub startup_success_header: String,

    /// Body for startup success notifications.
    pub startup_success_body: String,

    /// Footer for all notifications, placed at the end of messages.
    pub footer: String,
}

impl Default for MessageStrings {
    /// Provides default values for all message strings.
    fn default() -> Self {
        Self {
            alert_header: "PellX burner failure\\n".to_string(),
            alert_body: "It went into an error state at {fuzzy_high}.".to_string(),
            reminder_header: "PellX burner still in failure\\n".to_string(),
            reminder_body: "It has been in an error state since {fuzzy_high}.".to_string(),
            startup_failed_header: "PellX burner startup failed".to_string(),
            startup_failed_body: "It tried to start up but failed at {fuzzy_high}.".to_string(),
            startup_success_header: "PellX burner startup succeeded".to_string(),
            startup_success_body: "It successfully started up at {fuzzy_low}.".to_string(),
            footer: String::new(),
        }
    }
}

impl MessageStrings {
    /// Applies the provided configuration as read from disk to this message
    /// strings struct, modifying it in-place.
    pub fn apply_config(&mut self, config: &config::MessageStrings) {
        if let Some(alert_header) = &config.alert_header {
            self.alert_header.clone_from(alert_header);
        }

        if let Some(alert_body) = &config.alert_body {
            self.alert_body.clone_from(alert_body);
        }

        if let Some(reminder_header) = &config.reminder_header {
            self.reminder_header.clone_from(reminder_header);
        }

        if let Some(reminder_body) = &config.reminder_body {
            self.reminder_body.clone_from(reminder_body);
        }

        if let Some(startup_failed_header) = &config.startup_failed_header {
            self.startup_failed_header.clone_from(startup_failed_header);
        }

        if let Some(startup_failed_body) = &config.startup_failed_body {
            self.startup_failed_body.clone_from(startup_failed_body);
        }

        if let Some(startup_success_header) = &config.startup_success_header {
            self.startup_success_header
                .clone_from(startup_success_header);
        }

        if let Some(startup_success_body) = &config.startup_success_body {
            self.startup_success_body.clone_from(startup_success_body);
        }

        if let Some(footer) = &config.footer {
            self.footer.clone_from(footer);
        }
    }
}
