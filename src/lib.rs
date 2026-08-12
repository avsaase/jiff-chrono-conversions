//! Conversions between [`chrono`] and [`jiff`] types.
//!
//! `chrono` is in the process of being soft-deprecated in favor of `jiff`. For small codebases, it
//! usually makes sense to migrate all at once. For larger codebases, a full migration may not be
//! feasible in one go, and it can make more sense to migrate incrementally, converting between
//! `chrono` and `jiff` types at the boundaries. Even once your own code is fully on `jiff`, some
//! dependencies may still expose `chrono` types in their public APIs, so conversions are still
//! needed at those boundaries. This crate provides those conversions, exposed as the
//! [`ToJiff`]/[`TryToJiff`] traits for infallible conversions and [`ToChrono`]/[`TryToChrono`]
//! traits for fallible conversions.
//!
//! This is a small crate, and the conversions it provides are not complicated. But whether a
//! given conversion is fallible or infallible isn't always obvious from the two APIs alone, so it
//! can still be worth using this crate rather than hand-rolling the conversions yourself. Each
//! conversion is checked with property-based tests, making sure the conversions are correct:
//! round-tripping within the range two types have in common and correctly erroring outside of it.
//!
//! # Supported versions
//!
//! Support for each major version of `chrono` and `jiff` is gated behind its own Cargo feature.
//! Currently these are the supported versions:
//!
//! | Feature    | Crate     | Version |
//! |------------|-----------|---------|
//! | `chrono04` | `chrono`  | `0.4`   |
//! | `jiff02`   | `jiff`    | `0.2`   |
//!
//! Enabling the features for a given version of `chrono` and `jiff` will enable the conversions
//! between those versions. Currently the `chrono04` and `jiff02` features are enabled by default.
//!
//! Once `jiff` reaches `1.0` support for this version can be added to this crate.
//!
//! # Limitations
//!
//! Currently only `std` is supported. If you need `no_std` support, please
//! [open an issue](https://github.com/avsaase/jiff-chrono-conversions/issues).
//!
//! [`chrono`]: https://docs.rs/chrono
//! [`jiff`]: https://docs.rs/jiff

#[cfg(all(feature = "chrono04", feature = "jiff02"))]
pub mod chrono04_jiff02;

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
    /// The error type for this conversion.
    type Error;

    /// Convert this `chrono` type to the corresponding `jiff` type.
    ///
    /// This conversion is fallible,. If the converion fails, an error of type `Self::Error` will
    /// be returned. The reasons why the conversion can fail is document on the implementation of
    /// this trait for the specific `chrono` type.
    fn to_jiff(&self) -> Result<J, Self::Error>;
}

/// Fallible conversions from `jiff` types to `chrono` types.
pub trait TryToChrono<C> {
    /// The error type for this conversion.
    type Error;

    /// Convert this `jiff` type to the corresponding `chrono` type.
    ///
    /// This conversion is fallible. If the conversion fails, an error of type `Self::Error` will
    /// be returned. The reasons why the conversion can fail is document on the implementation of
    /// this trait for the specific `jiff` type.
    fn to_chrono(&self) -> Result<C, Self::Error>;
}
