//! Notification-sending logic.

mod dispatch;

pub use dispatch::{send_retries, send_to_all};

use std::fmt;
use std::time;

use crate::backend;
use crate::context;

/// Supertrait for notifiers that can send notifications and carry state.
pub trait StatefulNotifier: NotificationSender + StateCarrier {}

/// Blanket implementation of `StatefulNotifier` for any type that implements
/// both `NotificationSender` and `StateCarrier`.
impl<T: NotificationSender + StateCarrier> StatefulNotifier for T {}

/// A notifier that sends notifications, using a specific backend.
pub struct Notifier<B: backend::Backend> {
    /// State related to this notifier.
    pub state: NotifierState,

    /// The backend used to send notifications.
    backend: B,

    /// Whether this notifier is in dry-run mode, where it composes messages
    /// but only echoes them to the terminal without sending them.
    dry_run: bool,
}

impl<B: backend::Backend> Notifier<B> {
    /// Creates a new `Notifier`.
    ///
    /// # Parameters
    /// - `backend`: The backend to use for sending notifications.
    /// - `dry_run`: Whether to enable dry-run mode for this notifier.
    pub fn new(backend: B, dry_run: bool) -> Self {
        Self {
            state: NotifierState::default(),
            backend,
            dry_run,
        }
    }
}

impl<B: backend::Backend> StateCarrier for Notifier<B> {
    /// Accessor for the notifier's state.
    fn state(&self) -> &NotifierState {
        &self.state
    }

    /// Mutable accessor for the notifier's state.
    fn state_mut(&mut self) -> &mut NotifierState {
        &mut self.state
    }
}

impl<B: backend::Backend> NotificationSender for Notifier<B> {
    /// Accessor for the notifier's ID, which is the same as the backend's.
    fn id(&self) -> usize {
        self.backend.id()
    }

    /// Accessor for the notifier's name, which is the same as the backend's.
    fn name(&self) -> &str {
        self.backend.name()
    }

    /// Sends an alert notification using the notifier's backend.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing and sending the notification.
    ///
    /// # Returns
    /// A `SendResult` indicating the result of the send attempt.
    fn send_alert(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            let body = self.backend.compose_display(ctx, MessageType::Alert);
            return SendResult::Success(Some(body));
        }

        let body = self.backend.compose(ctx, MessageType::Alert);

        if body.is_empty() {
            return SendResult::NoMessage;
        }

        match self.backend.emit(ctx, &body, MessageType::Alert) {
            Ok(output) => SendResult::Success(output),
            Err(output) => SendResult::Failure(output),
        }
    }

    /// Sends a reminder notification using the notifier's backend.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing and sending the notification.
    ///
    /// # Returns
    /// A `SendResult` indicating the result of the send attempt.
    fn send_reminder(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            let body = self.backend.compose_display(ctx, MessageType::Reminder);
            return SendResult::Success(Some(body));
        }

        let body = self.backend.compose(ctx, MessageType::Reminder);

        if body.is_empty() {
            return SendResult::NoMessage;
        }

        match self.backend.emit(ctx, &body, MessageType::Reminder) {
            Ok(output) => SendResult::Success(output),
            Err(output) => SendResult::Failure(output),
        }
    }

    /// Sends a startup failure notification using the notifier's backend.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing and sending the notification.
    ///
    /// # Returns
    /// A `SendResult` indicating the result of the send attempt.
    fn send_startup_failed(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            let body = self
                .backend
                .compose_display(ctx, MessageType::StartupFailed);
            return SendResult::Success(Some(body));
        }

        let body = self.backend.compose(ctx, MessageType::StartupFailed);

        if body.is_empty() {
            return SendResult::NoMessage;
        }

        match self.backend.emit(ctx, &body, MessageType::StartupFailed) {
            Ok(output) => SendResult::Success(output),
            Err(output) => SendResult::Failure(output),
        }
    }

    /// Sends a startup success notification using the notifier's backend.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing and sending the notification.
    ///
    /// # Returns
    /// A `SendResult` indicating the result of the send attempt.
    fn send_startup_success(&mut self, ctx: &context::Context) -> SendResult {
        if self.dry_run {
            let body = self
                .backend
                .compose_display(ctx, MessageType::StartupSuccess);
            return SendResult::Success(Some(body));
        }

        let body = self.backend.compose(ctx, MessageType::StartupSuccess);

        if body.is_empty() {
            return SendResult::NoMessage;
        }

        match self.backend.emit(ctx, &body, MessageType::StartupSuccess) {
            Ok(output) => SendResult::Success(output),
            Err(output) => SendResult::Failure(output),
        }
    }

    /// Accessor for the notifier's stagger delay, which is the same as the backend's.
    fn stagger_delay(&self) -> time::Duration {
        self.backend.stagger_delay()
    }
}

/// State for a `Notifier`.
#[derive(Default)]
pub struct NotifierState {
    /// Information about the previous failed send attempt, if any.
    pub previous_failed_send: Option<FailedSendAttempt>,

    /// The time when the next reminder should be sent, if any.
    pub time_of_next_reminder: Option<time::Instant>,

    /// The time when the next retry should be attempted, if any.
    pub time_of_next_retry: Option<time::Instant>,

    /// The number of consecutive reminders that have been sent.
    pub reminder_count: u32,

    /// The number of consecutive retries that have been attempted.
    pub retry_count: u32,
}

impl NotifierState {
    /// Handles a successful startup notification.
    ///
    /// Resets all state related to failed sends and reminders, since a
    /// successful startup implies a clean slate going forward.
    pub fn on_startup_success(&mut self) {
        self.previous_failed_send = None;
        self.time_of_next_reminder = None;
        self.time_of_next_retry = None;
        self.reminder_count = 0;
        self.retry_count = 0;
    }

