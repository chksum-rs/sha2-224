//! This module is optional and can be enabled using the `writer` Cargo feature.
//!
//! The [`Writer`] allows on-the-fly calculation of the digest while writing the data.
//!
//! # Enabling
//!
//! Add the following entry to your `Cargo.toml` file to enable the `writer` feature:
//!
//! ```toml
//! [dependencies]
//! chksum-sha2-224 = { version = "0.1.0", features = ["writer"] }
//! ```
//!
//! Alternatively, use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-sha2-224 --features writer
//! ```
//!
//! # Example
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//! use std::io::Write; // required by writer
//!
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let mut writer = sha2_224::writer::new(file);
//!
//! writer.write_all(b"example data")?;
//!
//! let digest = writer.digest();
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```

use std::io::Write;

use chksum_writer as writer;
#[cfg(feature = "async-runtime-tokio")]
use tokio::io::AsyncWrite;

use crate::SHA2_224;

/// A specialized [`Writer`](writer::Writer) type with the [`SHA2_224`] hash algorithm.
pub type Writer<W> = writer::Writer<W, SHA2_224>;

#[cfg(feature = "async-runtime-tokio")]
/// A specialized [`AsyncWriter`](writer::AsyncWriter) type with the [`SHA2_224`] hash algorithm.
pub type AsyncWriter<R> = writer::AsyncWriter<R, SHA2_224>;

/// Creates new [`Writer`].
pub fn new(inner: impl Write) -> Writer<impl Write> {
    writer::new(inner)
}

/// Creates new [`Writer`] with provided hash.
pub fn with_hash(inner: impl Write, hash: SHA2_224) -> Writer<impl Write> {
    writer::with_hash(inner, hash)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`].
pub fn async_new(inner: impl AsyncWrite) -> AsyncWriter<impl AsyncWrite> {
    writer::async_new(inner)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`] with provided hash.
pub fn async_with_hash(inner: impl AsyncWrite, hash: SHA2_224) -> AsyncWriter<impl AsyncWrite> {
    writer::async_with_hash(inner, hash)
}
