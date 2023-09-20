// Re-use

#[cfg(all(feature = "clock", feature = "mock"))]
pub use self::clock::MockClock;
#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock};
#[cfg(feature = "http-client")]
pub use self::http::{
    DefaultHttpClient, DefaultHttpResponse, HttpClient, HttpRequest, HttpResponse,
};
#[cfg(all(feature = "http-client", feature = "mock"))]
pub use self::http::{MockHttpClient, MockHttpResponse};
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
    sys::MockSystem,
};
pub use self::{
    env::{DefaultEnv, Env, EnvParseError, EnvParseResult},
    fs::{
        DefaultDirEntry, DefaultFileSystem, DefaultMetadata, DefaultPermissions, DefaultReadDir,
        DirEntry, FileSystem, Metadata, Permissions, ReadDir, VecReadDir,
    },
    sys::{DefaultSystem, System},
};

// Mods

#[cfg(feature = "clock")]
mod clock;
mod env;
mod fs;
#[cfg(feature = "http-client")]
mod http;
#[cfg(any(feature = "mock", test))]
mod mock;
mod sys;
#[cfg(feature = "uuid")]
mod uuid;
