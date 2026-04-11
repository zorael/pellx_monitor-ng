use crate::compose;
use crate::context;

use std::process;

pub struct CommandBackend {
    pub id: usize,
    pub name: String,
    pub command: String,
    pub show_response: bool,
}

impl CommandBackend {
    pub fn new(id: usize, command: &str, show_response: bool) -> Self {
        let name = format!("command-{}", id);

        Self {
            id,
            name,
            command: command.to_string(),
            show_response,
        }
    }
}

impl super::Backend for CommandBackend {
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
        let command = process::Command::new(&self.command)
            .arg(message)
            .output()
            .map_err(|e| e.to_string())?;

        if self.show_response {
            let output = String::from_utf8_lossy(&command.stdout).trim().to_string();
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
