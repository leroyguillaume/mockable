use uuid::Uuid;

// UuidGenerator

/// A trait for generating UUIDs.
///
/// **This is supported on `feature=uuid` only.**
///
/// # Examples
///
/// ```
/// use mockable::{DefaultUuidGenerator, MockUuidGenerator, UuidGenerator};
/// use uuid::Uuid;
///
/// fn generate(generator: &dyn UuidGenerator) -> Uuid {
///    generator.generate_v4()
/// }
///
/// // Default
/// let uuid = generate(&DefaultUuidGenerator);
///
/// // Mock
/// let expected = Uuid::new_v4();
/// let mut generator = MockUuidGenerator::new();
/// generator
///     .expect_generate_v4()
///     .returning(move || expected);
/// let uuid = generate(&generator);
/// assert_eq!(uuid, expected);
/// ```
pub trait UuidGenerator: Send + Sync {
    /// Generates a new UUID V4.
    fn generate_v4(&self) -> Uuid;
}

// DefaultUuidGenerator

/// Default implementation of [`UuidGenerator`](trait.UuidGenerator.html).
///
/// **This is supported on `feature=uuid` only.**
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
    pub UuidGenerator {}

    impl UuidGenerator for UuidGenerator {
        fn generate_v4(&self) -> Uuid;
    }
}
