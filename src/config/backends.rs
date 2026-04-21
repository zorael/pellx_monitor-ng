//! Backend configuration structures.

use serde::{Deserialize, Serialize};

/// Configuration related to the println notifier backend.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintlnConfig {
    /// Custom message strings, used to compose notifications.
    pub strings: Option<super::MessageStrings>,

    /// Whether the println backend is enabled.
    pub enabled: Option<bool>,
}

/// Configuration related to the Slack notifier backend.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SlackConfig {
    /// Custom message strings, used to compose notifications.
    pub strings: Option<super::MessageStrings>,

    /// Whether the Slack backend is enabled.
    pub enabled: Option<bool>,

    /// Webhook URLs to which the Slack backend will send notifications.
    pub urls: Option<Vec<String>>,

    /// Whether to show the HTTP response from the Slack API in terminal output.
    pub show_response: Option<bool>,
}

/// Configuration related to the Batsign notifier backend.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BatsignConfig {
    /// Custom message strings, used to compose notifications.
    pub strings: Option<super::MessageStrings>,

    /// Whether the Batsign backend is enabled.
    pub enabled: Option<bool>,

    /// URLs to which the Batsign backend will send notifications.
    pub urls: Option<Vec<String>>,

    /// Whether to show the HTTP response from the Batsign API in terminal output.
    pub show_response: Option<bool>,
}

/// Configuration related to the external command notifier backend.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    /// Custom message strings, used to compose notifications.
    pub strings: Option<super::MessageStrings>,

    /// Whether the external command backend is enabled.
    pub enabled: Option<bool>,

    /// Commands to execute for each notification, with placeholders for
    /// message strings.
    pub commands: Option<Vec<String>>,

    /// Whether to show the output from the executed commands in terminal output.
    pub show_response: Option<bool>,
}
