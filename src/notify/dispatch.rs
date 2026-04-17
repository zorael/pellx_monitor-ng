//! Dispatch logic for sending notifications to notifiers.

use std::thread;
use std::time;

use crate::context;
use crate::logging;
use crate::settings;

/// Sends a notification via all notifiers.
///
/// # Parameters
/// - `notifiers`: The notifiers to send the notification through.
/// - `settings`: The program's global settings.
/// - `ctx`: The context of the main loop.
/// - `message_type`: The type of message being sent.
///
/// # Returns
/// A `NotificationResult` containing counts of the outcomes of the send attempts.
pub fn send_to_all(
    notifiers: &mut Vec<Box<dyn super::StatefulNotifier>>,
    settings: &settings::Settings,
    ctx: &context::Context,
    message_type: super::MessageType,
) -> NotificationResult {
    let mut result = NotificationResult {
        total: notifiers.len(),
        ..Default::default()
    };

    for n in notifiers {
        if n.id() > 0 {
            // Rate limit to avoid overwhelming backends
            thread::sleep(n.stagger_delay());
        }
        match send_to_one(n, ctx, message_type) {
            super::SendResult::Success(output) => {
                if let Some(output) = output {
                    logging::tsprintln!(
                        settings.disable_timestamps,
                        "[{}] push success:\n{output}",
                        n.name()
                    );
                }
                result.success += 1;
            }
            super::SendResult::Failure(output) => {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "[{}] push FAILURE:\n{output}",
                    n.name()
                );
                result.failure += 1;
            }
            super::SendResult::TryAgainLater => result.try_again_later += 1,
        }
    }

    result
}

/// Sends a notification via a single notifier.
///
/// # Parameters
/// - `n`: The notifier to send the notification through.
/// - `ctx`: The context of the main loop.
/// - `message_type`: The type of message being sent.
///
/// # Returns
/// A `SendResult` indicating the outcome of the send attempt.
fn send_to_one(
    n: &mut Box<dyn super::StatefulNotifier>,
    ctx: &context::Context,
    message_type: super::MessageType,
) -> super::SendResult {
    if message_type == super::MessageType::Reminder {
        match n.state().time_of_next_reminder {
            Some(t) if t > ctx.now.instant => {
                // The time of next reminder is in the future; not yet due
                return super::SendResult::TryAgainLater;
            }
            Some(_) => {
                // Due for reminder, drop down
            }
            None => {
                // No reminder scheduled? This should not happen
                return super::SendResult::Failure(
                    "Logic error: missing time_of_next_reminder".to_string(),
                );
            }
        }
    }

    let result = dispatch(n, ctx, message_type);
    apply_send_result(n, ctx, message_type, &result);
    result
}

/// Issues retries for notifiers with stored failed send attempts.
///
/// # Parameters
/// - `notifiers`: The notifiers to consider for retries.
/// - `settings`: The program's global settings.
/// - `now`: The current time, used to determine if retries are due.
pub fn send_retries(
    notifiers: &mut Vec<Box<dyn super::StatefulNotifier>>,
    settings: &settings::Settings,
    now: &time::Instant,
) {
    for n in notifiers {
        let Some(previous_failed_send) = n.state().previous_failed_send.clone() else {
            // No previous failed send, this notifier just hasn't had a failed send
            continue;
        };

        match &n.state().time_of_next_retry {
            Some(t) if t > now => {
                // The time is in the future, not yet due for retry
                continue;
            }
            Some(_) => {
                // Due for retry
            }
            None => {
                // No retry scheduled
                continue;
            }
        }

        // Consider staggering messages here later if server 429 turns out to be an issue.
        // It currently hasn't shown to be, hence why it's not implemented.

        match previous_failed_send.message_type {
            super::MessageType::StartupSuccess | super::MessageType::Alert => {
                // Should always be retried at any opportunity
                // Drop down and dispatch
            }
            super::MessageType::StartupFailed => {
                let Some(t) = previous_failed_send.ctx.time_of_startup else {
                    // Logic error, should never happen
                    continue;
                };

                if t.instant.elapsed() < settings.monitor.startup_window {
                    // Not enough time has passed to be able to call it a startup failure
                    continue;
                }
            }
            super::MessageType::Reminder => {
                let Some(t) = &n.state().time_of_next_reminder else {
                    // Logic error, should never happen
                    continue;
                };

                if t > now {
                    // Not yet due for reminder retry, skip
                    continue;
                }

                // Drop down and dispatch
            }
        }

        let result = dispatch(
            n,
            &previous_failed_send.ctx,
            previous_failed_send.message_type,
        );

        match &result {
            super::SendResult::Success(_) => {
                logging::tsprintln!(
                    settings.disable_timestamps,
                    "[{}] retry succeeded (message type: {:?})",
                    n.name(),
                    previous_failed_send.message_type
                );
            }
            super::SendResult::Failure(_) => {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "[{}] retry FAILED (message type: {:?})",
                    n.name(),
                    previous_failed_send.message_type
                );
            }
            super::SendResult::TryAgainLater => {}
        }

        apply_send_result(
            n,
            &previous_failed_send.ctx,
            previous_failed_send.message_type,
            &result,
        );
    }
}

