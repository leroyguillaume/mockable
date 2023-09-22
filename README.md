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

[Example](examples/clock.rs).

## Command Runner

The [`CommandRunner`](https://docs.rs/mockable/latest/mockable/trait.CommandRunner.html) trait provides a way to mock the execution of commands.

**Note:** This trait is only available when the `cmd` feature is enabled.

[Example](examples/cmd.rs).

## Env

The [`Env`](https://docs.rs/mockable/latest/mockable/trait.Env.html) trait provides a way to mock the environment variables.

[Example](examples/env.rs).

## File System

The [`FileSystem`](https://docs.rs/mockable/latest/mockable/trait.FileSystem.html) trait provides a way to mock the file system operations.

[Example](examples/fs.rs).

## HTTP Client

The [`HttpClient`](https://docs.rs/mockable/latest/mockable/trait.HttpClient.html) trait provides a way to mock the HTTP client.

**Note:** This trait is only available when the `http` feature is enabled.

[Example](examples/http.rs).

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

[Example](examples/sys.rs).

## UUID Generator

The [`UuidGenerator`](https://docs.rs/mockable/latest/mockable/trait.UuidGenerator.html) trait provides a way to mock the UUID generator.

**Note:** This trait is only available when the `uuid` feature is enabled.

[Example](examples/uuid.rs).
