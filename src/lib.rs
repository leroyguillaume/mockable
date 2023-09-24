// Re-use

#[cfg(all(feature = "clock", feature = "mock"))]
pub use self::clock::MockClock;
#[cfg(feature = "clock")]
pub use self::clock::{Clock, DefaultClock};
#[cfg(all(feature = "cmd", feature = "mock"))]
pub use self::cmd::MockCommandRunner;
#[cfg(feature = "cmd")]
pub use self::cmd::{Command, CommandOutput, CommandRunner, DefaultCommandRunner};
#[cfg(feature = "http-client")]
pub use self::http::{
    DefaultHttpClient, DefaultHttpResponse, HttpClient, HttpRequest, HttpResponse,
};
#[cfg(all(feature = "http-client", feature = "mock"))]
pub use self::http::{MockHttpClient, MockHttpResponse};
#[cfg(any(feature = "mock", test))]
pub use self::mock::Mock;
#[cfg(feature = "postgres")]
pub use self::postgres::{
    transactional, DefaultPostgresClient, DefaultPostgresPool, DefaultPostgresTransaction,
    PostgresClient, PostgresError, PostgresPool, PostgresResult, PostgresTransaction,
    ToPostgresClient,
};
#[cfg(all(feature = "postgres", feature = "mock"))]
pub use self::postgres::{MockPostgresClient, MockPostgresPool, MockPostgresTransaction};
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
#[cfg(feature = "cmd")]
mod cmd;
mod env;
mod fs;
#[cfg(feature = "http-client")]
mod http;
#[cfg(any(feature = "mock", test))]
mod mock;
#[cfg(feature = "postgres")]
mod postgres;
mod sys;
#[cfg(feature = "uuid")]
mod uuid;
