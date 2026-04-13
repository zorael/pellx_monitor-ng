pub fn print_timestamp_prefix() {
    let timestamp = chrono::Local::now().format("%H:%M:%S");
    print!("[{timestamp}] ");
}

pub fn eprint_timestamp_prefix() {
    let timestamp = chrono::Local::now().format("%H:%M:%S");
    eprint!("[{timestamp}] ");
}

macro_rules! tsprintln {
    ($disable_timestamps:expr, $($arg:tt)*) => {
        if !$disable_timestamps {
            $crate::logging::print_timestamp_prefix();
        }
        println!($($arg)*);
    }
}

macro_rules! tseprintln {
    ($disable_timestamps:expr, $($arg:tt)*) => {
        if !$disable_timestamps {
            $crate::logging::eprint_timestamp_prefix();
        }
        eprintln!($($arg)*);
    }
}

pub(crate) use {tseprintln, tsprintln};
