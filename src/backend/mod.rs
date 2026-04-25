//! Backend implementations for emitting notifications.

mod batsign;
mod command;
mod println;
mod slack;

pub use batsign::BatsignBackend;
pub use command::CommandBackend;
pub use println::PrintlnBackend;
pub use slack::SlackBackend;

use std::time;

use crate::context;
use crate::message;
use crate::notify;
use crate::settings;

/// Trait describing a notification backend.
pub trait Backend {
    /// Returns the unique numeric identifier of a backend.
    fn id(&self) -> usize;

    /// Returns the name of a backend.
    fn name(&self) -> &str;

    /// Returns the message strings associated with a backend.
    fn strings(&self) -> &settings::MessageStrings;

    /// Composes a message for a backend.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `message_type`: Type of the message being composed.
    ///
    /// # Returns
    /// A composed message body, as a string.
    fn compose(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        match message_type {
            notify::MessageType::Alert => message::compose_alert_message(ctx, self.strings()),
            notify::MessageType::Reminder => message::compose_reminder_message(ctx, self.strings()),
            notify::MessageType::StartupFailed => {
                message::compose_startup_failed_message(ctx, self.strings())
            }
            notify::MessageType::StartupSuccess => {
                message::compose_startup_success_message(ctx, self.strings())
            }
        }
    }

    /// Composes a message for a backend in a way that is suitable for
    /// display in terminal output.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `message_type`: Type of the message being composed.
    ///
    /// # Returns
    /// A composed display message, as a string.
    fn compose_display(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        self.compose(ctx, message_type)
    }

    /// Returns the stagger delay for a backend.
    ///
    /// The default duration is zero.
    fn stagger_delay(&self) -> time::Duration {
        time::Duration::ZERO
    }

    /// Emits a notification via a backend.
    ///
    /// # Parameters
    /// - `ctx`: Context of the notification.
    /// - `body`: Composed message body to send to the backend.
    /// - `message_type`: Type of the message being emitted.
    ///
    /// # Returns
    /// - `Some(response)` if the notification was sent successfully and the
    ///   backend was set up to return responses.
    /// - `None` if the notification was sent successfully otherwise.
    ///
    /// # Errors
    /// Errors if there was an error sending the notification.
    fn emit(
        &self,
        ctx: &context::Context,
        body: &str,
        message_type: notify::MessageType,
    ) -> Result<Option<String>, String>;
}
