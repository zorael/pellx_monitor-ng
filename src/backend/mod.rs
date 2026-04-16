mod batsign;
mod command;
mod println;
mod slack;

pub use batsign::BatsignBackend;
pub use command::CommandBackend;
pub use println::PrintlnBackend;
pub use slack::SlackBackend;

use std::time;

use crate::compose;
use crate::context;
use crate::notify;
use crate::settings;

pub trait Backend {
    fn id(&self) -> usize;
    fn name(&self) -> &str;
    fn strings(&self) -> &settings::MessageStrings;

    fn compose(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        match message_type {
            notify::MessageType::Alert => compose::compose_alert_message(ctx, self.strings()),
            notify::MessageType::Reminder => compose::compose_reminder_message(ctx, self.strings()),
            notify::MessageType::StartupFailed => {
                compose::compose_startup_failed_message(ctx, self.strings())
            }
            notify::MessageType::StartupSuccess => {
                compose::compose_startup_success_message(ctx, self.strings())
            }
        }
    }

    fn compose_display(&self, ctx: &context::Context, message_type: notify::MessageType) -> String {
        self.compose(ctx, message_type)
    }

    fn stagger_delay(&self) -> time::Duration {
        time::Duration::ZERO
    }

    fn emit(
        &self,
        ctx: &context::Context,
        message: &str,
        message_type: notify::MessageType,
    ) -> Result<Option<String>, String>;
}
