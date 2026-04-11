use std::time;

use crate::source;

#[derive(Debug, Clone)]
pub struct Context {
    pub loop_iteration: u64,
    pub went_low_at: Option<Timestamp>,
    pub went_high_at: Option<Timestamp>,
    pub time_of_state_change: Option<Timestamp>,
    pub time_of_startup_from_low: Option<Timestamp>,
    pub previous_reading: source::Reading,
    pub startup_succeeded: bool,
    pub now: Timestamp,
}

impl Context {
    pub fn new() -> Self {
        Self {
            loop_iteration: 0,
            went_low_at: None,
            went_high_at: None,
            time_of_state_change: None,
            time_of_startup_from_low: None,
            previous_reading: source::Reading::Low,
            startup_succeeded: false,
            now: Timestamp::now(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Timestamp {
    pub instant: time::Instant,
    pub wall: chrono::DateTime<chrono::Local>,
}

impl Timestamp {
    pub fn now() -> Self {
        let instant = time::Instant::now();
        let wall = chrono::Local::now();
        Self { instant, wall }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageType {
    Alert,
    Reminder,
    StartupFailed,
    StartupSuccess,
}

#[derive(Clone)]
pub struct FailedSendAttempt {
    pub message_type: MessageType,
    pub ctx: Context,
}

impl FailedSendAttempt {
    pub fn new(message_type: &MessageType, ctx: &Context) -> Self {
        Self {
            message_type: *message_type,
            ctx: ctx.clone(),
        }
    }
}
