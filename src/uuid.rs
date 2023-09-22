use uuid::Uuid;

// UuidGenerator

/// A trait for generating UUIDs.
///
/// **This is supported on `feature=uuid` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/uuid.rs).
pub trait UuidGenerator: Send + Sync {
    /// Generates a new UUID V4.
    fn generate_v4(&self) -> Uuid;
}

// DefaultUuidGenerator

/// Default implementation of [`UuidGenerator`](trait.UuidGenerator.html).
///
/// **This is supported on `feature=uuid` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/uuid.rs).
pub struct DefaultUuidGenerator;

impl UuidGenerator for DefaultUuidGenerator {
    fn generate_v4(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// MockClock

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`UuidGenerator`](trait.UuidGenerator.html).
    ///
    /// **This is supported on `feature=uuid,mock` only.**
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/uuid.rs).
    pub UuidGenerator {}

    impl UuidGenerator for UuidGenerator {
        fn generate_v4(&self) -> Uuid;
    }
}
