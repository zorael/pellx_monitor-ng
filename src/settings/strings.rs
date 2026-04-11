use crate::config;

#[derive(Debug)]
pub struct MessageStrings {
    pub alert_header: String,
    pub alert_body: String,
    pub reminder_header: String,
    pub reminder_body: String,
    pub startup_failed_header: String,
    pub startup_failed_body: String,
    pub footer: String,
}

impl Default for MessageStrings {
    fn default() -> Self {
        Self {
            alert_header: "PellX burner failure".to_string(),
            alert_body: "It went into an error state at {time_of_failure}.".to_string(),
            reminder_header: "PellX burner still in failure".to_string(),
            reminder_body: "It has been in an error state since {time_of_failure}.".to_string(),
            startup_failed_header: "PellX burner startup failed".to_string(),
            startup_failed_body: "It tried to start up but failed at {time_of_failure}."
                .to_string(),
            footer: "".to_string(),
        }
    }
}

impl MessageStrings {
    pub fn apply_config(&mut self, config: &config::MessageStrings) {
        if let Some(alert_header) = &config.alert_header {
            self.alert_header = alert_header.clone();
        }

        if let Some(alert_body) = &config.alert_body {
            self.alert_body = alert_body.clone();
        }

        if let Some(reminder_header) = &config.reminder_header {
            self.reminder_header = reminder_header.clone();
        }

        if let Some(reminder_body) = &config.reminder_body {
            self.reminder_body = reminder_body.clone();
        }

        if let Some(startup_failed_header) = &config.startup_failed_header {
            self.startup_failed_header = startup_failed_header.clone();
        }

        if let Some(startup_failed_body) = &config.startup_failed_body {
            self.startup_failed_body = startup_failed_body.clone();
        }

        if let Some(footer) = &config.footer {
            self.footer = footer.clone();
        }
    }
}
