use std::{
    char::ParseCharError,
    env::VarError,
    error::Error,
    ffi::OsString,
    net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, ParseFloatError,
        ParseIntError,
    },
    path::PathBuf,
    str::{FromStr, ParseBoolError},
};

use tracing::{trace, warn};

// Macros

macro_rules! parse_impl {
    ($ty:ident, $err:ty) => {
        parse_impl!($ty, $ty, $err);
    };

    ($ident:ident, $ty:ident, $err:ty) => {
        fn $ident(&self, key: &str) -> Option<Result<$ty, $err>> {
            self.parse(key)
        }
    };
}

macro_rules! var_impl {
    ($ty:ident) => {
        var_impl!($ty, $ty);
    };

    ($ident:ident, $ty:ident) => {
        fn $ident(&self, key: &str) -> Option<$ty> {
            self.var(key)
        }
    };
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
    fn bool(&self, key: &str) -> Option<Result<bool, ParseBoolError>>;

    /// Returns the value of the environment variable `key` as a `char`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `char`, an error is returned.
    fn char(&self, key: &str) -> Option<Result<char, ParseCharError>>;

    /// Returns the value of the environment variable `key` as a `f32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `f32`, an error is returned.
    fn f32(&self, key: &str) -> Option<Result<f32, ParseFloatError>>;

    /// Returns the value of the environment variable `key` as a `f64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `f64`, an error is returned.
    fn f64(&self, key: &str) -> Option<Result<f64, ParseFloatError>>;

    /// Returns the value of the environment variable `key` as a `i8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i8`, an error is returned.
    fn i8(&self, key: &str) -> Option<Result<i8, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `i16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i16`, an error is returned.
    fn i16(&self, key: &str) -> Option<Result<i16, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `i32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i32`, an error is returned.
    fn i32(&self, key: &str) -> Option<Result<i32, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `i64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i64`, an error is returned.
    fn i64(&self, key: &str) -> Option<Result<i64, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `i128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `i128`, an error is returned.
    fn i128(&self, key: &str) -> Option<Result<i128, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `IpAddr`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `IpAddr`, an error is returned.
    fn ip_addr(&self, key: &str) -> Option<Result<IpAddr, AddrParseError>>;

    /// Returns the value of the environment variable `key` as a `Ipv4Addr`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `Ipv4Addr`, an error is returned.
    fn ipv4_addr(&self, key: &str) -> Option<Result<Ipv4Addr, AddrParseError>>;

    /// Returns the value of the environment variable `key` as a `Ipv6Addr`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `Ipv6Addr`, an error is returned.
    fn ipv6_addr(&self, key: &str) -> Option<Result<Ipv6Addr, AddrParseError>>;

    /// Returns the value of the environment variable `key` as a `isize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `isize`, an error is returned.
    fn isize(&self, key: &str) -> Option<Result<isize, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI8`, an error is returned.
    fn non_zero_i8(&self, key: &str) -> Option<Result<NonZeroI8, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI16`, an error is returned.
    fn non_zero_i16(&self, key: &str) -> Option<Result<NonZeroI16, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI32`, an error is returned.
    fn non_zero_i32(&self, key: &str) -> Option<Result<NonZeroI32, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI64`, an error is returned.
    fn non_zero_i64(&self, key: &str) -> Option<Result<NonZeroI64, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroI128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroI128`, an error is returned.
    fn non_zero_i128(&self, key: &str) -> Option<Result<NonZeroI128, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroIsize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroIsize`, an error is returned.
    fn non_zero_isize(&self, key: &str) -> Option<Result<NonZeroIsize, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU8`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU8`, an error is returned.
    fn non_zero_u8(&self, key: &str) -> Option<Result<NonZeroU8, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU16`, an error is returned.
    fn non_zero_u16(&self, key: &str) -> Option<Result<NonZeroU16, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU32`, an error is returned.
    fn non_zero_u32(&self, key: &str) -> Option<Result<NonZeroU32, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU64`, an error is returned.
    fn non_zero_u64(&self, key: &str) -> Option<Result<NonZeroU64, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroU128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroU128`, an error is returned.
    fn non_zero_u128(&self, key: &str) -> Option<Result<NonZeroU128, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `NonZeroUsize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `NonZeroUsize`, an error is returned.
    fn non_zero_usize(&self, key: &str) -> Option<Result<NonZeroUsize, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `OsString`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `OsString`, an error is returned.
    fn os_string(&self, key: &str) -> Option<OsString>;

    /// Returns the value of the environment variable `key` as a `PathBuf`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `PathBuf`, an error is returned.
    fn path_buf(&self, key: &str) -> Option<PathBuf>;

    /// Returns the value of the environment variable `key` as a `String`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, an error is returned.
    /// See [`std::env::var`](https://doc.rust-lang.org/std/env/fn.var.html) for more details.
    fn raw(&self, key: &str) -> Result<String, VarError>;

    /// Returns the value of the environment variable `key` as a `SocketAddr`.
    ///
    /// If the environment variable is not present, `None` is returned.
    /// If the environment variable is not a valid `SocketAddr`, an error is returned.
    fn socket_addr(&self, key: &str) -> Option<Result<SocketAddr, AddrParseError>>;

    /// Returns the value of the environment variable `key` as a `SocketAddrV4`.
    ///
    /// If the environment variable is not present, `None` is returned.
    /// If the environment variable is not a valid `SocketAddrV4`, an error is returned.
    fn socket_addr_v4(&self, key: &str) -> Option<Result<SocketAddrV4, AddrParseError>>;

    /// Returns the value of the environment variable `key` as a `SocketAddrV6`.
    ///
    /// If the environment variable is not present, `None` is returned.
    /// If the environment variable is not a valid `SocketAddrV6`, an error is returned.
    fn socket_addr_v6(&self, key: &str) -> Option<Result<SocketAddrV6, AddrParseError>>;

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
    fn u8(&self, key: &str) -> Option<Result<u8, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `u16`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u16`, an error is returned.
    fn u16(&self, key: &str) -> Option<Result<u16, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `u32`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u32`, an error is returned.
    fn u32(&self, key: &str) -> Option<Result<u32, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `u64`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u64`, an error is returned.
    fn u64(&self, key: &str) -> Option<Result<u64, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `u128`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is not a valid `u128`, an error is returned.
    fn u128(&self, key: &str) -> Option<Result<u128, ParseIntError>>;

    /// Returns the value of the environment variable `key` as a `usize`.
    ///
    /// If the environment variable is not present or it is not a valid unicode, `None` is returned.
    /// If the environment variable is a valid `usize`, an error is returned.
    fn usize(&self, key: &str) -> Option<Result<usize, ParseIntError>>;
}

