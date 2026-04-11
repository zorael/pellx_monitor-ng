use crate::compose;
use crate::context;

pub struct SlackBackend {
    pub id: usize,
    pub name: String,
    pub agent: ureq::Agent,
    pub url: String,
    pub show_response: bool,
}

impl SlackBackend {
    pub fn new(id: usize, agent: ureq::Agent, url: &str, show_response: bool) -> Self {
        let name = format!("slack-{}", id);

        Self {
            id,
            name,
            agent,
            url: url.to_string(),
            show_response,
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

    fn compose_alert(&self, ctx: &context::Context) -> String {
        compose::compose_alert_message(ctx)
    }

    fn compose_reminder(&self, ctx: &context::Context) -> String {
        compose::compose_reminder_message(ctx)
    }

    fn compose_startup_failed(&self, ctx: &context::Context) -> String {
        compose::compose_startup_failed_message(ctx)
    }

    fn emit(&self, message: &str) -> Result<Option<String>, String> {
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
