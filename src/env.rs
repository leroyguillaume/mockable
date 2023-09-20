use std::{
    env::VarError,
    error::Error,
    ffi::OsString,
    fmt::{Display, Formatter, Result as FmtResult},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    path::PathBuf,
    str::FromStr,
};

use tracing::{trace, warn};

// Macros

macro_rules! implementation {
    ($ty:ident) => {
        implementation!($ty, $ty);
    };

    ($ident:ident, $ty:ident) => {
        fn $ident(&self, key: &str) -> Option<EnvParseResult<$ty>> {
            self.var(key)
        }
    };
}

// Types

pub type EnvParseResult<T> = Result<T, EnvParseError>;

// EnvParseError

/// An error that can occur when parsing an environment variable.
#[derive(Debug)]
pub struct EnvParseError(Box<dyn Error + Send + Sync>);

impl Display for EnvParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for EnvParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl From<Box<dyn Error + Send + Sync>> for EnvParseError {
    fn from(err: Box<dyn Error + Send + Sync>) -> Self {
        Self(err)
    }
}

// Env

/// A trait for getting environment variables.
///
/// # Examples
///
/// ```
/// use mockable::{DefaultEnv, Env, EnvParseResult, MockEnv};
///
/// fn get(env: &dyn Env) -> Option<EnvParseResult<u32>> {
///     env.u32("KEY")
/// }
///
/// std::env::set_var("KEY", "42");
///
/// // Default
/// let env = DefaultEnv::new();
/// let val = get(&env).unwrap().unwrap();
/// assert_eq!(val, 42);
///
/// // Mock
/// let mut env = MockEnv::new();
/// env
///     .expect_u32()
///     .returning(|_| Some(Ok(24)));
/// let val = get(&env).unwrap().unwrap();
/// assert_eq!(val, 24);
/// ```
pub trait Env: Send + Sync {
    /// Returns the value of the environment variable `key` as a `bool`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `bool`, an error is returned.
    fn bool(&self, key: &str) -> Option<EnvParseResult<bool>>;

    /// Returns the value of the environment variable `key` as a `char`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `char`, an error is returned.
    fn char(&self, key: &str) -> Option<EnvParseResult<char>>;

    /// Returns the value of the environment variable `key` as a `f32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `f32`, an error is returned.
    fn f32(&self, key: &str) -> Option<EnvParseResult<f32>>;

    /// Returns the value of the environment variable `key` as a `f64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `f64`, an error is returned.
    fn f64(&self, key: &str) -> Option<EnvParseResult<f64>>;

    /// Returns the value of the environment variable `key` as a `i8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i8`, an error is returned.
    fn i8(&self, key: &str) -> Option<EnvParseResult<i8>>;

    /// Returns the value of the environment variable `key` as a `i16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i16`, an error is returned.
    fn i16(&self, key: &str) -> Option<EnvParseResult<i16>>;

    /// Returns the value of the environment variable `key` as a `i32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i32`, an error is returned.
    fn i32(&self, key: &str) -> Option<EnvParseResult<i32>>;

    /// Returns the value of the environment variable `key` as a `i64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i64`, an error is returned.
    fn i64(&self, key: &str) -> Option<EnvParseResult<i64>>;

    /// Returns the value of the environment variable `key` as a `i128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i128`, an error is returned.
    fn i128(&self, key: &str) -> Option<EnvParseResult<i128>>;

    /// Returns the value of the environment variable `key` as a `IpAddr`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `IpAddr`, an error is returned.
    fn ip_addr(&self, key: &str) -> Option<EnvParseResult<IpAddr>>;

    /// Returns the value of the environment variable `key` as a `Ipv4Addr`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `Ipv4Addr`, an error is returned.
    fn ipv4_addr(&self, key: &str) -> Option<EnvParseResult<Ipv4Addr>>;

    /// Returns the value of the environment variable `key` as a `Ipv6Addr`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `Ipv6Addr`, an error is returned.
    fn ipv6_addr(&self, key: &str) -> Option<EnvParseResult<Ipv6Addr>>;

