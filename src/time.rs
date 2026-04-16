//! Time-related utilities.

use std::time;

/// A timestamp that allows for measuring both wall-clock time and elapsed time.
#[derive(Copy, Clone)]
pub struct Timestamp {
    /// The instant component of the timestamp, used for measuring elapsed time.
    pub instant: time::Instant,

    /// The wall-clock component of the timestamp, used for human-readable time.
    pub wall: chrono::DateTime<chrono::Local>,
}

impl Timestamp {
    /// Creates a new `Timestamp` based on the current time.
    pub fn now() -> Self {
        let instant = time::Instant::now();
        let wall = chrono::Local::now();
        Self { instant, wall }
    }
}

/// Expresses a timestamp as a fuzzy string which may omit the date or time
/// if the timestamp is recent (or distant) enough.
pub fn fuzzy_datestamp_of(when: &chrono::DateTime<chrono::Local>) -> String {
    const THREE_DAYS: chrono::Duration = chrono::Duration::days(3);
    const TWELVE_HOURS: chrono::Duration = chrono::Duration::hours(12);

    let ago = chrono::Local::now().signed_duration_since(when);

    if ago > THREE_DAYS {
        when.format("%Y-%m-%d").to_string()
    } else if ago > TWELVE_HOURS {
        when.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        when.format("%H:%M:%S").to_string()
    }
}
