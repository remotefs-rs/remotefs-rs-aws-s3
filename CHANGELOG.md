# Changelog

All notable changes to this project are documented in this file.

## 1.0.0

Released on 2026-09-09

### Breaking changes

- migrate to remotefs 1

> migrate AwsS3Fs to the remotefs 1 asynchronous API and release version 1.0.0.

### Added

- Breaking: migrate to remotefs 1

### Build

- update dependencies. (#3)

> Increased MSRV to 1.94.1

## 0.4.3

Released on 2025-12-20

### Fixed

- lint

## 0.4.2

Released on 2025-03-23

### Fixed

- fixed `remove_file` which didn't removed files

## 0.4.1

Released on 2025-03-16

### Fixed

- **upload:** fixed aws-s3 upload. It doesn't support offsets for write, but only multipart

## 0.4.0

Released on 2025-03-16

### Breaking changes

- use testcontainers for tests; rust edition 2024

> use testcontainers for tests; rust edition 2024

- **aws:** migrated to aws-sdk

> migrated to aws-sdk

### Added

- Breaking: use testcontainers for tests; rust edition 2024
- Breaking: **aws:** migrated to aws-sdk

> I've finally with like 3 years of delay **migrated** to the **official AWS SDK for Rust**.

### Fixed

- test is sync and send

## 0.3.1

Released on 2024-10-07

### Fixed

- removed users dep

## 0.3.0

Released on 2024-09-30

### Added

- remotefs 0.3.0

### Fixed

- bump rust-s3
- bump rust-s3 to 0.34
- lint
- windows build
- version
- ci

## 0.1.0

Released on 2022-01-04
