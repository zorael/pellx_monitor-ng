use crate::compose;
use crate::context;

pub struct PrintlnBackend {
    pub id: usize,
    pub name: String,
}

impl PrintlnBackend {
    pub fn new(id: usize) -> Self {
        let name = format!("println-{}", id);

        Self { id, name }
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
        compose::compose_alert_message(ctx)
    }

    fn compose_reminder(&self, ctx: &context::Context) -> String {
        compose::compose_reminder_message(ctx)
    }

    fn compose_startup_failed(&self, ctx: &context::Context) -> String {
        compose::compose_startup_failed_message(ctx)
    }

    fn emit(&self, message: &str) -> Result<Option<String>, String> {
        println!("{message}");
        Ok(None)
    }
}
