use std::time;

use crate::context;
use crate::defaults;
use crate::settings;

pub fn compose_alert_message(ctx: &context::Context, strings: &settings::MessageStrings) -> String {
    compose_common(ctx, &strings.alert_header, &strings.alert_body)
}

pub fn compose_reminder_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    compose_common(ctx, &strings.reminder_header, &strings.reminder_body)
}

pub fn compose_startup_failed_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    compose_common(
        ctx,
        &strings.startup_failed_header,
        &strings.startup_failed_body,
    )
}

fn compose_common(ctx: &context::Context, header: &str, body: &str) -> String {
    let mut message = String::new();

    if header.is_empty() {
        return message;
    }

    message.push_str(header);
    message.push('\n');
    message.push_str(body);
    replace_placeholders(&message, ctx)
}

fn replace_placeholders(message: &str, ctx: &context::Context) -> String {
    let mut out = message.to_string();

    if let Some(ref went_high_at) = ctx.went_high_at {
        out = out.replace("{went_high_at}", &fuzzy_datestamp_of(&went_high_at.wall));
    }

    if let Some(ref went_low_at) = ctx.went_low_at {
        out = out.replace("{went_low_at}", &fuzzy_datestamp_of(&went_low_at.wall));
    }

    if let Some(ref time_of_state_change) = ctx.time_of_state_change {
        out = out.replace(
            "{time_of_state_change}",
            &fuzzy_datestamp_of(&time_of_state_change.wall),
        );
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