/// Dispatches a notification send attempt to the appropriate method of a
/// notifier, based on the passed message type.
///
/// # Parameters
/// - `n`: The notifier to send the notification through.
/// - `ctx`: The context of the main loop.
/// - `message_type`: The type of message being sent.
///
/// # Returns
/// A `SendResult` indicating the outcome of the send attempt.
fn dispatch(
    n: &mut Box<dyn super::StatefulNotifier>,
    ctx: &context::Context,
    message_type: super::MessageType,
) -> super::SendResult {
    match message_type {
        super::MessageType::Alert => n.send_alert(ctx),
        super::MessageType::Reminder => n.send_reminder(ctx),
        super::MessageType::StartupFailed => n.send_startup_failed(ctx),
        super::MessageType::StartupSuccess => n.send_startup_success(ctx),
    }
}

/// Applies the result of a send attempt to the state of the notifier,
/// updating it in-place.
///
/// # Parameters
/// - `n`: The notifier whose state is to be updated based on the send result.
/// - `ctx`: The context of the main loop.
/// - `message_type`: The type of message that was attempted to be sent.
/// - `result`: The result of the send attempt, indicating success, failure,
///   or a request to try again later.
fn apply_send_result(
    n: &mut Box<dyn super::StatefulNotifier>,
    ctx: &context::Context,
    message_type: super::MessageType,
    result: &super::SendResult,
) {
    match &result {
        super::SendResult::Success(_) => {
            match message_type {
                super::MessageType::StartupSuccess => {
                    // End of the line, no reminders wanted
                    n.state_mut().on_startup_success();
                }
                super::MessageType::Reminder => {
                    n.state_mut().on_reminder_success();
                    n.state_mut().bump_time_of_next_reminder();
                }
                _ => {
                    // This kind of message type should have reminders.
                    // We need to set the reminder timestamp at *some* point.
                    // Is this not the right place?
                    n.state_mut().bump_time_of_next_reminder();
                }
            }

            // Reset failure state
            n.state_mut().retry_count = 0;
        }
        super::SendResult::Failure(_) => {
            n.state_mut().on_failure(ctx, message_type);
            n.state_mut().bump_time_of_next_retry();
        }
        super::SendResult::TryAgainLater => {}
    }
}

/// Result of a notification send attempt, indicating success, failure, or a
/// request to try again later.
#[derive(Default)]
pub struct NotificationResult {
    /// The total number of notifiers that a notification was attempted to be
    /// sent through.
    pub total: usize,

    /// The number of notifiers that successfully sent the notification.
    pub success: usize,

    /// The number of notifiers that failed to send the notification.
    pub failure: usize,

    /// The number of notifiers that requested to try again later, due to
    /// timing or rate-limiting reasons.
    pub try_again_later: usize,
}
