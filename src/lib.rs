// Re-use

#[cfg(all(feature = "clock", feature = "mock"))]
pub use self::clock::MockClock;
#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock};
#[cfg(feature = "mock")]
pub use self::env::MockEnv;
pub use self::env::{DefaultEnv, Env, EnvParseError, EnvParseResult};
#[cfg(any(feature = "mock", test))]
pub use self::mock::Mock;
#[cfg(all(feature = "uuid", feature = "mock"))]
pub use self::uuid::MockUuidGenerator;
#[cfg(feature = "uuid")]
pub use self::uuid::{DefaultUuidGenerator, UuidGenerator};

// Mods

#[cfg(feature = "clock")]
mod clock;
mod env;
#[cfg(any(feature = "mock", test))]
mod mock;
#[cfg(feature = "uuid")]
mod uuid;
