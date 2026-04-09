use crate::context;

pub trait Backend {
    #[allow(unused)]
    fn id(&self) -> usize;
    fn name(&self) -> &str;

    fn compose_alert(&self, ctx: &context::Context) -> &str;
    fn compose_reminder(&self, ctx: &context::Context) -> &str;
    fn compose_startup_failed(&self, ctx: &context::Context) -> &str;

    fn emit(&self, message: &str) -> Result<Option<String>, String>;
}

pub struct PrintlnBackend {
    pub id: usize,
    pub name: String,
}

impl PrintlnBackend {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

impl Backend for PrintlnBackend {
    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn compose_alert(&self, _ctx: &context::Context) -> &str {
        "ALERT"
    }

    fn compose_reminder(&self, _ctx: &context::Context) -> &str {
        "REMINDER"
    }

    fn compose_startup_failed(&self, _ctx: &context::Context) -> &str {
        "STARTUP FAILED"
    }

    fn emit(&self, message: &str) -> Result<Option<String>, String> {
        println!("emit: {message}");
        Ok(None)
    }
}
