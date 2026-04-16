//! Logging utilities and macros.

/// Prints a timestamp prefix to stdout in the format `"[HH:MM:SS] "`.
/// The trailing space is not an error.
///
/// This is used as part of the `tsprintln!` macro to print a timestamp
/// prefix before a message.
pub fn print_timestamp_prefix() {
    let timestamp = chrono::Local::now().format("%H:%M:%S");
    print!("[{timestamp}] ");
}

/// Prints a timestamp prefix to stderr in the format `"[HH:MM:SS] "`.
/// The trailing space is not an error.
///
/// This is used as part of the `tseprintln!` macro to print a timestamp
/// prefix before a message.
pub fn eprint_timestamp_prefix() {
    let timestamp = chrono::Local::now().format("%H:%M:%S");
    eprint!("[{timestamp}] ");
}

/// Prints a message to stdout, optionally prefixed with a timestamp.
///
/// The timestamp is printed if the passed `disable_timestamps` argument is false.
///
/// # Parameters
/// - `disable_timestamps`: If true, the message is printed without a timestamp prefix.
/// - `args`: The arguments to print, in the same format as `println!`.
macro_rules! tsprintln {
    ($disable_timestamps:expr, $($arg:tt)*) => {
        if !$disable_timestamps {
            $crate::logging::print_timestamp_prefix();
        }
        println!($($arg)*);
    }
}

/// Prints a message to stderr, optionally prefixed with a timestamp.
///
/// The timestamp is printed if the passed `disable_timestamps` argument is false.
///
/// # Parameters
/// - `disable_timestamps`: If true, the message is printed without a timestamp prefix.
/// - `args`: The arguments to print, in the same format as `eprintln!`.
macro_rules! tseprintln {
    ($disable_timestamps:expr, $($arg:tt)*) => {
        if !$disable_timestamps {
            $crate::logging::eprint_timestamp_prefix();
        }
        eprintln!($($arg)*);
    }
}

// Publicly re-export the macros.
pub(crate) use {tseprintln, tsprintln};
