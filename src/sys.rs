// System

/// A trait for interacting with the system.
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/sys.rs).
pub trait System: Send + Sync {
    /// Open a URL in the default browser.
    ///
    /// **This is supported on `feature=browser` only.**
    #[cfg(feature = "browser")]
    fn open_url(&self, url: &str) -> std::io::Result<()>;
}

// DefaultSystem

/// Default implementation of [`System`](trait.System.html).
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/sys.rs).
pub struct DefaultSystem;

impl System for DefaultSystem {
    #[cfg(feature = "browser")]
    fn open_url(&self, url: &str) -> std::io::Result<()> {
        open::that(url)
    }
}

// MockSystem

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`System`](trait.System.html).
    ///
    /// **This is supported on `feature=mock` only.**
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/sys.rs).
    pub System {}

    impl System for System {
        #[cfg(feature = "browser")]
        fn open_url(&self, url: &str) -> std::io::Result<()>;
    }
}
