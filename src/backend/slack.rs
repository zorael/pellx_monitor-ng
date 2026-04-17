//! Slack backend implementation.

use std::time;

use crate::context;
use crate::message;
use crate::notify;
use crate::settings;

/// Backend implementation for sending notifications to Slack via webhooks.
pub struct SlackBackend {
    /// Unique numeric identifier for this backend instance.
    pub id: usize,

    /// Name of the backend, used for display purposes.
    pub name: String,

    /// HTTP client agent used to send requests to the Slack API.
    pub agent: ureq::Agent,

    /// URL of the Slack webhook to send notifications to.
    pub url: String,

    /// Whether to show the HTTP response from the Slack API in terminal output.
    pub show_response: bool,

    /// Custom message strings, used to compose notifications.
    pub strings: settings::MessageStrings,
}

impl SlackBackend {
    /// Creates a new instance of the Slack backend.
    ///
    /// # Parameters
    /// - `id`: Unique numeric identifier for this backend instance.
    /// - `agent`: HTTP client agent used to send requests to the Slack API.
    /// - `url`: URL of the Slack webhook to send notifications to.
    /// - `show_response`: Whether to show the HTTP response from the Slack
    ///   API in terminal output.
    /// - `strings`: Custom message strings, used to compose notifications.
    pub fn new(
        id: usize,
        agent: ureq::Agent,
        url: &str,
        show_response: bool,
        strings: settings::MessageStrings,
    ) -> Self {
        let name = format!("slack-{id}");

        Self {
            id,
            name,
            agent,
            url: url.to_string(),
            show_response,
            strings,
        }
    }

    /// Helper to deduplicate the dispatch of message composition logic.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `message_type`: Type of the message being composed.
    ///
    /// # Returns
    /// A composed message body.
    fn compose_common(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        match message_type {
            notify::MessageType::Alert => message::compose_alert_message(ctx, &self.strings),
            notify::MessageType::Reminder => message::compose_reminder_message(ctx, &self.strings),
            notify::MessageType::StartupFailed => {
                message::compose_startup_failed_message(ctx, &self.strings)
            }
            notify::MessageType::StartupSuccess => {
                message::compose_startup_success_message(ctx, &self.strings)
            }
        }
    }
}

impl super::Backend for SlackBackend {
    /// Returns the unique numeric identifier of this backend instance.
    fn id(&self) -> usize {
        self.id
    }

    /// Returns the name of this backend instance.
    fn name(&self) -> &str {
        &self.name
    }

    /// Returns the message strings associated with this backend instance.
    fn strings(&self) -> &settings::MessageStrings {
        &self.strings
    }

    /// Composes a message for this backend instance.
    ///
    /// The message is composed as a JSON string, which is not very humanly
    /// readable. For a pretty-print version, see `compose_display`.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `message_type`: Type of the message being composed.
    ///
    /// # Returns
    /// A JSON string containing the composed message.
    fn compose(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        let body = self.compose_common(ctx, message_type);
        serde_json::json!({ "text": body }).to_string()
    }

    /// Composes a message for display purposes for this backend instance.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `message_type`: Type of the message being composed.
    ///
    /// # Returns
    /// A pretty-printed JSON string containing the composed message.
    fn compose_display(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        let body = self.compose_common(ctx, message_type);
        let value = serde_json::json!({ "text": body });

        match serde_json::to_string_pretty(&value) {
            Ok(pretty) => pretty,
            Err(_) => value.to_string(),
        }
    }

    /// Returns the stagger delay for this backend instance.
    ///
    /// For Slack, this is currently a hardcoded duration of 300 milliseconds.
    fn stagger_delay(&self) -> time::Duration {
        time::Duration::from_millis(300)
    }

    /// Emits a notification via the Slack backend.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `body`: The JSON string containing the composed message.
    /// - `message_type`: Type of the message being emitted.
    ///
    /// # Returns
    /// - `Ok(Some(String))` if the notification was sent successfully and the
    ///   response should be shown.
    /// - `Ok(None)` if the notification was sent successfully but the response
    ///   should not be shown.
    /// - `Err(String)` if there was an error sending the notification, with
    ///   the error message as a string.
    fn emit(
        &self,
        _ctx: &context::Context,
        body: &str,
        _message_type: notify::MessageType,
    ) -> Result<Option<String>, String> {
        let json: serde_json::Value = serde_json::from_str(body).map_err(|e| e.to_string())?;

        match self.agent.post(&self.url).send_json(json) {
            Ok(mut r) => match r.body_mut().read_to_string() {
                Ok(output) => {
                    if self.show_response {
                        Ok(Some(output))
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
