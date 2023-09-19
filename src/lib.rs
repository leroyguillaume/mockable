// Re-use

#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock, MockClock};
#[cfg(feature = "uuid")]
pub use self::uuid::{DefaultUuidGenerator, MockUuidGenerator, UuidGenerator};

// Mods

#[cfg(feature = "clock")]
mod clock;
#[cfg(feature = "uuid")]
mod uuid;
