// Re-use

#[cfg(all(feature = "clock", feature = "mock"))]
pub use self::clock::MockClock;
#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock};
#[cfg(any(feature = "mock", test))]
pub use self::mock::Mock;
#[cfg(all(feature = "uuid", feature = "mock"))]
pub use self::uuid::MockUuidGenerator;
#[cfg(feature = "uuid")]
pub use self::uuid::{DefaultUuidGenerator, UuidGenerator};
#[cfg(feature = "mock")]
pub use self::{
    env::MockEnv,
    fs::{MockDirEntry, MockFileSystem, MockMetadata, MockPermissions},
};
pub use self::{
    env::{DefaultEnv, Env, EnvParseError, EnvParseResult},
    fs::{
        DefaultDirEntry, DefaultFileSystem, DefaultMetadata, DefaultPermissions, DefaultReadDir,
        DirEntry, FileSystem, Metadata, Permissions, ReadDir, VecReadDir,
    },
};

// Mods

#[cfg(feature = "clock")]
mod clock;
mod env;
mod fs;
#[cfg(any(feature = "mock", test))]
mod mock;
#[cfg(feature = "uuid")]
mod uuid;