    /// Returns the value of the environment variable `key` as a `isize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `isize`, an error is returned.
    fn isize(&self, key: &str) -> Option<EnvParseResult<isize>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI8`, an error is returned.
    fn non_zero_i8(&self, key: &str) -> Option<EnvParseResult<NonZeroI8>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI16`, an error is returned.
    fn non_zero_i16(&self, key: &str) -> Option<EnvParseResult<NonZeroI16>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI32`, an error is returned.
    fn non_zero_i32(&self, key: &str) -> Option<EnvParseResult<NonZeroI32>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI64`, an error is returned.
    fn non_zero_i64(&self, key: &str) -> Option<EnvParseResult<NonZeroI64>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI128`, an error is returned.
    fn non_zero_i128(&self, key: &str) -> Option<EnvParseResult<NonZeroI128>>;

    /// Returns the value of the environment variable `key` as a `NonZeroIsize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroIsize`, an error is returned.
    fn non_zero_isize(&self, key: &str) -> Option<EnvParseResult<NonZeroIsize>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU8`, an error is returned.
    fn non_zero_u8(&self, key: &str) -> Option<EnvParseResult<NonZeroU8>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU16`, an error is returned.
    fn non_zero_u16(&self, key: &str) -> Option<EnvParseResult<NonZeroU16>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU32`, an error is returned.
    fn non_zero_u32(&self, key: &str) -> Option<EnvParseResult<NonZeroU32>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU64`, an error is returned.
    fn non_zero_u64(&self, key: &str) -> Option<EnvParseResult<NonZeroU64>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU128`, an error is returned.
    fn non_zero_u128(&self, key: &str) -> Option<EnvParseResult<NonZeroU128>>;

    /// Returns the value of the environment variable `key` as a `NonZeroUsize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroUsize`, an error is returned.
    fn non_zero_usize(&self, key: &str) -> Option<EnvParseResult<NonZeroUsize>>;

    /// Returns the value of the environment variable `key` as a `OsString`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `OsString`, an error is returned.
    fn os_string(&self, key: &str) -> Option<EnvParseResult<OsString>>;

    /// Returns the value of the environment variable `key` as a `PathBuf`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `PathBuf`, an error is returned.
    fn path_buf(&self, key: &str) -> Option<EnvParseResult<PathBuf>>;

    /// Returns the value of the environment variable `key` as a `String`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, an error is returned.
    /// See [`std::env::var`](https://doc.rust-lang.org/std/env/fn.var.html) for more details.
    fn raw(&self, key: &str) -> Result<String, VarError>;

    /// Returns the value of the environment variable `key` as a `SocketAddr`.
    ///
    /// If the environment variable is not present, `None` is returned.
    /// If the environment variable is not a valid `SocketAddr`, an error is returned.
    fn socket_addr(&self, key: &str) -> Option<EnvParseResult<SocketAddr>>;

    /// Returns the value of the environment variable `key` as a `SocketAddrV4`.
    ///
    /// If the environment variable is not present, `None` is returned.
    /// If the environment variable is not a valid `SocketAddrV4`, an error is returned.
    fn socket_addr_v4(&self, key: &str) -> Option<EnvParseResult<SocketAddrV4>>;

    /// Returns the value of the environment variable `key` as a `SocketAddrV6`.
    ///
    /// If the environment variable is not present, `None` is returned.
    /// If the environment variable is not a valid `SocketAddrV6`, an error is returned.
    fn socket_addr_v6(&self, key: &str) -> Option<EnvParseResult<SocketAddrV6>>;

    /// Returns the value of the environment variable `key` as a `String`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    fn string(&self, key: &str) -> Option<String>;

    /// Returns the value of the environment variable `key` as a `Vec<String>`.
    ///
    /// The value is split by `sep` and each item is trimmed.
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    fn strings(&self, key: &str, sep: &str) -> Option<Vec<String>>;

