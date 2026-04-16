pub mod program_metadata {
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    pub const CONFIG_FILENAME: &str = "pellxd.toml";
    pub const SOURCE_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
    pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
}

pub mod gpio {
    pub const PIN: u8 = 24;
}

pub mod dummy {
    pub const MODULUS: u32 = 30;
    pub const THRESHOLD: u32 = 15;
}

pub mod monitor {
    use std::time;

    pub const LOOP_INTERVAL: time::Duration = time::Duration::from_secs(1);
    pub const STARTUP_WINDOW: time::Duration = time::Duration::from_secs(480); // 8 minutes
}

#[allow(unused_variables)]
pub mod exit_codes {
    pub const FAILED_TO_LOAD_CONFIG_FILE: u8 = 10;
    pub const FAILED_TO_RESOLVE_CONFIG_DIRECTORY: u8 = 11;
    pub const CONFIG_FILE_DOES_NOT_EXIST: u8 = 12;
    pub const CONFIG_DIRECTORY_NOT_FOUND: u8 = 13;
    pub const CONFIG_DIRECTORY_NOT_A_DIRECTORY: u8 = 14;
    pub const CONFIG_SANITY_CHECK_FAILED: u8 = 15;
    pub const FAILED_TO_SAVE_CONFIG_FILE: u8 = 16;
    pub const NO_NOTIFIERS_CONFIGURED: u8 = 20;
}
