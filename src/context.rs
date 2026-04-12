use crate::source;
use crate::time;

#[derive(Debug, Clone)]
pub struct Context {
    pub loop_iteration: u64,
    pub went_low_at: Option<time::Timestamp>,
    pub went_high_at: Option<time::Timestamp>,
    pub time_of_state_change: Option<time::Timestamp>,
    pub time_of_startup_from_low: Option<time::Timestamp>,
    pub previous_reading: source::Reading,
    pub startup_succeeded: bool,
    pub now: time::Timestamp,
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
            now: time::Timestamp::now(),
        }
    }
}
