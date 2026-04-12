mod batsign;
mod command;
mod println;
mod slack;

use crate::settings;
use crate::compose;

pub use batsign::BatsignBackend;
pub use command::CommandBackend;
pub use println::PrintlnBackend;
pub use slack::SlackBackend;

use crate::context;

pub trait Backend {
    #[allow(unused)]
    fn id(&self) -> usize;
    fn name(&self) -> &str;
    fn strings(&self) -> &settings::MessageStrings;

    fn compose_alert(&self, ctx: &context::Context) -> String {
        compose::compose_alert_message(ctx, &self.strings())
    }

    fn compose_reminder(&self, ctx: &context::Context) -> String {
        compose::compose_reminder_message(ctx, &self.strings())
    }

    fn compose_startup_failed(&self, ctx: &context::Context) -> String {
        compose::compose_startup_failed_message(ctx, &self.strings())
    }

    fn compose_startup_success(&self, ctx: &context::Context) -> String {
        compose::compose_startup_success_message(ctx, &self.strings())
    }

    fn emit(
        &self,
        ctx: &context::Context,
        message: &str,
        message_type: &context::MessageType,
    ) -> Result<Option<String>, String>;
}
