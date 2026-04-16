use std::time;

use crate::context;
use crate::message;
use crate::notify;
use crate::settings;

pub struct SlackBackend {
    pub id: usize,
    pub name: String,
    pub agent: ureq::Agent,
    pub url: String,
    pub show_response: bool,
    pub strings: settings::MessageStrings,
}

impl SlackBackend {
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
    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn strings(&self) -> &settings::MessageStrings {
        &self.strings
    }

    fn compose(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        let body = self.compose_common(ctx, message_type);
        serde_json::json!({ "text": body }).to_string()
    }

    fn compose_display(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        let body = self.compose_common(ctx, message_type);
        let value = serde_json::json!({ "text": body });

        match serde_json::to_string_pretty(&value) {
            Ok(pretty) => pretty,
            Err(_) => value.to_string(),
        }
    }

    fn stagger_delay(&self) -> time::Duration {
        time::Duration::from_millis(300)
    }

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
