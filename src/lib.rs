#![crate_name = "remotefs_aws_s3"]
#![crate_type = "lib"]

//! # remotefs-aws-s3
//!
//! remotefs-aws-s3 is a client implementation for
//! [remotefs](https://github.com/remotefs-rs/remotefs-rs), providing support
//! for the AWS S3 protocol.
//!
//! ## Get started
//!
//! First, add **remotefs** and this client to your project dependencies:
//!
//! ```toml
//! remotefs = "1"
//! remotefs-aws-s3 = "1"
//! ```
//!
//! The client implements [`remotefs::AsyncRemoteFs`]. Every path is absolute
//! and rooted at the bucket (`/` is the bucket root); directories are objects
//! whose key ends with `/`.
//!
//! ## Feature flags
//!
//! | name | description | default |
//! | --- | --- | --- |
//! | `find` | Enable the remotefs `find_async()` function. | ✔ |
//! | `no-log` | Disable logging through the `log` crate. | |
//! | `tokio` | Enable the remotefs Tokio adapters for blocking callers. | |
//! | `with-containers` | Enable MinIO-backed integration tests. | |
//! | `with-s3-ci` | Enable tests against a configured S3 bucket. | |
//!
//! ### AWS S3 client
//!
//! ```rust,no_run
//! use std::path::Path;
//!
//! use remotefs::AsyncRemoteFs;
//! use remotefs::fs::{ReadOptions, WriteOptions};
//! use remotefs_aws_s3::AwsS3Fs;
//!
//! # async fn run() -> remotefs::RemoteResult<()> {
//! let mut client = AwsS3Fs::new("test-bucket")
//!     .region("eu-west-1")
//!     .profile("default")
//!     .access_key("AKIAxxxxxxxxxxxx")
//!     .secret_access_key("****************");
//! client.connect().await?;
//! let mut source = futures::io::Cursor::new(b"hello".to_vec());
//! client
//!     .write_file(
//!         Path::new("/docs/hello.txt"),
//!         &WriteOptions::default().size_hint(5),
//!         &mut source,
//!     )
//!     .await?;
//! let mut destination = futures::io::Cursor::new(Vec::new());
//! client
//!     .read_file(
//!         Path::new("/docs/hello.txt"),
//!         &ReadOptions::default().length(3),
//!         &mut destination,
//!     )
//!     .await?;
//! assert_eq!(destination.into_inner(), b"hel");
//! client.disconnect().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### MinIO client
//!
//! ```rust,no_run
//! use std::path::Path;
//!
//! use remotefs::AsyncRemoteFs;
//! use remotefs_aws_s3::AwsS3Fs;
//!
//! # async fn run() -> remotefs::RemoteResult<()> {
//! let mut client = AwsS3Fs::new("test-bucket")
//!     .endpoint("http://localhost:9000")
//!     .new_path_style(true) // required for MinIO
//!     .access_key("minioadmin")
//!     .secret_access_key("minioadmin");
//! client.connect().await?;
//! for entry in client.list_dir(Path::new("/")).await? {
//!     println!("{path}", path = entry.path().display());
//! }
//! client.disconnect().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Blocking usage
//!
//! Enable the `tokio` feature and wrap the client in `BlockOn`; the adapter
//! must not be called from inside an async context.
//!
//! ```rust,ignore
//! use remotefs::RemoteFs;
//! use remotefs::adapters::blocking::BlockOn;
//! use remotefs_aws_s3::AwsS3Fs;
//!
//! let runtime = tokio::runtime::Runtime::new().unwrap();
//! let mut client = BlockOn::new(AwsS3Fs::new("test-bucket"), runtime.handle().clone());
//! client.connect().unwrap();
//! ```
//!
//! ### Transfers
//!
//! `open` returns an owned read stream backed by a ranged `GetObject`; offsets
//! and lengths are honored natively (`Capabilities::RANGE_READ`). `create`
//! returns an owned write stream that stages 5 MiB multipart parts. The object
//! is created only when `finish` is awaited, and a dropped stream aborts the
//! multipart upload on a best-effort basis. `append`, `copy`, `rename`,
//! `symlink`, `set_metadata`, and `exec` are not supported by S3 and return
//! `UnsupportedFeature`.
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/remotefs-rs/remotefs-rs/main/assets/logo-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/remotefs-rs/remotefs-rs/main/assets/logo.png"
)]

// -- crates
#[macro_use]
extern crate log;

pub mod client;
pub use client::AwsS3Fs;

// -- key
pub(crate) mod key;
// -- object
pub(crate) mod object;
// -- streams
pub(crate) mod stream;
// -- mock
#[cfg(test)]
pub(crate) mod mock;
