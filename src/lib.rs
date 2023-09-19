// Re-use

#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock, MockClock};
#[cfg(feature = "mock")]
pub use self::mock::Mock;
#[cfg(feature = "uuid")]
pub use self::uuid::{DefaultUuidGenerator, MockUuidGenerator, UuidGenerator};

// Mods

#[cfg(feature = "clock")]
mod clock;
#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "uuid")]
mod uuid;
