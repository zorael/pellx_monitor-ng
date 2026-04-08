use rppal::gpio;
use std::time;

#[derive(Debug, Clone)]
pub struct Context {
    pub loop_iteration: u64,
    pub went_low_at: Option<time::Instant>,
    pub went_high_at: Option<time::Instant>,
    pub time_of_state_change: Option<time::Instant>,
    pub time_of_startup_from_low: Option<time::Instant>,
    pub previous_reading: gpio::Level,
    pub startup_succeeded: bool,
    pub start: time::Instant,
    pub now: time::Instant,
}

impl Context {
    pub fn new(start: time::Instant) -> Self {
        Self {
            loop_iteration: 0,
            went_low_at: None,
            went_high_at: None,
            time_of_state_change: None,
            time_of_startup_from_low: None,
            previous_reading: gpio::Level::Low,
            startup_succeeded: false,
            start,
            now: start,
        }
    }
}
