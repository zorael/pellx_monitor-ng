use crate::context;
use crate::defaults;
use crate::settings;

pub fn compose_alert_message(ctx: &context::Context, strings: &settings::MessageStrings) -> String {
    compose_common(
        ctx,
        &strings.alert_header,
        &strings.alert_body,
        &strings.footer,
    )
}

pub fn compose_reminder_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    compose_common(
        ctx,
        &strings.reminder_header,
        &strings.reminder_body,
        &strings.footer,
    )
}

pub fn compose_startup_failed_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    compose_common(
        ctx,
        &strings.startup_failed_header,
        &strings.startup_failed_body,
        &strings.footer,
    )
}

pub fn compose_startup_success_message(
    ctx: &context::Context,
    strings: &settings::MessageStrings,
) -> String {
    compose_common(
        ctx,
        &strings.startup_success_header,
        &strings.startup_success_body,
        &strings.footer,
    )
}

fn compose_common(ctx: &context::Context, header: &str, body: &str, footer: &str) -> String {
    let mut message = String::new();

    if header.is_empty() {
        return message;
    }

    message.push_str(header);
    message.push('\n');
    message.push_str(body);

    if !footer.is_empty() {
        message.push('\n');
        message.push_str(footer);
    }

    replace_placeholders(&message, ctx)
}

fn replace_placeholders(message: &str, ctx: &context::Context) -> String {
    let mut out = message.to_string();

    if let Some(went_high_at) = &ctx.went_high_at {
        out = out.replace("{went_high_at}", &fuzzy_datestamp_of(&went_high_at.wall));
    }

    if let Some(went_low_at) = &ctx.went_low_at {
        out = out.replace("{went_low_at}", &fuzzy_datestamp_of(&went_low_at.wall));
    }

    if let Some(time_of_state_change) = &ctx.time_of_state_change {
        out = out.replace(
            "{time_of_state_change}",
            &fuzzy_datestamp_of(&time_of_state_change.wall),
        );
    }

    if let Some(time_of_startup_from_low) = &ctx.time_of_startup_from_low {
        out = out.replace(
            "{time_of_startup_from_low}",
            &fuzzy_datestamp_of(&time_of_startup_from_low.wall),
        );
    }

    let now = chrono::Local::now();

    out = out.replace("{fuzzy_now}", &fuzzy_datestamp_of(&now));
    out = out.replace("{time_now}", &now.format("%H:%M").to_string());
    out = out.replace("{date_now}", &now.format("%Y-%m-%d").to_string());
    out = out.replace("{fuzzy_then}", &fuzzy_datestamp_of(&ctx.now.wall));
    out = out.replace("{time_then}", &ctx.now.wall.format("%H:%M").to_string());
    out = out.replace("{date_then}", &ctx.now.wall.format("%Y-%m-%d").to_string());
    out = out.replace("{name}", defaults::program_metadata::NAME);
    out = out.replace("{version}", defaults::program_metadata::VERSION);
    out
}

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
