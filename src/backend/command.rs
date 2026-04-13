use std::path;
use std::process;

use crate::context;
use crate::notify;
use crate::settings;
use crate::time;

pub struct CommandBackend {
    pub id: usize,
    pub name: String,
    pub command: path::PathBuf,
    pub show_response: bool,
    pub strings: settings::MessageStrings,
}

impl CommandBackend {
    pub fn new(
        id: usize,
        command: &str,
        show_response: bool,
        strings: settings::MessageStrings,
    ) -> Self {
        let name = format!("command-{}", id);

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
    fn id(&self) -> usize {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn strings(&self) -> &settings::MessageStrings {
        &self.strings
    }

    fn emit(
        &self,
        ctx: &context::Context,
        message: &str,
        message_type: &notify::MessageType,
    ) -> Result<Option<String>, String> {
        let command = process::Command::new(&self.command)
            .arg(message)
            .arg(format!("{message_type:?}"))
            .arg(ctx.loop_iteration.to_string())
            .arg(get_unix_timestamp(&ctx.went_low_at).to_string())
            .arg(get_unix_timestamp(&ctx.went_high_at).to_string())
            .arg(get_unix_timestamp(&ctx.time_of_state_change).to_string())
            .arg(get_unix_timestamp(&ctx.time_of_startup_from_low).to_string())
            .output()
            .map_err(|e| e.to_string())?;

        if self.show_response {
            let mut output = String::new();
            let stdout = String::from_utf8_lossy(&command.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&command.stderr).trim().to_string();

            if !stdout.is_empty() {
                output.push_str(&format!("STDOUT:\n{stdout}\n"));
            }

            if !stderr.is_empty() {
                if !output.is_empty() {
                    output.push_str("---\n");
                }
                output.push_str(&format!("STDERR:\n{stderr}\n"));
            }

            output = output.trim().to_string();

            if !output.is_empty() {
                Ok(Some(output))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

fn get_unix_timestamp(wall: &Option<time::Timestamp>) -> i64 {
    match wall {
        Some(t) => t.wall.timestamp(),
        None => 0,
    }
}
