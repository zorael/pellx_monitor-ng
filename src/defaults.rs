use std::time;

pub mod program_metadata {
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    pub const CONFIG_FILENAME: &str = "config.toml";
    pub const SOURCE_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
    pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
}

pub mod gpio {
    pub const PIN: u8 = 24;
}

pub mod monitor {
    use super::*;

    pub const LOOP_INTERVAL: time::Duration = time::Duration::from_secs(1);
    pub const MAX_ALLOWED_STARTUP_TIME: time::Duration = time::Duration::from_secs(5);
    pub const REMINDER_INTERVAL: time::Duration = time::Duration::from_secs(5);
    pub const NOTIFICATION_RETRY_INTERVAL: time::Duration = time::Duration::from_secs(5);
}
