use crate::compose;
use crate::context;
use crate::settings;

pub struct PrintlnBackend {
    pub id: usize,
    pub name: String,
    pub strings: settings::MessageStrings,
}

impl PrintlnBackend {
    pub fn new(id: usize, strings: settings::MessageStrings) -> Self {
        let name = format!("println-{}", id);

        Self { id, name, strings }
    }
}

impl super::Backend for PrintlnBackend {
    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn compose_alert(&self, ctx: &context::Context) -> String {
        compose::compose_alert_message(ctx, &self.strings)
    }

    fn compose_reminder(&self, ctx: &context::Context) -> String {
        compose::compose_reminder_message(ctx, &self.strings)
    }

    fn compose_startup_failed(&self, ctx: &context::Context) -> String {
        compose::compose_startup_failed_message(ctx, &self.strings)
    }

    fn emit(&self, _ctx: &context::Context, message: &str) -> Result<Option<String>, String> {
        println!("{message}");
        Ok(None)
    }
}
