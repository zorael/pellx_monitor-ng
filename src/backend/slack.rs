use crate::compose;
use crate::context;
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
        let name = format!("slack-{}", id);

        Self {
            id,
            name,
            agent,
            url: url.to_string(),
            show_response,
            strings,
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

    fn compose_alert(&self, ctx: &context::Context) -> String {
        let message = compose::compose_alert_message(ctx, &self.strings);
        serde_json::json!({ "text": message }).to_string()
    }

    fn compose_alert_display(&self, ctx: &context::Context) -> String {
        let message = compose::compose_alert_message(ctx, &self.strings);
        let value = serde_json::json!({ "text": message });

        match serde_json::to_string_pretty(&value) {
            Ok(pretty) => pretty,
            Err(_) => value.to_string(),
        }
    }

    fn compose_reminder(&self, ctx: &context::Context) -> String {
        let message = compose::compose_reminder_message(ctx, &self.strings);
        serde_json::json!({ "text": message }).to_string()
    }

    fn compose_reminder_display(&self, ctx: &context::Context) -> String {
        let message = compose::compose_reminder_message(ctx, &self.strings);
        let value = serde_json::json!({ "text": message });

        match serde_json::to_string_pretty(&value) {
            Ok(pretty) => pretty,
            Err(_) => value.to_string(),
        }
    }

    fn compose_startup_failed(&self, ctx: &context::Context) -> String {
        let message = compose::compose_startup_failed_message(ctx, &self.strings);
        serde_json::json!({ "text": message }).to_string()
    }

    fn compose_startup_failed_display(&self, ctx: &context::Context) -> String {
        let message = compose::compose_startup_failed_message(ctx, &self.strings);
        let value = serde_json::json!({ "text": message });

        match serde_json::to_string_pretty(&value) {
            Ok(pretty) => pretty,
            Err(_) => value.to_string(),
        }
    }

    fn compose_startup_success(&self, ctx: &context::Context) -> String {
        let message = compose::compose_startup_success_message(ctx, &self.strings);
        serde_json::json!({ "text": message }).to_string()
    }

    fn compose_startup_success_display(&self, ctx: &context::Context) -> String {
        let message = compose::compose_startup_success_message(ctx, &self.strings);
        let value = serde_json::json!({ "text": message });

        match serde_json::to_string_pretty(&value) {
            Ok(pretty) => pretty,
            Err(_) => value.to_string(),
        }
    }

    fn emit(
        &self,
        _ctx: &context::Context,
        message: &str,
        _message_type: &context::MessageType,
    ) -> Result<Option<String>, String> {
        let json: serde_json::Value = serde_json::from_str(message).map_err(|e| e.to_string())?;

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
