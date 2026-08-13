//! Conversions between [`chrono`] and [`jiff`] types.
//!
//! `chrono` is in the process of being [soft-deprecated] in favor of `jiff`. For small codebases, it
//! usually makes sense to migrate all at once. For larger codebases, a full migration may not be
//! feasible in one go, and it can make more sense to migrate incrementally, converting between
//! `chrono` and `jiff` types at the boundaries. Even once your own code is fully on `jiff`, some
//! dependencies may still expose `chrono` types in their public APIs, so conversions are still
//! needed at those boundaries. This crate provides those conversions, exposed as the
//! [`ToJiff`]/[`ToChrono`] traits for infallible conversions and [`TryToJiff`]/[`TryToChrono`]
//! traits for fallible conversions.
//!
//! This is a small crate, and the conversions it provides are not complicated. But whether a
//! given conversion is fallible or infallible isn't always obvious from the two APIs alone, so it
//! can still be worth using this crate rather than hand-rolling the conversions yourself. Each
//! conversion is checked with property-based tests, making sure the conversions are correct,
//! round-trip correctly, and that the fallible conversions fail when they should.
//!
//! # Example
//!
//! ```
//! # use chrono_04 as chrono;
//! # use jiff_02 as jiff;
//! use jiff_chrono_conversions::{ToChrono, TryToJiff};
//!
//! let jiff_date = jiff::civil::date(2024, 3, 15);
//! let chrono_date = jiff_date.to_chrono();
//! assert_eq!(chrono_date, chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
//!
//! let jiff_date: jiff::civil::Date = chrono_date.try_to_jiff().unwrap();
//! assert_eq!(jiff_date, jiff::civil::date(2024, 3, 15));
//! ```
//!
//! See the examples folder in the repository for more examples of available conversions.
//!
//! # Supported versions
//!
//! Support for each major version of `chrono` and `jiff` is gated behind its own Cargo feature.
//! Currently these are the supported versions:
//!
//! | Feature         | Crate       | Version  |
//! |-----------------|-------------|----------|
//! | `chrono-04`     | `chrono`    | `0.4`    |
//! | `chrono-tz-010` | `chrono-tz` | `0.10.4` |
//! | `jiff-02`       | `jiff`      | `0.2`    |
//!
//! Enabling the features for a given version of `chrono` and `jiff` will enable the conversions
//! between those versions. Currently only the `chrono-04` and `jiff-02` features are enabled by
//! default; `chrono-tz-010` adds conversions for `chrono_tz::Tz` and must be enabled separately.
//!
//! Once `jiff` reaches `1.0` support for this version can be added to this crate.
//!
//! # Error handling
//!
//! To keep the error handling as simple as possible, all fallible conversions in this crate return
//! the same [`Error`] type. This is because `chrono` does not consistently use `Result` for its
//! fallible operations and not all possible errors in the conversions compose cleanly. If a
//! conversion failed because of an underlying `chrono` or `jiff` error you can inspect the
//! underlying cause via [`std::error::Error::source`]. Otherwise, the error message will describe
//! the failure reason in a human-readable way.
//!
//! # Limitations
//!
//! Currently only `std` is supported. If you need `no_std` support, please
//! [open an issue](https://github.com/avsaase/jiff-chrono-conversions/issues).
//!
//! # Licence
//! MIT
//!
//! [`chrono`]: https://docs.rs/chrono
//! [`jiff`]: https://docs.rs/jiff
//! [soft-deprecated]: https://github.com/chronotope/chrono/issues/1768

#[cfg(all(feature = "chrono-04", feature = "jiff-02"))]
pub mod chrono04_jiff02;

/// The error type returned by the fallible conversions in this crate.
///
/// This type is deliberately opaque: it carries only a human-readable message (available via its
/// `Display` implementation) and, where applicable, an underlying cause (available via
/// [`std::error::Error::source`]).
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Error {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub(crate) fn with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

/// Infallible conversions from `chrono` types to `jiff` types.
pub trait ToJiff<J> {
    /// Convert this `chrono` type to the corresponding `jiff` type.
    ///
    /// This conversion is infallible, so it will always succeed.
    fn to_jiff(&self) -> J;
}

/// Infallible conversions from `jiff` types to `chrono` types.
pub trait ToChrono<C> {
    /// Convert this `jiff` type to the corresponding `chrono` type.
    ///
    /// This conversion is infallible, so it will always succeed.
    fn to_chrono(&self) -> C;
}

/// Fallible conversions from `chrono` types to `jiff` types.
pub trait TryToJiff<J> {
    /// Convert this `chrono` type to the corresponding `jiff` type.
    ///
    /// This conversion is fallible. If the conversion fails, an [`Error`] is returned. The
    /// reasons why the conversion can fail is documented on the implementation of this trait for
    /// the specific `chrono` type.
    fn try_to_jiff(&self) -> Result<J, Error>;
}

/// Fallible conversions from `jiff` types to `chrono` types.
pub trait TryToChrono<C> {
    /// Convert this `jiff` type to the corresponding `chrono` type.
    ///
    /// This conversion is fallible. If the conversion fails, an [`Error`] is returned. The
    /// reasons why the conversion can fail is documented on the implementation of this trait for
    /// the specific `jiff` type.
    fn try_to_chrono(&self) -> Result<C, Error>;
}
