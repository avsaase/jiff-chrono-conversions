# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support for `jiff 1.0` behind the new `jiff-10` feature.

### Changed

- The `jiff-10` and `chrono-04` features are now enabled by default. The `jiff-02` feature is still
  available, but must be enabled explicitly.

## [0.1.1] - 2026-08-15

### Fixed

- Move trait impl doc comments from impl blocks to methods so rust-analyzer shows them.

## [0.1.0] - 2026-08-13

First release ready for general use.

### Fixed

- Hide module with conversion trait impls from docs since it doesn't contain any importable item.

## [0.0.2]

### Added

- Separate cargo feature to make `chrono-tz` an optional dependency.

## [0.0.1]

Initial release.