    /// Handles a successful reminder notification.
    pub fn on_reminder_success(&mut self) {
        self.previous_failed_send = None;
        self.time_of_next_retry = None;
        self.reminder_count += 1;
        self.retry_count = 0;
    }

    /// Handles a failed send attempt.
    ///
    /// Information about this failed attempt is stored in the
    /// `previous_failed_send` field.
    ///
    /// # Parameters
    /// - `ctx`: The context of this failed send attempt.
    /// - `message_type`: The type of message that failed to send.
    pub fn on_failure(&mut self, ctx: &context::Context, message_type: MessageType) {
        let failed_send = FailedSendAttempt::new(message_type, ctx);
        self.previous_failed_send = Some(failed_send);
    }

    /// Bumps the time of the next reminder, based on the number of reminders
    /// that have already been sent.
    ///
    /// This is on a growing scale to space out reminders more and more.
    pub fn bump_time_of_next_reminder(&mut self) {
        const HOUR: time::Duration = time::Duration::from_secs(3600);

        let multiplier = match self.reminder_count {
            0 => 3,
            1 => 6,
            2 => 12,
            3 | 4 => 24,
            5 | 6 => 48,
            7 => 96,
            _ => 168,
        };

        self.time_of_next_reminder = Some(time::Instant::now() + multiplier * HOUR);
    }

    /// Bumps the time of the next retry, based on the number of retries that
    /// have already been attempted.
    ///
    /// This is on a growing scale to space out retries more and more.
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

    /// Checks if a reminder is due, based on the passed time.
    ///
    /// # Parameters
    /// - `now`: The time to check against the time of the next reminder.
    pub fn has_due_reminder(&self, now: time::Instant) -> bool {
        self.time_of_next_reminder.is_some_and(|t| t <= now)
    }

    /// Checks if a retry is due, based on the passed time.
    ///
    /// # Parameters
    /// - `now`: The time to check against the time of the next retry.
    pub fn has_due_retry(&self, now: time::Instant) -> bool {
        self.time_of_next_retry.is_some_and(|t| t <= now)
    }
}

/// Trait of something that can send notifications.
pub trait NotificationSender {
    /// Accessor for the notifier's ID.
    fn id(&self) -> usize;

    /// Accessor for the notifier's name.
    fn name(&self) -> &str;

    /// Composes an alert notification.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing the notification.
    ///
    /// # Returns
    /// A `SendResult` of the send attempt.
    fn send_alert(&mut self, ctx: &context::Context) -> SendResult;

    /// Composes a reminder notification.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing the notification.
    ///
    /// # Returns
    /// A `SendResult` of the send attempt.
    fn send_reminder(&mut self, ctx: &context::Context) -> SendResult;

    /// Composes a startup failure notification.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing the notification.
    ///
    /// # Returns
    /// A `SendResult` of the send attempt.
    fn send_startup_failed(&mut self, ctx: &context::Context) -> SendResult;

    /// Composes a startup success notification.
    ///
    /// # Parameters
    /// - `ctx`: The context to use when composing the notification.
    ///
    /// # Returns
    /// A `SendResult` of the send attempt.
    fn send_startup_success(&mut self, ctx: &context::Context) -> SendResult;

    /// Accessor for the stagger delay of the notifier.
    fn stagger_delay(&self) -> time::Duration;
}

/// Trait of a notifier that can carry state.
pub trait StateCarrier {
    /// Accessor for the carried state.
    fn state(&self) -> &NotifierState;

    /// Mutable accessor for the carried state.
    fn state_mut(&mut self) -> &mut NotifierState;
}

/// Result of a send attempt.
pub enum SendResult {
    /// The send attempt was successful, with an optional output string.
    Success(Option<String>),

    /// The send attempt failed, with an output string describing the failure.
    Failure(String),

    /// The send attempt was not made due to the message the backend rendered
    /// ended up empty.
    NoMessage,

    /// The send attempt was postponed due to spacing logic.
    TryAgainLater,
}

/// Type of message being sent as a notification.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageType {
    /// An alert notification, when the input source implies an error state.
    Alert,

    /// A reminder notification, when the situation since the last alert has
    /// not been resolved.
    Reminder,

    /// A startup failure notification, when the pellets burner attempted to
    /// start up but failed to do so.
    StartupFailed,

    /// A startup success notification, when the pellets burner started up
    /// successfully.
    StartupSuccess,
}

impl fmt::Display for MessageType {
    /// Formats a `MessageType` variants as lowercase strings.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MessageType::Alert => "alert",
            MessageType::Reminder => "reminder",
            MessageType::StartupFailed => "startup_failed",
            MessageType::StartupSuccess => "startup_success",
        };
        write!(f, "{s}")
    }
}

/// A failed send attempt, containing information needed to retry the send.
#[derive(Clone)]
pub struct FailedSendAttempt {
    /// The type of message that failed to send.
    pub message_type: MessageType,

    /// The context of the main loop at the time of the failed send attempt.
    pub ctx: context::Context,
}

impl FailedSendAttempt {
    /// Creates a new `FailedSendAttempt`.
    ///
    /// # Parameters
    /// - `message_type`: The type of message that failed to send.
    /// - `ctx`: The context of the main loop at the time of the failed send attempt.
    pub fn new(message_type: MessageType, ctx: &context::Context) -> Self {
        Self {
            message_type,
            ctx: ctx.clone(),
        }
    }
}
