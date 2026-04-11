mod backend;
mod cli;
mod compose;
mod config;
mod context;
mod defaults;
mod notify;
mod settings;
mod source;

use clap::Parser;
use std::error::Error;
use std::path;
use std::process;
use std::thread;
use std::time;

fn print_banner() {
    println!(
        "{} v{} | copyright (c) 2026 {}\n$ git clone {}",
        defaults::program_metadata::NAME,
        defaults::program_metadata::VERSION,
        defaults::program_metadata::AUTHORS,
        defaults::program_metadata::SOURCE_REPOSITORY
    );
}

fn main() -> process::ExitCode {
    let cli = cli::Cli::parse();

    print_banner();

    if cli.version {
        println!(
            "\nThis project is licensed under {}, at your option.",
            defaults::program_metadata::LICENSE
        );
        return process::ExitCode::SUCCESS;
    }

    let config_path = match &cli.config {
        Some(path) if path.exists() => path,
        Some(path) => {
            eprintln!("Specified config file {} does not exist", path.display());
            return process::ExitCode::FAILURE;
        }
        None => &path::PathBuf::from(defaults::program_metadata::CONFIG_FILENAME),
    };

    let config = match confy::load_path(config_path) {
        Ok(config) => match config {
            Some(config) => config,
            None => {
                eprintln!(
                    "No config file found (at {}), using defaults",
                    defaults::program_metadata::CONFIG_FILENAME
                );
                config::Config::default()
            }
        },
        Err(err) => {
            eprintln!("Failed to load configuration: {err}");
            let mut src = err.source();

            while let Some(e) = src {
                eprintln!("  caused by: {e}");
                src = e.source();
            }

            return process::ExitCode::FAILURE;
        }
    };

    let mut settings = settings::Settings::default();
    settings.apply_config(&config);
    settings.apply_cli(&cli);

    if cli.save {
        return save_settings(settings);
    }

    run_loop(settings)
}

fn run_loop(settings: settings::Settings) -> process::ExitCode {
    let mut source: Box<dyn source::InputSource> = match settings.monitor.source {
        source::ChoiceOfInputSource::Gpio => {
            Box::new(source::GpioInputSource::new(settings.gpio.pin))
        }
        source::ChoiceOfInputSource::Dummy => Box::new(source::MockInputSource::new()),
    };

    match source.init() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Failed to initialize input source! {err}");
            return process::ExitCode::FAILURE;
        }
    }

    let mut notifiers: Vec<Box<dyn notify::StatefulNotifier>> = Vec::new();
    let agent = ureq::Agent::new_with_defaults();

    if settings.slack.enabled {
        for (i, url) in settings.slack.urls.iter().enumerate() {
            let backend = backend::SlackBackend::new(
                i,
                agent.clone(),
                url,
                settings.slack.show_response,
                settings.slack.strings.clone(),
            );

            let n = notify::Notifier::new(backend, settings.dry_run);
            notifiers.push(Box::new(n));
        }
    }

    if settings.batsign.enabled {
        for (i, url) in settings.batsign.urls.iter().enumerate() {
            let backend = backend::BatsignBackend::new(
                i,
                agent.clone(),
                url,
                settings.batsign.show_response,
                settings.batsign.strings.clone(),
            );

            let n = notify::Notifier::new(backend, settings.dry_run);
            notifiers.push(Box::new(n));
        }
    }

    if settings.command.enabled {
        for (i, command) in settings.command.commands.iter().enumerate() {
            let backend = backend::CommandBackend::new(
                i,
                command,
                settings.command.show_response,
                settings.command.strings.clone(),
            );
            let n = notify::Notifier::new(backend, settings.dry_run);
            notifiers.push(Box::new(n));
        }
    }

    if settings.println.enabled {
        let backend = backend::PrintlnBackend::new(0, settings.println.strings.clone());
        let n = notify::Notifier::new(backend, settings.dry_run);

        notifiers.push(Box::new(n));
    }

    match settings.sanity_check() {
        Ok(()) => (),
        Err(errors) => {
            eprintln!("Configuration sanity check failed with the following errors:");
            for error in errors {
                eprintln!("  - {error}");
            }
            return process::ExitCode::FAILURE;
        }
    }

    let mut ctx = context::Context::new();

    loop {
        ctx.now = context::Timestamp::now();

        if at_least_one_notifier_is_due_for_retry(&mut notifiers, &ctx.now.instant) {
            send_retries(&mut notifiers, &ctx.now.instant, &settings);
        }

        let reading = source.read();
        let reading_changed = reading != ctx.previous_reading;
        let is_first_iteration = ctx.loop_iteration == 0;

        println!(
            "{}: {reading:?}/{:?} => {reading_changed}",
            ctx.loop_iteration, ctx.previous_reading
        );

        if reading_changed {
            // Reset
            match reading {
                source::Reading::Low => {
                    ctx.went_low_at = Some(context::Timestamp::now());
                    ctx.time_of_startup_from_low = None;
                    ctx.startup_succeeded = false;
                }
                source::Reading::High => {
                    ctx.went_high_at = Some(context::Timestamp::now());
                }
            }

            // Update
            ctx.previous_reading = reading;
            ctx.time_of_state_change = Some(context::Timestamp::now());
        }

        if ctx.time_of_state_change.is_none() && reading == source::Reading::Low {
            // Program was just started, state has never changed from low
            end_loop(&mut ctx, settings.monitor.loop_interval);
            continue;
        }

        match reading {
            source::Reading::Low => {
                if ctx.startup_succeeded {
                    // All is well
                    end_loop(&mut ctx, settings.monitor.loop_interval);
                    continue;
                }

                // We are low, but we don't know if we have completely started up yet
                let time_of_startup_from_low = match ctx.time_of_startup_from_low {
                    Some(t) => t,
                    None => {
                        // First loop after going low, can't have started up yet
                        // (provided startup_duration > 0)
                        println!("-- NEW LOW --");
                        ctx.time_of_startup_from_low = Some(context::Timestamp::now());
                        end_loop(&mut ctx, settings.monitor.loop_interval);
                        continue;
                    }
                };

                if time_of_startup_from_low.instant.elapsed()
                    >= settings.monitor.max_allowed_startup_time
                {
                    // Startup succeeded, can notify success
                    send_startup_success_to_all(&mut notifiers, &ctx);
                    ctx.startup_succeeded = true;
                }

                end_loop(&mut ctx, settings.monitor.loop_interval);
                continue;
            }
            source::Reading::High => {
                if reading_changed || is_first_iteration {
                    println!("-- NEW HIGH --");

                    if let Some(t) = ctx.time_of_startup_from_low
                        && t.instant.elapsed() < settings.monitor.max_allowed_startup_time
                    {
                        // We went high again before startup duration elapsed, this is a startup failure
                        send_startup_failed_to_all(&mut notifiers, &ctx);
                        end_loop(&mut ctx, settings.monitor.loop_interval);
                        continue;
                    }

                    // We just randomly went HIGH for no reason, this is an alert
                    send_alert_to_all(&mut notifiers, &ctx);
                } else {
                    // We have been HIGH for a while, this is a reminder
                    send_reminder_to_all(&mut notifiers, &ctx);
                }

                end_loop(&mut ctx, settings.monitor.loop_interval);
                continue;
            }
        }
    }
}

