use crate::context;
use crate::defaults;
use crate::settings;
use crate::time;

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
    let mut msg = String::new();

    if header.is_empty() {
        return msg;
    }

    msg.push_str(header);
    msg.push('\n');
    msg.push_str(body);

    if !footer.is_empty() {
        msg.push('\n');
        msg.push_str(footer);
    }

    msg = unescape(&msg);
    replace_placeholders(&msg, ctx).trim_end().to_string()
}

fn replace_placeholders(body: &str, ctx: &context::Context) -> String {
    let mut out = body.to_string();


    if let Some(went_low_at) = &ctx.went_low_at {
        out = out.replace("{fuzzy_low}", &time::fuzzy_datestamp_of(&went_low_at.wall));
    }

    if let Some(went_high_at) = &ctx.went_high_at {
        out = out.replace(
            "{fuzzy_high}",
            &time::fuzzy_datestamp_of(&went_high_at.wall),
        );
    }

    if let Some(time_of_state_change) = &ctx.time_of_state_change {
        out = out.replace(
            "{fuzzy_state_change}",
            &time::fuzzy_datestamp_of(&time_of_state_change.wall),
        );
    }

    if let Some(time_of_startup) = &ctx.time_of_startup {
        out = out.replace(
            "{fuzzy_startup}",
            &time::fuzzy_datestamp_of(&time_of_startup.wall),
        );
    }

    let now = chrono::Local::now();

    out = out.replace("{fuzzy_now}", &time::fuzzy_datestamp_of(&now));
    out = out.replace("{time_now}", &now.format("%H:%M").to_string());
    out = out.replace("{date_now}", &now.format("%Y-%m-%d").to_string());
    out = out.replace("{fuzzy_then}", &time::fuzzy_datestamp_of(&ctx.now.wall));
    out = out.replace("{time_then}", &ctx.now.wall.format("%H:%M").to_string());
    out = out.replace("{date_then}", &ctx.now.wall.format("%Y-%m-%d").to_string());
    out = out.replace("{name}", defaults::program_metadata::NAME);
    out = out.replace("{version}", defaults::program_metadata::VERSION);

    out
}

fn unescape(input: &str) -> String {
    input
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\{", "{")
        .replace("\\}", "}")
}
