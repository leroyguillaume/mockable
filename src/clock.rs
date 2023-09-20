use chrono::{DateTime, Local, Utc};

// Clock

/// A trait for getting the current time.
///
/// **This is supported on `feature=clock` only.**
/// # Examples
///
/// ```
/// use chrono::{DateTime, Duration, Utc};
/// use mockable::{Clock, DefaultClock, MockClock};
///
/// fn now(clock: &dyn Clock) -> DateTime<Utc> {
///    clock.utc()
/// }
///
/// // Default
/// let time = now(&DefaultClock);
///
/// // Mock
/// let expected = Utc::now();
/// let mut clock = MockClock::new();
/// clock
///     .expect_utc()
///     .returning(move || expected);
/// let time = now(&clock);
/// assert_eq!(time, expected);
/// ```
pub trait Clock: Send + Sync {
    /// Returns the current time in the local timezone.
    fn local(&self) -> DateTime<Local>;

    /// Returns the current time in UTC.
    fn utc(&self) -> DateTime<Utc>;
}

// DefaultClock

/// Default implementation of `Clock`.
///
/// **This is supported on `feature=clock` only.**
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn local(&self) -> DateTime<Local> {
        Local::now()
    }

    fn utc(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

// MockClock

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of `Clock`.
    ///
    /// **This is supported on `feature=clock,mock` only.**
    pub Clock {}

    impl Clock for Clock {
        fn local(&self) -> DateTime<Local>;
        fn utc(&self) -> DateTime<Utc>;
    }
}
