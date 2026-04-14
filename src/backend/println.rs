use crate::context;
use crate::notify;
use crate::settings;

pub struct PrintlnBackend {
    pub id: usize,
    pub name: String,
    pub strings: settings::MessageStrings,
}

impl PrintlnBackend {
    pub fn new(id: usize, strings: settings::MessageStrings) -> Self {
        let name = format!("println-{id}");
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

    fn strings(&self) -> &settings::MessageStrings {
        &self.strings
    }

    fn emit(
        &self,
        _ctx: &context::Context,
        message: &str,
        message_type: notify::MessageType,
    ) -> Result<Option<String>, String> {
        println!("[println] emit: {message_type:?}");
        println!("---");
        println!("{message}");
        println!("---");
        Ok(None)
    }
}
