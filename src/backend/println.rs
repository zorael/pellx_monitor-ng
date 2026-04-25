//! Println backend implementation.

use crate::context;
use crate::notify;
use crate::settings;

/// Backend implementation for printing notifications to the terminal using `println`.
pub struct PrintlnBackend {
    /// Unique numeric identifier for this backend instance.
    pub id: usize,

    /// Name of the backend, used for display purposes.
    pub name: String,

    /// Custom message strings, used to compose notifications.
    pub strings: settings::MessageStrings,
}

impl PrintlnBackend {
    /// Creates a new instance of the Println backend.
    ///
    /// # Parameters
    /// - `id`: Unique numeric identifier for this backend instance.
    /// - `strings`: Custom message strings, used to compose notifications.
    pub fn new(id: usize, strings: settings::MessageStrings) -> Self {
        let name = format!("println-{id}");
        Self { id, name, strings }
    }
}

impl super::Backend for PrintlnBackend {
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

    /// Returns the message body as-is, faking a successful notification push,
    /// additionally as if the backend was set up to return responses.
    fn emit(
        &self,
        _ctx: &context::Context,
        body: &str,
        _message_type: notify::MessageType,
    ) -> Result<Option<String>, String> {
        Ok(Some(body.to_string()))
    }
}
