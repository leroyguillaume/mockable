use chrono::{DateTime, Local, Utc};

// Clock

/// A trait for getting the current time.
///
/// **This is supported on `feature=clock` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/clock.rs).
pub trait Clock: Send + Sync {
    /// Returns the current time in the local timezone.
    fn local(&self) -> DateTime<Local>;

    /// Returns the current time in UTC.
    fn utc(&self) -> DateTime<Utc>;
}

// DefaultClock

/// Default implementation of [`Clock`](trait.Clock.html).
///
/// **This is supported on `feature=clock` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/clock.rs).
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
    /// `mockall` implementation of [`Clock`](trait.Clock.html).
    ///
    /// **This is supported on `feature=clock,mock` only.**
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/clock.rs).
    pub Clock {}

    impl Clock for Clock {
        fn local(&self) -> DateTime<Local>;
        fn utc(&self) -> DateTime<Utc>;
    }
}