    /// Returns the value of the environment variable `key` as a `u8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u8`, an error is returned.
    fn u8(&self, key: &str) -> Option<EnvParseResult<u8>>;

    /// Returns the value of the environment variable `key` as a `u16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u16`, an error is returned.
    fn u16(&self, key: &str) -> Option<EnvParseResult<u16>>;

    /// Returns the value of the environment variable `key` as a `u32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u32`, an error is returned.
    fn u32(&self, key: &str) -> Option<EnvParseResult<u32>>;

    /// Returns the value of the environment variable `key` as a `u64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u64`, an error is returned.
    fn u64(&self, key: &str) -> Option<EnvParseResult<u64>>;

    /// Returns the value of the environment variable `key` as a `u128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u128`, an error is returned.
    fn u128(&self, key: &str) -> Option<EnvParseResult<u128>>;

    /// Returns the value of the environment variable `key` as a `usize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is a valid `usize`, an error is returned.
    fn usize(&self, key: &str) -> Option<EnvParseResult<usize>>;
}

// DefaultEnv

/// Default implementation of [`Env`](trait.Env.html).
pub struct DefaultEnv(Box<dyn VarFnWrapper>);

impl DefaultEnv {
    /// Creates a new `DefaultEnv`.
    pub fn new() -> Self {
        Self(Box::new(DefaultVarFnWrapper))
    }

    #[inline]
    fn var<E: Error + Send + Sync + 'static, T: FromStr<Err = E>>(
        &self,
        key: &str,
    ) -> Option<EnvParseResult<T>> {
        match self.raw(key) {
            Ok(val) => Some(val.parse::<T>().map_err(|err| EnvParseError(Box::new(err)))),
            Err(err) => match err {
                VarError::NotPresent => {
                    trace!(key, "environment variable is not defined");
                    None
                }
                VarError::NotUnicode(_) => {
                    warn!(details = %err, key, "reading environment variable failed");
                    None
                }
            },
        }
    }
}

impl Default for DefaultEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Env for DefaultEnv {
    implementation!(bool);

    implementation!(char);

    implementation!(f32);

    implementation!(f64);

    implementation!(i8);

    implementation!(i16);

    implementation!(i32);

    implementation!(i64);

    implementation!(i128);

    implementation!(ip_addr, IpAddr);

    implementation!(ipv4_addr, Ipv4Addr);

    implementation!(ipv6_addr, Ipv6Addr);

    implementation!(isize);

    implementation!(non_zero_i8, NonZeroI8);

    implementation!(non_zero_i16, NonZeroI16);

    implementation!(non_zero_i32, NonZeroI32);

    implementation!(non_zero_i64, NonZeroI64);

    implementation!(non_zero_i128, NonZeroI128);

    implementation!(non_zero_isize, NonZeroIsize);

    implementation!(non_zero_u8, NonZeroU8);

    implementation!(non_zero_u16, NonZeroU16);

    implementation!(non_zero_u32, NonZeroU32);

    implementation!(non_zero_u64, NonZeroU64);

    implementation!(non_zero_u128, NonZeroU128);

    implementation!(non_zero_usize, NonZeroUsize);

    implementation!(os_string, OsString);

    implementation!(path_buf, PathBuf);

    fn raw(&self, key: &str) -> Result<String, VarError> {
        trace!(key, "reading environment variable");
        self.0.var(key)
    }

    implementation!(socket_addr, SocketAddr);

    implementation!(socket_addr_v4, SocketAddrV4);

    implementation!(socket_addr_v6, SocketAddrV6);

    fn string(&self, key: &str) -> Option<String> {
        self.raw(key).ok()
    }

    fn strings(&self, key: &str, sep: &str) -> Option<Vec<String>> {
        self.string(key).map(|val| {
            let val = val.trim();
            if val.is_empty() {
                vec![]
            } else {
                val.split(sep).map(|item| item.trim().to_string()).collect()
            }
        })
    }

    implementation!(u8);

    implementation!(u16);

    implementation!(u32);