fn end_loop(ctx: &mut context::Context, interval: time::Duration) {
    ctx.loop_iteration += 1;
    thread::sleep(interval);
}

fn at_least_one_notifier_is_due_for_retry(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    now: &time::Instant,
) -> bool {
    for n in notifiers {
        match n.state().time_of_next_retry {
            Some(t) if t >= *now => {
                // The time is in the future, not yet due for retry
                continue;
            }
            Some(_) => {
                // Due for retry
                return true;
            }
            None => {
                // No retry scheduled
                continue;
            }
        }
    }

    false
}

fn send_startup_success_to_all(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &context::Context,
) -> notify::NotificationResult {
    let mut result = notify::NotificationResult {
        total: notifiers.len(),
        ..Default::default()
    };

    for n in notifiers {
        match send_startup_success_to_one(n, ctx) {
            notify::SendResult::Success => result.success += 1,
            notify::SendResult::Failure => result.failure += 1,
            notify::SendResult::TryAgainLater => result.try_again_later += 1,
        }
    }

    result
}

fn send_startup_success_to_one(
    n: &mut Box<dyn notify::StatefulNotifier>,
    ctx: &context::Context,
) -> notify::SendResult {
    let result = n.send_startup_success(ctx);

    match result {
        notify::SendResult::Success => {
            n.state_mut().reset();
            n.state_mut().bump_time_of_next_reminder();
        }
        notify::SendResult::Failure => {
            n.state_mut().on_failure(ctx);
            n.state_mut().bump_time_of_next_retry();
        }
        notify::SendResult::TryAgainLater => {}
    };

    result
}

fn send_startup_failed_to_all(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &context::Context,
) -> notify::NotificationResult {
    let mut result = notify::NotificationResult {
        total: notifiers.len(),
        ..Default::default()
    };

    for n in notifiers {
        match send_startup_failed_to_one(n, ctx) {
            notify::SendResult::Success => result.success += 1,
            notify::SendResult::Failure => result.failure += 1,
            notify::SendResult::TryAgainLater => result.try_again_later += 1,
        }
    }

    result
}

