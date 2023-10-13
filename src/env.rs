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
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/env.rs).
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
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/env.rs).
pub struct DefaultEnv;

impl DefaultEnv {
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
        std::env::var(key)
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
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/env.rs).
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
