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

    /// Emits a notification via the Println backend.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `body`: Composed message body to print to the terminal.
    /// - `message_type`: Type of the message being emitted.
    ///
    /// # Returns
    /// `Ok(Some(String))` containing the composed message body, for the caller
    /// to print to the terminal.
    fn emit(
        &self,
        _ctx: &context::Context,
        body: &str,
        _message_type: notify::MessageType,
    ) -> Result<Option<String>, String> {
        Ok(Some(body.to_string()))
    }
}
