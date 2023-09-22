# mockable

This crate provides usefull traits to make easier to mock your code using [`mockall`](https://github.com/asomers/mockall) crate.

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
mockable = { version = "0.1", features = [ ... ] }

[dev-dependencies]
mockable = { version = "0.1", features = ["mock"] }
```

## Documentation

[Documentation](https://docs.rs/mockable/latest/mockable/)

## Clock

The [`Clock`](https://docs.rs/mockable/latest/mockable/trait.Clock.html) trait provides a way to mock the current time.

**Note:** This trait is only available when the `clock` feature is enabled.

```rust
use chrono::{DateTime, Duration, Utc};
use mockable::{Clock, DefaultClock, MockClock};

fn now(clock: &dyn Clock) -> DateTime<Utc> {
   clock.utc()
}

// Default
let time = now(&DefaultClock);

// Mock
let expected = Utc::now();
let mut clock = MockClock::new();
clock
    .expect_utc()
    .returning(move || expected);
let time = now(&clock);
assert_eq!(time, expected);
```

## Command Runner

The [`CommandRunner`](https://docs.rs/mockable/latest/mockable/trait.CommandRunner.html) trait provides a way to mock the execution of commands.

**Note:** This trait is only available when the `cmd` feature is enabled.

```rust
use std::io::Result;

use mockall::predicate::eq;
use mockable::{Command, CommandOutput, CommandRunner, DefaultCommandRunner, MockCommandRunner};

async fn run(cmd: Command, runner: &dyn CommandRunner) -> Result<CommandOutput> {
    runner.run(cmd).await
}

tokio_test::block_on(async {
    let cmd = Command {
        args: vec!["-n".to_string(), "Hello world!".to_string()],
        cwd: None,
        env: None,
        gid: None,
        program: "echo".to_string(),
        uid: None,
    };

    // Default
    let runner = DefaultCommandRunner;
    let outputs = run(cmd.clone(), &runner).await.unwrap();
    assert_eq!(outputs.code, Some(0));
    assert_eq!(outputs.stdout, "Hello world!".as_bytes().to_vec());

    // Mock
    let expected = CommandOutput {
        code: Some(0),
        stderr: vec![],
        stdout: "Hello world!".as_bytes().to_vec(),
    };
    let mut runner = MockCommandRunner::new();
    runner
        .expect_run()
        .with(eq(cmd.clone()))
        .returning({
            let expected = expected.clone();
            move |_| Ok(expected.clone())
        });
    let output = run(cmd, &runner).await.unwrap();
    assert_eq!(output, expected);
});
```

## Env

The [`Env`](https://docs.rs/mockable/latest/mockable/trait.Env.html) trait provides a way to mock the environment variables.

```rust
use mockable::{DefaultEnv, Env, EnvParseResult, MockEnv};

fn get(env: &dyn Env) -> Option<EnvParseResult<u32>> {
    env.u32("KEY")
}

std::env::set_var("KEY", "42");

// Default
let env = DefaultEnv::new();
let val = get(&env).unwrap().unwrap();
assert_eq!(val, 42);

// Mock
let mut env = MockEnv::new();
env
    .expect_u32()
    .returning(|_| Some(Ok(24)));
let val = get(&env).unwrap().unwrap();
assert_eq!(val, 24);
```

## File System

The [`FileSystem`](https://docs.rs/mockable/latest/mockable/trait.FileSystem.html) trait provides a way to mock the file system operations.

```rust
use std::{io::Result, path::Path};

use mockall::predicate::eq;
use mockable::{DefaultFileSystem, FileSystem, Metadata, MockFileSystem, MockMetadata};

fn get_metadata(path: &Path, fs: &dyn FileSystem) -> Result<Box<dyn Metadata>> {
    fs.metadata(path)
}

// Default
let metadata = get_metadata(Path::new("/"), &DefaultFileSystem).unwrap();
assert!(metadata.is_dir());

// Mock
let mut fs = MockFileSystem::new();
fs
    .expect_metadata()
    .with(eq(Path::new("/")))
    .returning(|_| {
        let mut metadata = MockMetadata::new();
        metadata
            .expect_is_dir()
            .returning(|| true);
        Ok(Box::new(metadata))
    });
let metadata = get_metadata(Path::new("/"), &fs).unwrap();
assert!(metadata.is_dir());
```

## HTTP Client

The [`HttpClient`](https://docs.rs/mockable/latest/mockable/trait.HttpClient.html) trait provides a way to mock the HTTP client.

**Note:** This trait is only available when the `http` feature is enabled.

```rust
use mockall::predicate::eq;
use mockable::{DefaultHttpClient, HttpClient, HttpRequest, HttpResponse, MockHttpClient, MockHttpResponse};
use reqwest::{Method, Result, StatusCode};

async fn send(req: HttpRequest, client: &dyn HttpClient) -> Result<Box<dyn HttpResponse>> {
    client.send(req).await
}

tokio_test::block_on(async {
    let req = HttpRequest {
        headers: Default::default(),
        method: Method::GET,
        query: Default::default(),
        url: "https://google.com".to_string(),
    };

    // Default
    let client = DefaultHttpClient;
    let resp = send(req.clone(), &client).await.unwrap();
    assert!(resp.status().is_success());

    // Mock
    let mut client = MockHttpClient::new();
    client
        .expect_send()
        .with(eq(req.clone()))
        .returning(|_| {
            let mut resp = MockHttpResponse::new();
            resp
                .expect_status()
                .returning(|| StatusCode::OK);
            Ok(Box::new(resp))
        });
    let resp = send(req, &client).await.unwrap();
    assert!(resp.status().is_success());
});
```

## Mock

The [`Mock`](https://docs.rs/mockable/latest/mockable/struct.Mock.html) trait provides a way to mock a function.

```rust
use mockable::Mock;
use mockall::automock;

// Never
let mock: Mock<()> = Mock::never();
// fist call will panic

// Once
let mock = Mock::once(|| 42);
assert_eq!(mock.call(), 42);

// Several
let mock = Mock::with(vec![
    Box::new(|| 1),
    Box::new(|| 2),
    Box::new(|| 3)]
);
assert_eq!(mock.call(), 1);
assert_eq!(mock.call(), 2);
assert_eq!(mock.call(), 3);
// next call will panic

// Always
let mock = Mock::always(|idx| idx);
assert_eq!(mock.call(), 0);
assert_eq!(mock.call(), 1);
assert_eq!(mock.call(), 2);
// next call will never panic

// with mockall
#[automock]
trait MyTrait {
    fn foo(&self) -> &'static str;
}

let mock = Mock::once(move || "bar");
let mut mymock = MockMyTrait::new();
mymock
    .expect_foo()
    .returning({
        let mock = mock.clone();
        move || mock.call()
    });
assert_eq!(mymock.foo(), "bar");
assert_eq!(mock.count(), 1);
```

## System

The [`System`](https://docs.rs/mockable/latest/mockable/trait.System.html) trait provides a way to mock the system.

## UUID Generator

The [`UuidGenerator`](https://docs.rs/mockable/latest/mockable/trait.UuidGenerator.html) trait provides a way to mock the UUID generator.

**Note:** This trait is only available when the `uuid` feature is enabled.

```rust
use mockable::{DefaultUuidGenerator, MockUuidGenerator, UuidGenerator};
use uuid::Uuid;

fn generate(generator: &dyn UuidGenerator) -> Uuid {
   generator.generate_v4()
}

// Default
let uuid = generate(&DefaultUuidGenerator);

// Mock
let expected = Uuid::new_v4();
let mut generator = MockUuidGenerator::new();
generator
    .expect_generate_v4()
    .returning(move || expected);
let uuid = generate(&generator);
assert_eq!(uuid, expected);
```