    implementation!(u64);

    implementation!(u128);

    implementation!(usize);
}

// MockEnv

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`Env`](trait.Env.html).
    ///
    /// **This is supported on `feature=mock` only.**
    pub Env {}

    impl Env for Env {
        fn bool(&self, key: &str) -> Option<EnvParseResult<bool>>;

        fn char(&self, key: &str) -> Option<EnvParseResult<char>>;

        fn f32(&self, key: &str) -> Option<EnvParseResult<f32>>;

        fn f64(&self, key: &str) -> Option<EnvParseResult<f64>>;

        fn i8(&self, key: &str) -> Option<EnvParseResult<i8>>;

        fn i16(&self, key: &str) -> Option<EnvParseResult<i16>>;

        fn i32(&self, key: &str) -> Option<EnvParseResult<i32>>;

        fn i64(&self, key: &str) -> Option<EnvParseResult<i64>>;

        fn i128(&self, key: &str) -> Option<EnvParseResult<i128>>;

        fn ip_addr(&self, key: &str) -> Option<EnvParseResult<IpAddr>>;

        fn ipv4_addr(&self, key: &str) -> Option<EnvParseResult<Ipv4Addr>>;

        fn ipv6_addr(&self, key: &str) -> Option<EnvParseResult<Ipv6Addr>>;

        fn isize(&self, key: &str) -> Option<EnvParseResult<isize>>;

        fn non_zero_i8(&self, key: &str) -> Option<EnvParseResult<NonZeroI8>>;

        fn non_zero_i16(&self, key: &str) -> Option<EnvParseResult<NonZeroI16>>;

        fn non_zero_i32(&self, key: &str) -> Option<EnvParseResult<NonZeroI32>>;

        fn non_zero_i64(&self, key: &str) -> Option<EnvParseResult<NonZeroI64>>;

        fn non_zero_i128(&self, key: &str) -> Option<EnvParseResult<NonZeroI128>>;

        fn non_zero_isize(&self, key: &str) -> Option<EnvParseResult<NonZeroIsize>>;

        fn non_zero_u8(&self, key: &str) -> Option<EnvParseResult<NonZeroU8>>;

        fn non_zero_u16(&self, key: &str) -> Option<EnvParseResult<NonZeroU16>>;

        fn non_zero_u32(&self, key: &str) -> Option<EnvParseResult<NonZeroU32>>;

        fn non_zero_u64(&self, key: &str) -> Option<EnvParseResult<NonZeroU64>>;

        fn non_zero_u128(&self, key: &str) -> Option<EnvParseResult<NonZeroU128>>;

        fn non_zero_usize(&self, key: &str) -> Option<EnvParseResult<NonZeroUsize>>;

        fn os_string(&self, key: &str) -> Option<EnvParseResult<OsString>>;

        fn path_buf(&self, key: &str) -> Option<EnvParseResult<PathBuf>>;

        fn raw(&self, key: &str) -> Result<String, VarError>;

        fn socket_addr(&self, key: &str) -> Option<EnvParseResult<SocketAddr>>;

        fn socket_addr_v4(&self, key: &str) -> Option<EnvParseResult<SocketAddrV4>>;

        fn socket_addr_v6(&self, key: &str) -> Option<EnvParseResult<SocketAddrV6>>;

        fn string(&self, key: &str) -> Option<String>;

        fn strings(&self, key: &str, sep: &str) -> Option<Vec<String>>;

        fn u8(&self, key: &str) -> Option<EnvParseResult<u8>>;

        fn u16(&self, key: &str) -> Option<EnvParseResult<u16>>;

        fn u32(&self, key: &str) -> Option<EnvParseResult<u32>>;

        fn u64(&self, key: &str) -> Option<EnvParseResult<u64>>;

        fn u128(&self, key: &str) -> Option<EnvParseResult<u128>>;

        fn usize(&self, key: &str) -> Option<EnvParseResult<usize>>;
    }
}

// VarFnWrapper

