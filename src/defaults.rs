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
    pub const REQUIRED_TIME_FOR_STARTUP: time::Duration = time::Duration::from_secs(5);
}
