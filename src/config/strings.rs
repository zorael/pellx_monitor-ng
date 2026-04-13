use serde::{Deserialize, Serialize};

use crate::settings;

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MessageStrings {
    pub alert_header: Option<String>,
    pub alert_body: Option<String>,
    pub reminder_header: Option<String>,
    pub reminder_body: Option<String>,
    pub startup_failed_header: Option<String>,
    pub startup_failed_body: Option<String>,
    pub startup_success_header: Option<String>,
    pub startup_success_body: Option<String>,
    pub footer: Option<String>,
}

impl From<settings::MessageStrings> for MessageStrings {
    fn from(settings: settings::MessageStrings) -> Self {
        Self {
            alert_header: Some(settings.alert_header),
            alert_body: Some(settings.alert_body),
            reminder_header: Some(settings.reminder_header),
            reminder_body: Some(settings.reminder_body),
            startup_failed_header: Some(settings.startup_failed_header),
            startup_failed_body: Some(settings.startup_failed_body),
            startup_success_header: Some(settings.startup_success_header),
            startup_success_body: Some(settings.startup_success_body),
            footer: Some(settings.footer),
        }
    }
}
