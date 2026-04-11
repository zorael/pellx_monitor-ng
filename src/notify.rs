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
        let message = self.backend.compose_alert(ctx);

        if self.dry_run {
            println!("DRY RUN: send_alert");
            println!("Message:\n{message}");
            return SendResult::Success;
        }

        match self.backend.emit(ctx, &message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }

    fn send_reminder(&mut self, ctx: &context::Context) -> SendResult {
        let message = self.backend.compose_reminder(ctx);

        if self.dry_run {
            println!("DRY RUN: send_reminder");
            println!("Message:\n{message}");
            return SendResult::Success;
        }

        match self.backend.emit(ctx, &message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }

    fn send_startup_failed(&mut self, ctx: &context::Context) -> SendResult {
        let message = self.backend.compose_startup_failed(ctx);

        if self.dry_run {
            println!("DRY RUN: send_startup_failed");
            println!("Message:\n{message}");
            return SendResult::Success;
        }

        match self.backend.emit(ctx, &message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }

    fn send_startup_success(&mut self, ctx: &context::Context) -> SendResult {
        let message = self.backend.compose_startup_success(ctx);

        if self.dry_run {
            println!("DRY RUN: send_startup_success");
            println!("Message:\n{message}");
            return SendResult::Success;
        }

        match self.backend.emit(ctx, &message) {
            Ok(_) => SendResult::Success,
            Err(_) => SendResult::Failure,
        }
    }
}

#[derive(Clone)]
pub struct NotifierState {
    pub previous_failed_send: Option<context::FailedSendAttempt>,
    pub time_of_next_reminder: Option<time::Instant>,
    pub time_of_next_retry: Option<time::Instant>,
    pub reminder_count: u32,
    pub retry_count: u32,
}

impl NotifierState {
    pub fn new() -> Self {
        Self {
            previous_failed_send: None,
            time_of_next_reminder: None,
            time_of_next_retry: None,
            reminder_count: 0,
            retry_count: 0,
        }
    }

    pub fn reset(&mut self) {
        //self.previous_failed_context = None;
        self.time_of_next_reminder = None;
        self.time_of_next_retry = None;
        self.reminder_count = 0;
        self.retry_count = 0;
    }

    pub fn on_reminder_success(&mut self) {
        //self.previous_failed_context = None;
        self.time_of_next_retry = None;
        self.reminder_count += 1;
    }

    pub fn on_failure(&mut self, ctx: &context::Context, message_type: &context::MessageType) {
        let failed_send = context::FailedSendAttempt::new(message_type, ctx);
        self.previous_failed_send = Some(failed_send);
    }

    pub fn bump_time_of_next_reminder(&mut self) {
        const HOUR: time::Duration = time::Duration::from_secs(3600);

        let multiplier = match self.reminder_count {
            0 => 3,
            1 => 6,
            2 => 12,
            3 => 24,
            4 => 24,
            5 => 48,
            6 => 48,
            7 => 96,
            _ => 168,
        };

        self.time_of_next_reminder = Some(time::Instant::now() + multiplier * HOUR);
    }

    pub fn bump_time_of_next_retry(&mut self) {
        const SECOND: time::Duration = time::Duration::from_secs(1);

        let multiplier = match self.retry_count {
            0..=3 => 5,
            4..=6 => 15,
            7..=9 => 30,
            10..=12 => 60,
            _ => 120,
        };

        self.time_of_next_retry = Some(time::Instant::now() + multiplier * SECOND);
    }
}

pub trait NotificationSender {
    #[allow(unused)]
    fn id(&self) -> usize;
    fn name(&self) -> &str;
    fn send_alert(&mut self, ctx: &context::Context) -> SendResult;
    fn send_reminder(&mut self, ctx: &context::Context) -> SendResult;
    fn send_startup_failed(&mut self, ctx: &context::Context) -> SendResult;
    fn send_startup_success(&mut self, ctx: &context::Context) -> SendResult;
}

pub trait StateCarrier {
    fn state(&self) -> &NotifierState;
    fn state_mut(&mut self) -> &mut NotifierState;
}

pub enum SendResult {
    Success,
    Failure,
    TryAgainLater,
}

#[derive(Default)]
pub struct NotificationResult {
    #[allow(unused)]
    pub total: usize,
    pub success: usize,
    pub failure: usize,
    pub try_again_later: usize,
}
