# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Move trait impl doc comments from impl blocks to methods so rust-analyzer shows them.

# [0.1.0] - 2026-08-13

First release ready for general use.

### Fixed

- Hide module with conversion trait impls from docs since it doesn't contain any importable item.

## [0.0.2]

### Added

- Separate cargo feature to make `chrono-tz` an optional dependency.

## [0.0.1]

Initial release.
