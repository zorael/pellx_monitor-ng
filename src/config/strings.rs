/// Configuration file definitions for message strings.
use serde::{Deserialize, Serialize};

use crate::settings;

/// Custom message strings for composing notifications.
///
/// The layout of this struct must mirror that of `settings::MessageStrings`,
/// as it is used to apply configuration from disk to the in-memory settings.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MessageStrings {
    /// Header for alert notifications.
    pub alert_header: Option<String>,

    /// Body for alert notifications.
    pub alert_body: Option<String>,

    /// Header for reminder notifications.
    pub reminder_header: Option<String>,

    /// Body for reminder notifications.
    pub reminder_body: Option<String>,

    /// Header for startup failure notifications.
    pub startup_failed_header: Option<String>,

    /// Body for startup failure notifications.
    pub startup_failed_body: Option<String>,

    /// Header for startup success notifications.
    pub startup_success_header: Option<String>,

    /// Body for startup success notifications.
    pub startup_success_body: Option<String>,

    /// Footer for all notifications, placed at the end of messages.
    pub footer: Option<String>,
}

impl From<settings::MessageStrings> for MessageStrings {
    /// Converts from the in-memory settings struct to this config struct,
    /// done prior to writing it to disk.
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
