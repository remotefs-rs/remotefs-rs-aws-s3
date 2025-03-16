# Changelog

- [Changelog](#changelog)
  - [0.4.1](#041)
  - [0.4.0](#040)
  - [0.3.1](#031)
  - [0.3.0](#030)
  - [0.2.4](#024)
  - [0.2.3](#023)
  - [0.2.2](#022)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.0](#010)

---

## 0.4.1

Released on 16/03/2024

- fixed aws-s3 upload. It doesn't support offsets for write, but only multipart

## 0.4.0

Released on 16/03/2024

- Migrated to rust `aws-sdk-s3`
- use `testcontainers` for tests
- rust edition `2024`

‼️ WARNING: this release has changed the S3 Backend!!!

I've finally with like 3 years of delay **migrated** to the **official AWS SDK for Rust**.

From the user side, actually there aren't many changes, the only thing that actually matters is that **you need to have a tokio Runtime** to run the client. The `AwsS3Fs::new` now takes both the `bucket` and the `runtime` as an `Arc<Runtime>` argument.

## 0.3.1

Released on 07/10/2024

- Removed unused dep: `users`

## 0.3.0

Released on 30/09/2024

- remotefs 0.3.0

## 0.2.4

Released on 02/03/2024

- Fixed windows build

## 0.2.3

Released on 01/03/2024

- Bump `rust-s3` to `0.34.0-rc4` which fixes issues with `open_file`

## 0.2.2

Released on 01/03/2024

- Bump `rust-s3` to `0.33`

## 0.2.1

Released on 10/10/2022

- Added `native-tls` and `rustls` support

## 0.2.0

Released on 05/02/2022

- Added support for S3 compatible APIs (such as minio, yandex)
- New constructor methods
  - `new()` will now accept only the bucket name
  - `region()` to specify the region. If no region is specified, custom region will be used
  - `endpoint()` to specify the endpoint. Useful to connect to minio
  - `new_path_style()`: must be specified when connecting to some backends, such as minio

## 0.1.0

Released on 04/01/2022

- First release
