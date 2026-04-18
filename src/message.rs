//! Message composition utilities.

use crate::context;
use crate::defaults;
use crate::settings;
use crate::time;

/// Composes a message for an alert notification.
///
/// The message is constructed from the provided `context::Context` of the main
/// loop, and the `settings::MessageStrings` of the backend of the calling notifier.
///
/// Some placeholders are supported in the message strings, which will be
/// replaced with values from the context. See [`replace_placeholders`].
///
/// # Parameters
/// - `ctx`: The context of the main loop, containing state and timestamps.
/// - `strings`: The message strings from the backend of the calling notifier.
///
/// # Returns
/// A composed alert message, ready to be sent by a notifier.
pub fn compose_alert_message(ctx: &context::Context, strings: &settings::MessageStrings) -> String {
    compose_common(
        ctx,
        &strings.alert_header,
        &strings.alert_body,
        &strings.footer,
    )
}

/// Composes a message for a reminder notification.
///
/// See [`compose_alert_message`] for more details.
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

/// Composes a message for a notification upon failure to start up properly.
///
/// See [`compose_alert_message`] for more details.
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

/// Composes a message for a notification upon successful startup.
///
/// See [`compose_alert_message`] for more details.
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

/// Common routine for composing a message for a notification.
///
/// This does not have a `settings::MessageStrings` parameter, and instead
/// the caller must provide the relevant header, body and footer strings directly.
/// Having it in a separate function like this greatly deduplicates code.
///
/// Some placeholders are supported in the message strings, which will be
/// replaced with values from the context. See [`replace_placeholders`].
///
/// # Parameters
/// - `ctx`: The context of the main loop, containing state and timestamps.
/// - `header`: The header string for the message, which may be empty.
/// - `body`: The body string for the message, which may be empty.
/// - `footer`: The footer string for the message, which may be empty.
///
/// # Returns
/// A composed message of unspecified type, ready to be sent by a notifier.
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

/// Replaces placeholders in a message string with values from the passed context.
///
/// The currently supported placeholders are:
/// - `{fuzzy_now}`: The current time in a human-friendly format that may be
///   a mixture of date and time, depending how long ago the time was.
///   In the case of the current time, this will always be a timestamp without date.
/// - `{time_now}`: The current time in `HH:MM` format.
/// - `{date_now}`: The current date in `YYYY-MM-DD` format.
/// - `{fuzzy_then}`: The time of the context's `now` field. This is generally
///   the same as the current time, but may be different if the context is
///   from a retry of a previously failed send.
/// - `{time_then}`: The time of the context's `now` field, in `HH:MM` format.
/// - `{date_then}`: The date of the context's `now` field, in `YYYY-MM-DD` format.
/// - `{name}`: The name of the program.
/// - `{version}`: The version of the program.
/// - `{fuzzy_low}`: The time of the most recent transition to
///   `source::Reading::Low` in a human-friendly format.
/// - `{fuzzy_high}`: The time of the most recent transition to
///   `source::Reading::High` in a human-friendly format.
/// - `{fuzzy_state_change}`: The time of the most recent transition to either
///   `source::Reading::Low` or `source::Reading::High` in a human-friendly format.
///
/// # Parameters
/// - `body`: The message string potentially containing placeholders to replace.
/// - `ctx`: The context of the main loop, containing state and timestamps to
///   replace the placeholders with.
///
/// # Returns
/// The message string with placeholders replaced with values from the context
/// and/or with such based on the current time and date.
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

/// Unescapes a string, replacing some escape sequences with their literal characters.
///
/// The currently supported escape sequences are:
/// - `\\`: A literal backslash (`\`).
/// - `\"`: A literal double quote (`"`).
/// - `\n`: A newline character.
/// - `\r`: A carriage return character.
/// - `\t`: A tab character.
/// - `\{`: A literal opening curly brace (`{`).
/// - `\}`: A literal closing curly brace (`}`).
///
/// With this it's possible to have strings in the configuration file that
/// contain `\n` without having to actually insert a (for example) newline
/// with the `"""` syntax.
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