fn send_startup_failed_to_one(
    n: &mut Box<dyn notify::StatefulNotifier>,
    ctx: &context::Context,
) -> notify::SendResult {
    let result = n.send_startup_failed(ctx);

    match result {
        notify::SendResult::Success => {
            n.state_mut().reset();
            n.state_mut().bump_time_of_next_reminder();
        }
        notify::SendResult::Failure => {
            n.state_mut().on_failure(ctx);
            n.state_mut().bump_time_of_next_retry();
        }
        notify::SendResult::TryAgainLater => {}
    };

    result
}

fn send_alert_to_all(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &context::Context,
) -> notify::NotificationResult {
    let mut result = notify::NotificationResult {
        total: notifiers.len(),
        ..Default::default()
    };

    for n in notifiers {
        match send_alert_to_one(n, ctx) {
            notify::SendResult::Success => result.success += 1,
            notify::SendResult::Failure => result.failure += 1,
            notify::SendResult::TryAgainLater => result.try_again_later += 1,
        }
    }

    result
}

fn send_alert_to_one(
    n: &mut Box<dyn notify::StatefulNotifier>,
    ctx: &context::Context,
) -> notify::SendResult {
    let result = n.send_alert(ctx);

    match result {
        notify::SendResult::Success => {
            n.state_mut().reset();
            n.state_mut().bump_time_of_next_reminder();
        }
        notify::SendResult::Failure => {
            n.state_mut().on_failure(ctx);
            n.state_mut().bump_time_of_next_retry();
        }
        notify::SendResult::TryAgainLater => {}
    };

    result
}

fn send_reminder_to_all(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &context::Context,
) -> notify::NotificationResult {
    let mut result = notify::NotificationResult {
        total: notifiers.len(),
        ..Default::default()
    };

    for n in notifiers {
        match send_reminder_to_one(n, ctx) {
            notify::SendResult::Success => result.success += 1,
            notify::SendResult::Failure => result.failure += 1,
            notify::SendResult::TryAgainLater => result.try_again_later += 1,
        }
    }

    result
}

fn send_reminder_to_one(
    n: &mut Box<dyn notify::StatefulNotifier>,
    ctx: &context::Context,
) -> notify::SendResult {
    match n.state().time_of_next_reminder {
        Some(t) if t > ctx.now.instant => {
            // The time of next reminder is in the future; not yet due
            return notify::SendResult::TryAgainLater;
        }
        Some(_) => {
            // Due for reminder, drop down
        }
        None => {
            // No reminder scheduled? This should not happen
            eprintln!(
                "Logic error: send_reminder_to_one called on notifier {} \
                but it is missing a time_of_next_reminder",
                n.name()
            );
            return notify::SendResult::Failure;
        }
    }

    let result = n.send_reminder(ctx);

    match result {
        notify::SendResult::Success => {
            n.state_mut().on_reminder_success();
            n.state_mut().bump_time_of_next_reminder();
        }
        notify::SendResult::Failure => {
            n.state_mut().on_failure(ctx);
            n.state_mut().bump_time_of_next_retry();
        }
        notify::SendResult::TryAgainLater => {}
    };

    result
}

fn send_retries(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    now: &time::Instant,
    settings: &settings::Settings,
) {
    for n in notifiers {
        match n.state().time_of_next_retry {
            Some(t) if t >= *now => {
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

        let Some(previous_failed_ctx) = &n.state().previous_failed_context.clone() else {
            eprintln!(
                "Error: retry calculated due but {} is missing a failed context",
                n.name()
            );
            continue;
        };

        match previous_failed_ctx.time_of_startup_from_low {
            Some(t) if t.instant.elapsed() < settings.monitor.max_allowed_startup_time => {
                // This was a startup failure
                let _ = send_startup_failed_to_one(n, previous_failed_ctx);
                continue;
            }
            Some(_) => {
                // This may be a startup failure, we don't know.
                // See f it is a reminder or an alert below.
            }
            None => {
                // This is a very rare first-iteration HIGH failure
            }
        }

        match n.state().time_of_next_reminder {
            Some(t) if t < *now => {
                // This was a reminder failure
                let _ = send_reminder_to_one(n, previous_failed_ctx);
                continue;
            }
            Some(_) => {
                // This was a reminder failure but we should not yet send a new one
                continue;
            }
            None => {}
        }

        // This was an alert failure
        let _ = send_alert_to_one(n, previous_failed_ctx);
    }
}

fn save_settings(settings: settings::Settings) -> process::ExitCode {
    let config_file = settings.config_file.clone();

    let config = config::Config::from(settings.clone());

    match confy::store_path(config_file, config) {
        Ok(()) => {
            println!(
                "Config saved successfully to {}",
                settings.config_file.display()
            );
            process::ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!(
                "Failed to save configuration to {}: {err}",
                settings.config_file.display()
            );
            process::ExitCode::FAILURE
        }
    }
}

enum MessageType {
    Alert,
    Reminder,
    StartupFailed,
    StartupSuccess,
}
