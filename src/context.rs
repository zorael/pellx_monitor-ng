//! Structs for storing and passing around the program's state and context.

use crate::source;
use crate::time;

/// Struct encapsulating the context of the main loop, including state and timestamps.
///
/// This struct is passed to notifiers when they send notifications, allowing
/// them to include relevant information in their messages. In the case where
/// a notification fails, a copy of the failed context is stored for later retrying.
#[derive(Clone)]
pub struct Context {
    /// The number of iterations of the main loop that have been completed.
    pub loop_iteration: u64,

    /// The time of the most recent transition to `source::Reading::Low`.
    pub went_low_at: Option<time::Timestamp>,

    /// The time of the most recent transition to `source::Reading::High`.
    pub went_high_at: Option<time::Timestamp>,

    /// The time of the most recent transition to either `source::Reading::Low`
    /// or `source::Reading::High` (from the opposite reading).
    pub time_of_state_change: Option<time::Timestamp>,

    /// The time when the pellets burner managed to start back up from an error state.
    pub time_of_startup: Option<time::Timestamp>,

    /// The previous reading from the input source.
    pub previous_reading: source::Reading,

    /// Whether the pellets burner managed to start back up from an error state.
    pub startup_succeeded: bool,

    /// The current time, snapshot at the beginning of each loop iteration.
    pub now: time::Timestamp,
}

impl Context {
    /// Creates a new `Context` with default values.
    pub fn new() -> Self {
        Self {
            loop_iteration: 0,
            went_low_at: None,
            went_high_at: None,
            time_of_state_change: None,
            time_of_startup: None,
            previous_reading: source::Reading::Low,
            startup_succeeded: false,
            now: time::Timestamp::now(),
        }
    }
}
