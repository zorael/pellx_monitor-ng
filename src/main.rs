mod backend;
mod cli;
mod compose;
mod config;
mod context;
mod defaults;
mod logging;
mod notify;
mod settings;
mod source;
mod time;

use clap::Parser;
use std::error::Error;
use std::path;
use std::process;
use std::thread;
use std::time as std_time;

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

    let config_path = match cli.config.as_deref() {
        Some(path) if path.exists() => path,
        Some(path) if cli.save => path, // allow saving to non-existent path
        Some(path) => {
            logging::tseprintln!(
                cli.disable_timestamps,
                "Config file {} does not exist",
                path.display()
            );
            return process::ExitCode::FAILURE;
        }
        None => &path::PathBuf::from(defaults::program_metadata::CONFIG_FILENAME),
    };

    let config = match load_config_file(config_path) {
        Ok(config) => config,
        //Err(_) if !config_path.exists() => None,
        Err(err) => {
            logging::tseprintln!(
                cli.disable_timestamps,
                "Failed to load configuration: {err}"
            );

            let mut src = err.source();

            while let Some(e) = src {
                eprintln!("\n  caused by: {e}");
                src = e.source();
            }

            return process::ExitCode::FAILURE;
        }
    };

    let mut settings = settings::Settings::default();
    settings.apply_config(config.as_ref());
    settings.apply_cli(&cli);

    match settings.sanity_check() {
        Ok(()) => (),
        Err(errors) => {
            logging::tseprintln!(
                settings.disable_timestamps,
                "Configuration sanity check failed with the following errors:"
            );

            for error in errors {
                logging::tseprintln!(settings.disable_timestamps, "  - {error}");
            }

            if settings.dry_run {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "Continuing anyway because --dry-run is enabled."
                );
            } else if !cli.save {
                // Allow errors if we're passing --save
                return process::ExitCode::FAILURE;
            }
        }
    }

    if cli.save {
        return settings.save(config_path);
    }

    let source = match init_source(&settings) {
        Ok(source) => source,
        Err(code) => return code,
    };

    let notifiers = build_notifiers(&settings);

    run_loop(notifiers, source, settings)
}

fn load_config_file(config_path: &path::Path) -> Result<Option<config::Config>, confy::ConfyError> {
    if !config_path.exists() {
        return Ok(None);
    }

    match confy::load_path(config_path) {
        Ok(config) => Ok(Some(config)),
        Err(err) => Err(err),
    }
}

fn init_source(
    settings: &settings::Settings,
) -> Result<Box<dyn source::InputSource>, process::ExitCode> {
    let mut source: Box<dyn source::InputSource> = match settings.monitor.source {
        source::ChoiceOfInputSource::Gpio => {
            Box::new(source::GpioInputSource::new(settings.gpio.pin))
        }
        source::ChoiceOfInputSource::Dummy => Box::new(source::MockInputSource::new()),
    };

    match source.init() {
        Ok(()) => Ok(source),
        Err(err) => {
            logging::tseprintln!(
                settings.disable_timestamps,
                "Failed to initialize input source! {err}"
            );
            Err(process::ExitCode::FAILURE)
        }
    }
}

fn build_notifiers(settings: &settings::Settings) -> Vec<Box<dyn notify::StatefulNotifier>> {
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

    notifiers
}