#[cfg_attr(test, mockall::automock)]
trait VarFnWrapper: Send + Sync {
    fn var(&self, key: &str) -> Result<String, VarError>;
}

// DefaultVarFnWrapper

struct DefaultVarFnWrapper;

impl VarFnWrapper for DefaultVarFnWrapper {
    fn var(&self, key: &str) -> Result<String, VarError> {
        std::env::var(key)
    }
}

// Tests

#[cfg(test)]
mod test {
    use std::path::Path;

    use mockall::predicate::eq;

    use crate::mock::Mock;

    use super::*;

    // Mods

    mod default_env {
        use super::*;

        // Macros

        macro_rules! test {
            ($ident:ident, $expected:expr) => {
                mod $ident {
                    use super::*;

                    #[test]
                    fn none_when_var_failed() {
                        let res = run(Mock::once(|| Err(VarError::NotPresent)), |env, key| {
                            env.$ident(key)
                        });
                        assert!(res.is_none());
                    }

                    #[test]
                    fn err_when_parse_failed() {
                        let res = run(Mock::once(|| Ok(String::new())), |env, key| env.$ident(key))
                            .unwrap();
                        assert!(res.is_err());
                    }

                    #[test]
                    fn value() {
                        let val = run(Mock::once(|| Ok($expected.to_string())), |env, key| {
                            env.$ident(key)
                        })
                        .unwrap()
                        .unwrap();
                        assert_eq!(val, $expected);
                    }
                }
            };
        }

        // Types

        type MockVar = Mock<Result<String, VarError>>;

        // run

        fn run<T: FromStr, F: Fn(&DefaultEnv, &str) -> Option<EnvParseResult<T>>>(
            var: MockVar,
            f: F,
        ) -> Option<EnvParseResult<T>> {
            let key = "KEY";
            let mut wrapper = MockVarFnWrapper::new();
            wrapper
                .expect_var()
                .with(eq(key))
                .returning(move |_| var.call());
            let env = DefaultEnv(Box::new(wrapper));
            f(&env, key)
        }

        // Mods

        test!(bool, true);

        test!(char, 'c');

        test!(f32, -3.4);

        test!(f64, -3.4);

        test!(i8, -1);

        test!(i16, -1);

        test!(i32, -1);

        test!(i64, -1);

        test!(i128, -1);

        test!(ip_addr, IpAddr::V4(Ipv4Addr::LOCALHOST));

        test!(ipv4_addr, Ipv4Addr::LOCALHOST);

        test!(ipv6_addr, Ipv6Addr::LOCALHOST);

        test!(isize, -1);

        test!(non_zero_i8, NonZeroI8::MIN);

        test!(non_zero_i16, NonZeroI16::MIN);

        test!(non_zero_i32, NonZeroI32::MIN);

        test!(non_zero_i64, NonZeroI64::MIN);

        test!(non_zero_i128, NonZeroI128::MIN);

        test!(non_zero_isize, NonZeroIsize::MIN);

        test!(non_zero_u8, NonZeroU8::MIN);

        test!(non_zero_u16, NonZeroU16::MIN);

        test!(non_zero_u32, NonZeroU32::MIN);

        test!(non_zero_u64, NonZeroU64::MIN);

        test!(non_zero_u128, NonZeroU128::MIN);

        test!(non_zero_usize, NonZeroUsize::MIN);

        mod os_string {
            use super::*;

            // run

            fn run(var: MockVar) -> Option<EnvParseResult<OsString>> {
                let key = "KEY";
                let mut wrapper = MockVarFnWrapper::new();
                wrapper
                    .expect_var()
                    .with(eq(key))
                    .returning(move |_| var.call());
                let env = DefaultEnv(Box::new(wrapper));
                env.os_string(key)
            }

            // Tests

            #[test]
            fn none_when_var_failed() {
                let val = run(Mock::once(|| Err(VarError::NotPresent)));
                assert!(val.is_none());
            }

