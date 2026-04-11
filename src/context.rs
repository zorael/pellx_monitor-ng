use std::time;

use crate::source;

#[derive(Debug, Clone)]
pub struct Context {
    pub loop_iteration: u64,
    /*pub went_low_at: Option<time::Instant>,
    pub went_high_at: Option<time::Instant>,
    pub time_of_state_change: Option<time::Instant>,
    pub time_of_startup_from_low: Option<time::Instant>,*/
    pub went_low_at: Option<Timestamp>,
    pub went_high_at: Option<Timestamp>,
    pub time_of_state_change: Option<Timestamp>,
    pub time_of_startup_from_low: Option<Timestamp>,
    pub previous_reading: source::Reading,
    pub startup_succeeded: bool,
    pub _start: time::Instant,
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
            previous_reading: source::Reading::Low,
            startup_succeeded: false,
            _start: start,
            now: start,
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
