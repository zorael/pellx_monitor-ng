use std::time;

use crate::context;
use crate::notify;
use crate::settings;

pub struct BatsignBackend {
    pub id: usize,
    pub name: String,
    pub agent: ureq::Agent,
    pub url: String,
    pub show_response: bool,
    pub strings: settings::MessageStrings,
}

impl BatsignBackend {
    pub fn new(
        id: usize,
        agent: ureq::Agent,
        url: &str,
        show_response: bool,
        strings: settings::MessageStrings,
    ) -> Self {
        let name = format!("batsign-{id}");

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

impl super::Backend for BatsignBackend {
    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn strings(&self) -> &settings::MessageStrings {
        &self.strings
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
        let body = format!("Subject: {body}");

        match self.agent.post(&self.url).send(&body) {
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