            #[test]
            fn value() {
                let expected = "VALUE";
                let val = run(Mock::once(move || Ok(expected.to_string())))
                    .unwrap()
                    .unwrap();
                assert_eq!(val, expected);
            }
        }

        mod path_buf {
            use super::*;

            // run

            fn run(var: MockVar) -> Option<EnvParseResult<PathBuf>> {
                let key = "KEY";
                let mut wrapper = MockVarFnWrapper::new();
                wrapper
                    .expect_var()
                    .with(eq(key))
                    .returning(move |_| var.call());
                let env = DefaultEnv(Box::new(wrapper));
                env.path_buf(key)
            }

            // Tests

            #[test]
            fn none_when_var_failed() {
                let val = run(Mock::once(|| Err(VarError::NotPresent)));
                assert!(val.is_none());
            }

            #[test]
            fn value() {
                let expected = "VALUE";
                let val = run(Mock::once(move || Ok(expected.to_string())))
                    .unwrap()
                    .unwrap();
                assert_eq!(val, Path::new(expected));
            }
        }

        mod raw {
            use super::*;

            // run

            fn run(var: MockVar) -> Result<String, VarError> {
                let key = "KEY";
                let mut wrapper = MockVarFnWrapper::new();
                wrapper
                    .expect_var()
                    .with(eq(key))
                    .returning(move |_| var.call());
                let env = DefaultEnv(Box::new(wrapper));
                env.raw(key)
            }

            // Tests

            #[test]
            fn err_when_var_failed() {
                let res = run(Mock::once(|| Err(VarError::NotPresent)));
                assert!(res.is_err());
            }

            #[test]
            fn value() {
                let expected = "VALUE";
                let val = run(Mock::once(move || Ok(expected.to_string()))).unwrap();
                assert_eq!(val, expected);
            }
        }

        test!(
            socket_addr,
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080))
        );

        test!(socket_addr_v4, SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));

        test!(
            socket_addr_v6,
            SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0)
        );

        mod string {
            use super::*;

            // run

            fn run(var: MockVar) -> Option<String> {
                let key = "KEY";
                let mut wrapper = MockVarFnWrapper::new();
                wrapper
                    .expect_var()
                    .with(eq(key))
                    .returning(move |_| var.call());
                let env = DefaultEnv(Box::new(wrapper));
                env.string(key)
            }

            // Tests

            #[test]
            fn none_when_var_failed() {
                let val = run(Mock::once(|| Err(VarError::NotPresent)));
                assert!(val.is_none());
            }

            #[test]
            fn value() {
                let expected = "VALUE";
                let val = run(Mock::once(move || Ok(expected.to_string()))).unwrap();
                assert_eq!(val, expected);
            }
        }

        mod strings {
            use super::*;

            // run

            fn run(sep: &str, var: MockVar) -> Option<Vec<String>> {
                let key = "KEY";
                let mut wrapper = MockVarFnWrapper::new();
                wrapper
                    .expect_var()
                    .with(eq(key))
                    .returning(move |_| var.call());
                let env = DefaultEnv(Box::new(wrapper));
                env.strings(key, sep)
            }

            // Tests

            #[test]
            fn none_when_var_failed() {
                let val = run(",", Mock::once(|| Err(VarError::NotPresent)));
                assert!(val.is_none());
            }

            #[test]
            fn empty() {
                let expected: Vec<String> = vec![];
                let val = run(",", Mock::once(move || Ok(String::new()))).unwrap();
                assert_eq!(val, expected);
            }

            #[test]
            fn values() {
                let sep = ",";
                let expected = vec!["a".to_string(), "b".to_string()];
                let val = run(
                    sep,
                    Mock::once({
                        let expected = expected.clone();
                        move || Ok(expected.join(&format!(" {sep} ")))
                    }),
                )
                .unwrap();
                assert_eq!(val, expected);
            }
        }

        test!(u8, 1);

        test!(u16, 1);

        test!(u32, 1);

        test!(u64, 1);

        test!(u128, 1);

        test!(usize, 1);
    }
}
