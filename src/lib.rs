// Re-use

#[cfg(all(feature = "clock", feature = "mock"))]
pub use self::clock::MockClock;
#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock};
#[cfg(all(feature = "cmd", feature = "mock"))]
pub use self::cmd::MockCommandRunner;
#[cfg(feature = "cmd")]
pub use self::cmd::{Command, CommandOutput, CommandRunner, DefaultCommandRunner};
#[cfg(any(feature = "mock", test))]
pub use self::mock::Mock;
#[cfg(all(feature = "uuid", feature = "mock"))]
pub use self::uuid::MockUuidGenerator;
#[cfg(feature = "uuid")]
pub use self::uuid::{DefaultUuidGenerator, UuidGenerator};
#[cfg(feature = "mock")]
pub use self::{env::MockEnv, sys::MockSystem};
pub use self::{
    env::{DefaultEnv, Env},
    sys::{DefaultSystem, System},
};

// Mods

#[cfg(feature = "clock")]
mod clock;
#[cfg(feature = "cmd")]
mod cmd;
mod env;
#[cfg(any(feature = "mock", test))]
mod mock;
mod sys;
#[cfg(feature = "uuid")]
mod uuid;
