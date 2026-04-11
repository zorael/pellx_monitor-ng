mod batsign;
mod command;
mod println;
mod slack;

pub use println::PrintlnBackend;

use crate::context;

pub trait Backend {
    #[allow(unused)]
    fn id(&self) -> usize;
    fn name(&self) -> &str;

    fn compose_alert(&self, ctx: &context::Context) -> String;
    fn compose_reminder(&self, ctx: &context::Context) -> String;
    fn compose_startup_failed(&self, ctx: &context::Context) -> String;

    fn emit(&self, message: &str) -> Result<Option<String>, String>;
}
