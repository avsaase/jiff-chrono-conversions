#[cfg(all(feature = "chrono04", feature = "jiff02"))]
pub mod chrono04_jiff02;

/// Trait for infallible conversions from `chrono` types to `jiff` types.
pub trait ToJiff<T> {
    fn to_jiff(&self) -> T;
}

/// Trait for infallible conversions from `jiff` types to `chrono` types.
pub trait ToChrono<T> {
    fn to_chrono(&self) -> T;
}

/// Trait for fallible conversions from `chrono` types to `jiff` types.
pub trait TryToJiff<T> {
    type Error;

    fn to_jiff(&self) -> Result<T, Self::Error>;
}

/// Trait for fallible conversions from `jiff` types to `chrono` types.
pub trait TryToChrono<T> {
    type Error;

    fn to_chrono(&self) -> Result<T, Self::Error>;
}
