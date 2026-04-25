//! Batsign backend implementation.

use std::time;

use crate::context;
use crate::notify;
use crate::settings;

/// Backend implementation for sending notifications to Batsign via webhooks.
pub struct BatsignBackend {
    /// Unique numeric identifier for this backend instance.
    pub id: usize,

    /// Name of the backend, used for display purposes.
    pub name: String,

    /// HTTP client agent used to send requests to the Batsign server.
    pub agent: ureq::Agent,

    /// Batsign URL to send notifications to.
    pub url: String,

    /// Whether to show the HTTP response from the Batsign server in terminal output.
    pub show_response: bool,

    /// Custom message strings, used to compose notifications.
    pub strings: settings::MessageStrings,
}

impl BatsignBackend {
    /// Creates a new instance of the Batsign backend.
    ///
    /// # Parameters
    /// - `id`: Unique numeric identifier for this backend instance.
    /// - `agent`: HTTP client agent used to send requests to the Batsign server.
    /// - `url`: Batsign URL to send notifications to.
    /// - `show_response`: Whether to show the HTTP response from the Batsign
    ///   server in terminal output.
    /// - `strings`: Custom message strings, used to compose notifications.
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

    /// Returns the stagger delay for this backend instance.
    ///
    /// For Batsign, this is currently a hardcoded duration of 300 milliseconds.
    fn stagger_delay(&self) -> time::Duration {
        time::Duration::from_millis(300)
    }

    /// Appends "Subject: " to the composed message and sends a HTTP POST
    /// request to the Batsign server with the message as the request body.
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
