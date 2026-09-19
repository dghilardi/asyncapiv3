# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Add `Clone` derive to the `Error` enum
- Add `preserve_order` feature backing spec maps with `IndexMap` to keep insertion order
- Add rustdoc to the spec types and show feature gating on docs.rs
- Add `Clone`, `Debug` and `PartialEq` derives to the spec types
- Add WebSocket, NATS and HTTP bindings for server, channel, operation and message objects

### Changed
- Omit optional fields with no value when serializing, instead of emitting `null`
- **Breaking**: rename the `builder_unstable` feature to `writer` and enable it by default

## [0.1.3] 2025-06-28
### Changed
- Upgrade dependencies: schemars 1.0, thiserror: 2.0

## [0.1.2] 2024-01-28
### Fixed
- Fixed wrong types and defaults for deserialization

## [0.1.1] 2024-01-27
### Added
- Add CHANGELOG.md file
