# jiff-chrono-conversions

[![crates.io](https://img.shields.io/crates/v/jiff-chrono-conversions.svg)](https://crates.io/crates/jiff-chrono-conversions)
[![docs.rs](https://docs.rs/jiff-chrono-conversions/badge.svg)](https://docs.rs/jiff-chrono-conversions)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo +nightly reedme

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

Conversions between [`chrono`] and [`jiff`] types.

`chrono` is in the process of being [soft-deprecated] in favor of `jiff`. For small codebases, it
usually makes sense to migrate all at once. For larger codebases, a full migration may not be
feasible in one go, and it can make more sense to migrate incrementally, converting between
`chrono` and `jiff` types at the boundaries. Even once your own code is fully on `jiff`, some
dependencies may still expose `chrono` types in their public APIs, so conversions are still
needed at those boundaries. This crate provides those conversions, exposed as the
[`ToJiff`](https://docs.rs/jiff-chrono-conversions/latest/jiff_chrono_conversions/trait.ToJiff.html)/[`ToChrono`](https://docs.rs/jiff-chrono-conversions/latest/jiff_chrono_conversions/trait.ToChrono.html) traits for infallible conversions and [`TryToJiff`](https://docs.rs/jiff-chrono-conversions/latest/jiff_chrono_conversions/trait.TryToJiff.html)/[`TryToChrono`](https://docs.rs/jiff-chrono-conversions/latest/jiff_chrono_conversions/trait.TryToChrono.html)
traits for fallible conversions.

This is a small crate, and the conversions it provides are not complicated. But whether a
given conversion is fallible or infallible isn’t always obvious from the two APIs alone, so it
can still be worth using this crate rather than hand-rolling the conversions yourself. Each
conversion is checked with property-based tests, making sure the conversions are correct,
round-trip correctly, and that the fallible conversions fail when they should.

## Example

```rust
use jiff_chrono_conversions::{ToChrono, TryToJiff};

let jiff_date = jiff::civil::date(2024, 3, 15);
let chrono_date = jiff_date.to_chrono();
assert_eq!(chrono_date, chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());

let jiff_date: jiff::civil::Date = chrono_date.try_to_jiff().unwrap();
assert_eq!(jiff_date, jiff::civil::date(2024, 3, 15));
```

See the examples folder in the repository for more examples of avialble conversions.

## Supported versions

Support for each major version of `chrono` and `jiff` is gated behind its own Cargo feature.
Currently these are the supported versions:

| Feature    | Crate     | Version |
|------------|-----------|---------|
| `chrono04` | `chrono`  | `0.4`   |
| `jiff02`   | `jiff`    | `0.2`   |

Enabling the features for a given version of `chrono` and `jiff` will enable the conversions
between those versions. Currently only the `chrono04` and `jiff02` features are available and
they are enabled by default.

Once `jiff` reaches `1.0` support for this version can be added to this crate.

## Error handling

To keep the error handling as simple as possible, all fallible conversions in this crate return
the same [`Error`](https://docs.rs/jiff-chrono-conversions/latest/jiff_chrono_conversions/struct.Error.html) type. This is because `chrono` does not consistently use `Result` for its
fallible operations and not all possible errors in the conversions compose cleanly. If a
conversion failed because of an underlying `chrono` or `jiff` error you can inspect the
underlying cause via [`std::error::Error::source`](https://doc.rust-lang.org/stable/core/error/Error/fn.source.html). Otherwise, the error message will describe
the failure reason in a human-readable way.

## Limitations

Currently only `std` is supported. If you need `no_std` support, please
[open an issue](https://github.com/avsaase/jiff-chrono-conversions/issues).

## Licence
MIT

[`chrono`]: https://docs.rs/chrono
[`jiff`]: https://docs.rs/jiff
[soft-deprecated]: https://github.com/chronotope/chrono/issues/1768

<!-- cargo-reedme: end -->
