use std::thread;
use std::time;

use crate::context;
use crate::logging;
use crate::settings;

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
                        "[{}] success:\n{output}",
                        n.name()
                    );
                }
                result.success += 1;
            }
            super::SendResult::Failure(output) => {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "[{}] FAILURE:\n{output}",
                    n.name()
                );
                result.failure += 1;
            }
            super::SendResult::TryAgainLater => result.try_again_later += 1,
        }
    }

    result
}

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
    apply_send_result(n, ctx, message_type, result)
}

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
                    "Retry succeeded for notifier {} (message type: {:?})",
                    n.name(),
                    previous_failed_send.message_type
                );
            }
            super::SendResult::Failure(_) => {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "Retry failed for notifier {} (message type: {:?})",
                    n.name(),
                    previous_failed_send.message_type
                );
            }
            super::SendResult::TryAgainLater => {}
        }

        let _ = apply_send_result(
            n,
            &previous_failed_send.ctx,
            previous_failed_send.message_type,
            result,
        );
    }
}

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

fn apply_send_result(
    n: &mut Box<dyn super::StatefulNotifier>,
    ctx: &context::Context,
    message_type: super::MessageType,
    result: super::SendResult,
) -> super::SendResult {
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

    // Pass through
    result
}

#[derive(Default)]
pub struct NotificationResult {
    pub total: usize,
    pub success: usize,
    pub failure: usize,
    pub try_again_later: usize,
}