// DefaultEnv

/// Default implementation of [`Env`](trait.Env.html).
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/env.rs).
pub struct DefaultEnv;

impl DefaultEnv {
    #[inline]
    fn parse<E: Error + Send + Sync + 'static, T: FromStr<Err = E>>(
        &self,
        key: &str,
    ) -> Option<Result<T, E>> {
        match self.raw(key) {
            Ok(val) => Some(val.parse::<T>()),
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

    #[inline]
    fn var<T: From<String>>(&self, key: &str) -> Option<T> {
        self.string(key).map(|val| val.into())
    }
}

impl Env for DefaultEnv {
    parse_impl!(bool, ParseBoolError);

    parse_impl!(char, ParseCharError);

    parse_impl!(f32, ParseFloatError);

    parse_impl!(f64, ParseFloatError);

    parse_impl!(i8, ParseIntError);

    parse_impl!(i16, ParseIntError);

    parse_impl!(i32, ParseIntError);

    parse_impl!(i64, ParseIntError);

    parse_impl!(i128, ParseIntError);

    parse_impl!(ip_addr, IpAddr, AddrParseError);

    parse_impl!(ipv4_addr, Ipv4Addr, AddrParseError);

    parse_impl!(ipv6_addr, Ipv6Addr, AddrParseError);

    parse_impl!(isize, ParseIntError);

    parse_impl!(non_zero_i8, NonZeroI8, ParseIntError);

    parse_impl!(non_zero_i16, NonZeroI16, ParseIntError);

    parse_impl!(non_zero_i32, NonZeroI32, ParseIntError);

    parse_impl!(non_zero_i64, NonZeroI64, ParseIntError);

    parse_impl!(non_zero_i128, NonZeroI128, ParseIntError);

    parse_impl!(non_zero_isize, NonZeroIsize, ParseIntError);

    parse_impl!(non_zero_u8, NonZeroU8, ParseIntError);

    parse_impl!(non_zero_u16, NonZeroU16, ParseIntError);

    parse_impl!(non_zero_u32, NonZeroU32, ParseIntError);

    parse_impl!(non_zero_u64, NonZeroU64, ParseIntError);

    parse_impl!(non_zero_u128, NonZeroU128, ParseIntError);

    parse_impl!(non_zero_usize, NonZeroUsize, ParseIntError);

    var_impl!(os_string, OsString);

    var_impl!(path_buf, PathBuf);

    fn raw(&self, key: &str) -> Result<String, VarError> {
        trace!(key, "reading environment variable");
        std::env::var(key)
    }

    parse_impl!(socket_addr, SocketAddr, AddrParseError);

    parse_impl!(socket_addr_v4, SocketAddrV4, AddrParseError);

    parse_impl!(socket_addr_v6, SocketAddrV6, AddrParseError);

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

    parse_impl!(u8, ParseIntError);

    parse_impl!(u16, ParseIntError);

    parse_impl!(u32, ParseIntError);

    parse_impl!(u64, ParseIntError);

    parse_impl!(u128, ParseIntError);

    parse_impl!(usize, ParseIntError);
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
        fn bool(&self, key: &str) -> Option<Result<bool, ParseBoolError>>;
        fn char(&self, key: &str) -> Option<Result<char, ParseCharError>>;
        fn f32(&self, key: &str) -> Option<Result<f32, ParseFloatError>>;
        fn f64(&self, key: &str) -> Option<Result<f64, ParseFloatError>>;
        fn i8(&self, key: &str) -> Option<Result<i8, ParseIntError>>;
        fn i16(&self, key: &str) -> Option<Result<i16, ParseIntError>>;
        fn i32(&self, key: &str) -> Option<Result<i32, ParseIntError>>;
        fn i64(&self, key: &str) -> Option<Result<i64, ParseIntError>>;
        fn i128(&self, key: &str) -> Option<Result<i128, ParseIntError>>;
        fn ip_addr(&self, key: &str) -> Option<Result<IpAddr, AddrParseError>>;
        fn ipv4_addr(&self, key: &str) -> Option<Result<Ipv4Addr, AddrParseError>>;
        fn ipv6_addr(&self, key: &str) -> Option<Result<Ipv6Addr, AddrParseError>>;
        fn isize(&self, key: &str) -> Option<Result<isize, ParseIntError>>;
        fn non_zero_i8(&self, key: &str) -> Option<Result<NonZeroI8, ParseIntError>>;
        fn non_zero_i16(&self, key: &str) -> Option<Result<NonZeroI16, ParseIntError>>;
        fn non_zero_i32(&self, key: &str) -> Option<Result<NonZeroI32, ParseIntError>>;
        fn non_zero_i64(&self, key: &str) -> Option<Result<NonZeroI64, ParseIntError>>;
        fn non_zero_i128(&self, key: &str) -> Option<Result<NonZeroI128, ParseIntError>>;
        fn non_zero_isize(&self, key: &str) -> Option<Result<NonZeroIsize, ParseIntError>>;
        fn non_zero_u8(&self, key: &str) -> Option<Result<NonZeroU8, ParseIntError>>;
        fn non_zero_u16(&self, key: &str) -> Option<Result<NonZeroU16, ParseIntError>>;
        fn non_zero_u32(&self, key: &str) -> Option<Result<NonZeroU32, ParseIntError>>;
        fn non_zero_u64(&self, key: &str) -> Option<Result<NonZeroU64, ParseIntError>>;
        fn non_zero_u128(&self, key: &str) -> Option<Result<NonZeroU128, ParseIntError>>;
        fn non_zero_usize(&self, key: &str) -> Option<Result<NonZeroUsize, ParseIntError>>;
        fn os_string(&self, key: &str) -> Option<OsString>;
        fn path_buf(&self, key: &str) -> Option<PathBuf>;
        fn raw(&self, key: &str) -> Result<String, VarError>;
        fn socket_addr(&self, key: &str) -> Option<Result<SocketAddr, AddrParseError>>;
        fn socket_addr_v4(&self, key: &str) -> Option<Result<SocketAddrV4, AddrParseError>>;
        fn socket_addr_v6(&self, key: &str) -> Option<Result<SocketAddrV6, AddrParseError>>;
        fn string(&self, key: &str) -> Option<String>;
        fn strings(&self, key: &str, sep: &str) -> Option<Vec<String>>;
        fn u8(&self, key: &str) -> Option<Result<u8, ParseIntError>>;
        fn u16(&self, key: &str) -> Option<Result<u16, ParseIntError>>;
        fn u32(&self, key: &str) -> Option<Result<u32, ParseIntError>>;
        fn u64(&self, key: &str) -> Option<Result<u64, ParseIntError>>;
        fn u128(&self, key: &str) -> Option<Result<u128, ParseIntError>>;
        fn usize(&self, key: &str) -> Option<Result<usize, ParseIntError>>;
    }
}
