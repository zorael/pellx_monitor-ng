mod backend;
mod context;
mod notify;
mod source;

use rppal::gpio;
use std::thread;
use std::time;

fn main() {
    run_loop();
}

fn run_loop() {
    let mut source: Box<dyn source::InputSource> = Box::new(source::MockInputSource::new(24));

    let notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>> = &mut Vec::new();
    notifiers.push(Box::new(notify::Notifier::new(
        backend::PrintlnBackend::new(0, "println"),
        false,
    )));

    let interval = time::Duration::from_secs(1);
    let startup_duration = time::Duration::from_secs(7);

    let mut ctx = context::Context::new(time::Instant::now());

    loop {
        ctx.now = time::Instant::now();
        let reading = source.read();
        let reading_changed = reading != ctx.previous_reading;
        let is_first_iteration = ctx.loop_iteration == 0;

        println!(
            "{}: {reading:?}/{:?} => {is_first_iteration}",
            ctx.loop_iteration, ctx.previous_reading
        );

        if reading_changed {
            // Reset
            match reading {
                gpio::Level::Low => {
                    ctx.went_low_at = Some(ctx.now);
                    ctx.time_of_startup_from_low = None;
                    ctx.startup_succeeded = false;
                }
                gpio::Level::High => {
                    ctx.went_high_at = Some(ctx.now);
                }
            }

            // Update
            ctx.previous_reading = reading;
            let _ = ctx.time_of_state_change.insert(ctx.now);
        }

        if ctx.time_of_state_change.is_none() && reading == gpio::Level::Low {
            // Program was just started, state has never changed from low
            end_loop(&mut ctx, interval);
            continue;
        }

        match reading {
            gpio::Level::Low => {
                if ctx.startup_succeeded {
                    // All is well
                    end_loop(&mut ctx, interval);
                    continue;
                }

                // We are low, but we don't know if we have completely started up yet
                let time_of_startup_from_low = match ctx.time_of_startup_from_low {
                    Some(t) => t,
                    None => {
                        // First loop after going low, can't have started up yet
                        // (provided startup_duration > 0)
                        println!("-- NEW LOW --");
                        ctx.time_of_startup_from_low = Some(ctx.now);
                        end_loop(&mut ctx, interval);
                        continue;
                    }
                };

                if time_of_startup_from_low.elapsed() >= startup_duration {
                    // Startup succeeded, can notify success
                    println!("--> notify LOW");
                    ctx.startup_succeeded = true;
                }

                end_loop(&mut ctx, interval);
                continue;
            }
            gpio::Level::High => {
                if reading_changed || is_first_iteration {
                    println!("-- NEW HIGH --");

                    if let Some(t) = ctx.time_of_startup_from_low
                        && t.elapsed() < startup_duration
                    {
                        // We went high again before startup duration elapsed, this is a startup failure
                        send_startup_failed(notifiers, &ctx);
                        end_loop(&mut ctx, interval);
                        continue;
                    }

                    // We just randomly went HIGH for no reason, this is an alert
                    send_alert(notifiers, &ctx);
                } else {
                    // We have been HIGH for a while, this is a reminder
                    send_reminder(notifiers, &ctx);
                }

                end_loop(&mut ctx, interval);
                continue;
            }
        }
    }
}

fn end_loop(ctx: &mut context::Context, interval: time::Duration) {
    ctx.loop_iteration += 1;
    thread::sleep(interval);
}

fn send_startup_failed(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &context::Context,
) {
    for n in notifiers {
        match n.send_startup_failed(ctx) {
            notify::SendResult::Success => {}
            notify::SendResult::Failure => {
                let mut state = n.state_mut();
                state.previous_failed_context = Some(ctx.clone());
            }
        }
    }
}

fn send_alert(notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>, ctx: &context::Context) {
    for n in notifiers {
        match n.send_alert(ctx) {
            notify::SendResult::Success => {}
            notify::SendResult::Failure => {
                let mut state = n.state_mut();
                state.previous_failed_context = Some(ctx.clone());
            }
        }
    }
}

fn send_reminder(notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>, ctx: &context::Context) {
    for n in notifiers {
        if !n.state().next_reminder_is_due() {
            continue;
        }

        n.send_reminder(ctx);
    }
}

fn send_retries(notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>) {
    for n in notifiers {
        if !n.state().next_retry_is_due() {
            continue;
        }

        let previous_failed_ctx = match &n.state().previous_failed_context {
            Some(ctx) => ctx.clone(),
            None => continue,
        };

        match previous_failed_ctx.time_of_startup_from_low {
            Some(t) if t.elapsed() >= time::Duration::from_secs(10) => {
                // This was a startup failure
                match n.send_startup_failed(&previous_failed_ctx) {
                    notify::SendResult::Success => {
                        n.state_mut().previous_failed_context = None;
                    }
                    notify::SendResult::Failure => {
                        let state = n.state_mut();
                        state.previous_failed_context = Some(previous_failed_ctx.clone());
                    }
                }
                continue;
            }
            Some(_) => {
                // This may be a startup failure, we don't know.
                // See if it is a reminder or an alert below.
            }
            None => {
                // This is a very rare first-iteration HIGH failure
            }
        }

        match n.state().time_of_last_reminder {
            Some(t) if t.elapsed() >= time::Duration::from_secs(10) => {
                // This was a reminder failure
                match n.send_reminder(&previous_failed_ctx) {
                    notify::SendResult::Success => {
                        n.state_mut().time_of_last_reminder = None;
                    }
                    notify::SendResult::Failure => {
                        let state = n.state_mut();
                        state.time_of_last_reminder = Some(time::Instant::now());
                    }
                }
                continue;
            }
            Some(t) => {
                // This was a reminder failure but we should not yet send a new one
                continue;
            }
            None => {
                // This was an alert failure
                match n.send_alert(&previous_failed_ctx) {
                    notify::SendResult::Success => {
                        n.state_mut().time_of_last_alert = None;
                    }
                    notify::SendResult::Failure => {
                        let state = n.state_mut();
                        state.time_of_last_alert = Some(time::Instant::now());
                    }
                }
            }
        }
    }
}
