mod batsign;
mod command;
mod println;
mod slack;

pub use batsign::BatsignBackend;
pub use command::CommandBackend;
pub use println::PrintlnBackend;
pub use slack::SlackBackend;

use crate::context;

pub trait Backend {
    #[allow(unused)]
    fn id(&self) -> usize;
    fn name(&self) -> &str;

    fn compose_alert(&self, ctx: &context::Context) -> String;
    fn compose_reminder(&self, ctx: &context::Context) -> String;
    fn compose_startup_failed(&self, ctx: &context::Context) -> String;
    fn compose_startup_success(&self, ctx: &context::Context) -> String;

    fn emit(&self, ctx: &context::Context, message: &str) -> Result<Option<String>, String>;
}