fn run_loop(
    mut notifiers: Vec<Box<dyn notify::StatefulNotifier>>,
    mut source: Box<dyn source::InputSource>,
    settings: settings::Settings,
) -> process::ExitCode {
    let mut ctx = context::Context::new();

    loop {
        ctx.now = time::Timestamp::now();

        let at_least_one_notifier_is_due_for_retry = notifiers
            .iter()
            .any(|n| n.state().has_due_retry(ctx.now.instant));

        if at_least_one_notifier_is_due_for_retry {
            notify::send_retries(&mut notifiers, &settings, &ctx.now.instant);
        }

        let reading = source.read();
        let reading_changed = reading != ctx.previous_reading;

        if settings.debug {
            println!(
                "{}: {reading:?}/{:?} => {reading_changed}",
                ctx.loop_iteration, ctx.previous_reading
            );
        }

        if reading_changed {
            // Reset
            match reading {
                source::Reading::Low => {
                    ctx.went_low_at = Some(time::Timestamp::now());
                    ctx.time_of_startup_from_low = None;
                    ctx.startup_succeeded = false;
                }
                source::Reading::High => {
                    ctx.went_high_at = Some(time::Timestamp::now());
                }
            }

            // Update
            ctx.previous_reading = reading;
            ctx.time_of_state_change = Some(time::Timestamp::now());
        }

        match reading {
            source::Reading::Low => handle_low_reading(&mut notifiers, &mut ctx, &settings),
            source::Reading::High => {
                handle_high_reading(&mut notifiers, &mut ctx, &settings, reading_changed)
            }
        }

        end_loop(&mut ctx, settings.monitor.loop_interval);
    }
}

fn handle_low_reading(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &mut context::Context,
    settings: &settings::Settings,
) {
    if ctx.time_of_state_change.is_none() || ctx.startup_succeeded {
        // Early return.
        // Either program was just started and state has never changed from low
        // or startup succeeded and all is well
        return;
    }

    // We are low, but we don't know if we have completely started up yet
    let time_of_startup_from_low = match ctx.time_of_startup_from_low {
        Some(t) => t,
        None => {
            // First loop after going low, can't have started up yet
            // (provided startup_duration > 0)
            if settings.debug {
                logging::tsprintln!(settings.disable_timestamps, "-- NEW LOW --");
            }

            ctx.time_of_startup_from_low = Some(time::Timestamp::now());
            return;
        }
    };

    if time_of_startup_from_low.instant.elapsed() >= settings.monitor.max_allowed_startup_time {
        // Startup succeeded, can notify success
        let result = notify::send_to_all(
            notifiers,
            settings,
            ctx,
            notify::MessageType::StartupSuccess,
        );

        if result.success != result.total {
            logging::tseprintln!(
                settings.disable_timestamps,
                "Failed to send some startup success notifications: {}/{} succeeded",
                result.success,
                result.total
            );
        }

        ctx.startup_succeeded = true;
    }
}

fn handle_high_reading(
    notifiers: &mut Vec<Box<dyn notify::StatefulNotifier>>,
    ctx: &mut context::Context,
    settings: &settings::Settings,
    reading_changed: bool,
) {
    let is_first_iteration = ctx.loop_iteration == 0;
    if reading_changed || is_first_iteration {
        if settings.debug {
            logging::tsprintln!(settings.disable_timestamps, "-- NEW HIGH --");
        }

        if let Some(t) = ctx.time_of_startup_from_low
            && t.instant.elapsed() < settings.monitor.max_allowed_startup_time
        {
            // We went high again before startup duration elapsed, this is a startup failure
            let result = notify::send_to_all(notifiers, settings, ctx, notify::MessageType::StartupFailed);

            if result.success != result.total {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "Failed to send some startup failure notifications: {}/{} succeeded",
                    result.success,
                    result.total
                );
            }
            return;
        }

        // We just randomly went HIGH for no reason, this is an alert
        let result = notify::send_to_all(notifiers, settings, ctx, notify::MessageType::Alert);

        if result.success != result.total {
            logging::tseprintln!(
                settings.disable_timestamps,
                "Failed to send some alert notifications: {}/{} succeeded",
                result.success,
                result.total
            );
        }
    } else {
        // We have been HIGH for a while, it may be time for a reminder
        let at_least_one_notifier_due_for_reminder = notifiers
            .iter()
            .any(|n| n.state().has_due_reminder(ctx.now.instant));

        if at_least_one_notifier_due_for_reminder {
            let result = notify::send_to_all(notifiers, settings, ctx, notify::MessageType::Reminder);

            if result.success != result.total {
                logging::tseprintln!(
                    settings.disable_timestamps,
                    "Failed to send some reminder notifications: {}/{} succeeded",
                    result.success,
                    result.total
                );
            }
        }
    }
}

fn end_loop(ctx: &mut context::Context, interval: std_time::Duration) {
    ctx.loop_iteration += 1;
    thread::sleep(interval);
}
