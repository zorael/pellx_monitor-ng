use std::time;

use crate::backend;
use crate::context;

pub trait StatefulNotifier: NotificationSender + StateCarrier {}

impl<T: NotificationSender + StateCarrier> StatefulNotifier for T {}

pub struct Notifier<B: backend::Backend> {
    pub state: NotifierState,
    backend: B,
    dry_run: bool,
}

impl<B: backend::Backend> Notifier<B> {
    pub fn new(backend: B, dry_run: bool) -> Self {
        Self {
            state: NotifierState::new(),
            backend,
            dry_run,
        }
    }
}

impl<B: backend::Backend> StateCarrier for Notifier<B> {
    fn state(&self) -> &NotifierState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut NotifierState {
        &mut self.state
    }
}

impl<B: backend::Backend> NotificationSender for Notifier<B> {
    fn id(&self) -> usize {
        self.backend.id()
    }

    fn name(&self) -> &str {
        self.backend.name()
    }

    fn send_alert(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            println!("DRY RUN: send_alert");
            return SendResult::Success;
        }

        let message = self.backend.compose_alert(ctx);

        match self.backend.emit(message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }

    fn send_reminder(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            println!("DRY RUN: send_reminder");
            return SendResult::Success;
        }

        let message = self.backend.compose_reminder(ctx);

        match self.backend.emit(message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }

    fn send_startup_failed(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            println!("DRY RUN: send_startup_failed");
            return SendResult::Success;
        }

        let message = self.backend.compose_startup_failed(ctx);

        match self.backend.emit(message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }
}

pub struct NotifierState {
    pub time_of_last_alert: Option<time::Instant>,
    pub time_of_last_reminder: Option<time::Instant>,
    pub time_of_last_retry: Option<time::Instant>,
    pub previous_failed_context: Option<context::Context>,
    pub retry_count: u32,
}

impl NotifierState {
    pub fn new() -> Self {
        Self {
            time_of_last_alert: None,
            time_of_last_reminder: None,
            time_of_last_retry: None,
            previous_failed_context: None,
            retry_count: 0,
        }
    }

    pub fn next_reminder_is_due(&self) -> bool {
        match (self.time_of_last_reminder, self.time_of_last_alert) {
            (Some(reminder), Some(alert)) => {
                reminder.elapsed() >= time::Duration::from_secs(5)
                    && alert.elapsed() >= time::Duration::from_secs(5)
            }
            (Some(reminder), None) => reminder.elapsed() >= time::Duration::from_secs(5),
            (None, Some(alert)) => alert.elapsed() >= time::Duration::from_secs(5),
            (None, None) => true,
        }
    }

    pub fn next_retry_is_due(&self) -> bool {
        match self.time_of_last_retry {
            Some(t) => t.elapsed() >= time::Duration::from_secs(5),
            None => true,
        }
    }
}

pub trait NotificationSender {
    fn id(&self) -> usize;
    fn name(&self) -> &str;
    fn send_alert(&mut self, ctx: &context::Context) -> SendResult;
    fn send_reminder(&mut self, ctx: &context::Context) -> SendResult;
    fn send_startup_failed(&mut self, ctx: &context::Context) -> SendResult;
}

pub trait StateCarrier {
    fn state(&self) -> &NotifierState;
    fn state_mut(&mut self) -> &mut NotifierState;
}

pub enum SendResult {
    Success,
    Failure,
}
