//! External command backend implementation.

use std::fmt::Write;
use std::path;
use std::process;

use crate::context;
use crate::notify;
use crate::settings;
use crate::time;

/// Backend implementation for executing external commands.
pub struct CommandBackend {
    /// Unique numeric identifier for this backend instance.
    pub id: usize,

    /// Name of the backend, used for display purposes.
    pub name: String,

    /// Path to the external command to execute for notifications.
    pub command: path::PathBuf,

    /// Whether to show the stdout and stderr output the external command
    /// produces as terminal output.
    pub show_response: bool,

    /// Custom message strings, used to compose notifications.
    pub strings: settings::MessageStrings,
}

impl CommandBackend {
    /// Creates a new instance of the Command backend.
    ///
    /// # Parameters
    /// - `id`: Unique numeric identifier for this backend instance.
    /// - `command`: Path to the external command to execute for notifications.
    /// - `show_response`: Whether to show the stdout and stderr output the
    ///   external command produces as terminal output.
    /// - `strings`: Custom message strings, used to compose notifications.
    pub fn new(
        id: usize,
        command: &str,
        show_response: bool,
        strings: settings::MessageStrings,
    ) -> Self {
        let name = format!("command-{id}");

        Self {
            id,
            name,
            command: path::PathBuf::from(command),
            show_response,
            strings,
        }
    }
}

impl super::Backend for CommandBackend {
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

    /// Emits a notification via the Command backend.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `body`: Composed message body to pass as argument to the external command.
    /// - `message_type`: Type of the message being emitted.
    ///
    /// # Returns
    /// - `Ok(Some(String))` if the command executed successfully and the
    ///   response should be shown.
    /// - `Ok(None)` if the command executed successfully but the response
    ///   should not be shown.
    /// - `Err(String)` if there was an error running the command, with an
    ///   error message as a string.
    fn emit(
        &self,
        ctx: &context::Context,
        body: &str,
        message_type: notify::MessageType,
    ) -> Result<Option<String>, String> {
        let command = process::Command::new(&self.command)
            .arg(body)
            .arg(format!("{message_type:?}"))
            .arg(ctx.loop_iteration.to_string())
            .arg(get_unix_timestamp(ctx.went_low_at.as_ref()).to_string())
            .arg(get_unix_timestamp(ctx.went_high_at.as_ref()).to_string())
            .arg(get_unix_timestamp(ctx.time_of_state_change.as_ref()).to_string())
            .arg(get_unix_timestamp(ctx.time_of_startup.as_ref()).to_string())
            .output()
            .map_err(|e| e.to_string())?;

        if self.show_response {
            let mut output = String::new();
            let stdout = String::from_utf8_lossy(&command.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&command.stderr).trim().to_string();

            if !stdout.is_empty() {
                let _ = write!(output, "STDOUT:\n{stdout}\n");
            }

            if !stderr.is_empty() {
                if !output.is_empty() {
                    output.push_str("---\n");
                }
                let _ = write!(output, "STDERR:\n{stderr}\n");
            }

            output = output.trim().to_string();

            if output.is_empty() {
                Ok(None)
            } else {
                Ok(Some(output))
            }
        } else {
            Ok(None)
        }
    }
}

/// Expresses the passed time as a UNIX timestamp.
///
/// If it is `None`, this returns `0`.
///
/// # Parameters
/// - `when`: `Option<&time::Timestamp>` to convert to a UNIX timestamp.
fn get_unix_timestamp(when: Option<&time::Timestamp>) -> i64 {
    match when {
        Some(t) => t.wall.timestamp(),
        None => 0,
    }
}
