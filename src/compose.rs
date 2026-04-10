use crate::context;

pub fn compose_alert_message(ctx: &context::Context) -> String {
    let mut message = String::new();

    message.push_str("PellX Monitor Alert\n\n");

    message.push_str(&format!("Went high at: {:?}\n", ctx.went_high_at));

    message
}

pub fn compose_reminder_message(ctx: &context::Context) -> String {
    let mut message = String::new();

    message.push_str("PellX Monitor Reminder\n\n");

    message.push_str(&format!("Went high at: {:?}\n", ctx.went_high_at));

    message
}

pub fn compose_startup_failed_message(ctx: &context::Context) -> String {
    let mut message = String::new();

    message.push_str("PellX Monitor Startup Failed\n\n");

    message.push_str(&format!(
        "Time of startup from low: {:?}\n",
        ctx.time_of_startup_from_low
    ));

    message
}
