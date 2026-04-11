use std::time;

use crate::context;
use crate::defaults;
use crate::settings;

pub fn compose_alert_message(ctx: &context::Context, strings: &settings::MessageStrings) -> String {
    let mut message = String::new();

    message.push_str("PellX Monitor Alert\n\n");
    message.push_str(&format!("Went high at: {:?}\n", ctx.went_high_at));
    message
}

pub fn compose_reminder_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    let mut message = String::new();

    message.push_str(&strings.reminder_header);
    message.push('\n');
    message.push_str(&strings.reminder_body);
    replace_placeholders(&mut message, ctx);

    message
}

pub fn compose_startup_failed_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    let mut message = String::new();

    message.push_str("PellX Monitor Startup Failed\n\n");
    message.push_str(&format!(
        "Time of startup from low: {:?}\n",
        ctx.time_of_startup_from_low
    ));
    message
}

fn replace_placeholders(message: &mut String, ctx: &context::Context) -> String {
    let mut out = message.clone();

    if let Some(ref went_high_at) = ctx.went_high_at {
        out = out.replace("{went_high_at}", &fuzzy_datestamp_of(&went_high_at.wall));
    }

    if let Some(ref went_low_at) = ctx.went_low_at {
        out = out.replace("{went_low_at}", &fuzzy_datestamp_of(&went_low_at.wall));
    }

    if let Some(ref time_of_state_change) = ctx.time_of_state_change {
        out = out.replace("{time_of_state_change}", &fuzzy_datestamp_of(&time_of_state_change.wall));
    }

    if let Some(ref time_of_startup_from_low) = ctx.time_of_startup_from_low {
        out = out.replace(
            "{time_of_startup_from_low}",
            &fuzzy_datestamp_of(&time_of_startup_from_low.wall),
        );
    }

    let now = chrono::Local::now();

    out = out.replace("{current_fuzzy_datestamp}", &fuzzy_datestamp_of(&now));
    out = out.replace("{current_time}", &now.format("%H:%M").to_string());
    out = out.replace("{current_date}", &now.format("%Y-%m-%d").to_string());
    out = out.replace("{name}", defaults::program_metadata::NAME);
    out = out.replace("{version}", defaults::program_metadata::VERSION);
    out
}

pub fn fuzzy_datestamp_of(when: &chrono::DateTime<chrono::Local>) -> String {
    const THREE_DAYS: time::Duration = time::Duration::from_secs(3 * 24 * 3600);
    const TWELVE_HOURS: time::Duration = time::Duration::from_secs(12 * 3600);

    let now = time::SystemTime::now();
    let when_system_time = time::SystemTime::from(*when);
    let since = now.duration_since(when_system_time).unwrap_or_default();

    if since > THREE_DAYS {
        when.format("%Y-%m-%d").to_string()
    } else if since > TWELVE_HOURS {
        when.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        when.format("%H:%M:%S").to_string()
    }
}
