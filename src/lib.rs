//! This crate provides an implementation of the SHA-2 224 hash function with a straightforward interface for computing digests of bytes, files, directories, and more.
//!
//! For a low-level interface, you can explore the [`chksum_hash_sha2_224`] crate.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum-sha2-224 = "0.0.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum-sha2-224
//! ```     
//!
//! # Usage
//!
//! Use the [`chksum`] function to calcualate digest of file, directory and so on.
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_224::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Input Types
//!
//! ## Bytes
//!
//! ### Array
//!
//! ```rust
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper() -> Result<()> {
//! let data = [0, 1, 2, 3];
//! let digest = sha2_224::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Vec
//!
//! ```rust
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper() -> Result<()> {
//! let data = vec![0, 1, 2, 3];
//! let digest = sha2_224::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Slice
//!
//! ```rust
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper() -> Result<()> {
//! let data = &[0, 1, 2, 3];
//! let digest = sha2_224::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Strings
//!
//! ### str
//!
//! ```rust
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper() -> Result<()> {
//! let data = "&str";
//! let digest = sha2_224::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### String
//!
//! ```rust
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper() -> Result<()> {
//! let data = String::from("String");
//! let digest = sha2_224::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## File
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha2_224::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Directory
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::read_dir;
//!
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let readdir = read_dir(path)?;
//! let digest = sha2_224::chksum(readdir)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Path
//!
//! ```rust
//! # use std::path::Path;
//! use std::path::PathBuf;
//!
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let path = PathBuf::from(path);
//! let digest = sha2_224::chksum(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Standard Input
//!
//! ```rust
//! use std::io::stdin;
//!
//! # use chksum_sha2_224::Result;
//! use chksum_sha2_224 as sha2_224;
//!
//! # fn wrapper() -> Result<()> {
//! let stdin = stdin();
//! let digest = sha2_224::chksum(stdin)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! Cargo features are utilized to enable extra options.
//!
//! * `reader` enables the [`reader`] module with the [`Reader`] struct.
//! * `writer` enables the [`writer`] module with the [`Writer`] struct.
//!
//! By default, neither of these features is enabled.
//!
//! To customize your setup, disable the default features and enable only those that you need in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! chksum-sha2-224 = { version = "0.0.0", features = ["reader", "writer"] }
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-sha2-224 --features reader,writer
//! ```
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

#[cfg(feature = "reader")]
pub mod reader;
#[cfg(feature = "writer")]
pub mod writer;

use std::fmt::{self, Display, Formatter, LowerHex, UpperHex};

use chksum_core as core;
#[doc(no_inline)]
pub use chksum_core::{Chksumable, Error, Hash, Hashable, Result};
#[doc(no_inline)]
pub use chksum_hash_sha2_224 as hash;

#[cfg(feature = "reader")]
#[doc(inline)]
pub use crate::reader::Reader;
#[cfg(feature = "writer")]
#[doc(inline)]
pub use crate::writer::Writer;

/// Creates a new hash.
///
/// # Example
///
/// ```rust
/// use chksum_sha2_224 as sha2_224;
///
/// let mut hash = sha2_224::new();
/// hash.update(b"example data");
/// let digest = hash.digest();
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
/// );
/// ```
#[must_use]
pub fn new() -> SHA2_224 {
    SHA2_224::new()
}

/// Creates a default hash.
///
/// # Example
///
/// ```rust
/// use chksum_sha2_224 as sha2_224;
///
/// let mut hash = sha2_224::default();
/// hash.update(b"example data");
/// let digest = hash.digest();
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
/// );
/// ```
#[must_use]
pub fn default() -> SHA2_224 {
    core::default()
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_sha2_224 as sha2_224;
///
/// let data = b"example data";
/// let digest = sha2_224::hash(data);
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
/// );
/// ```
pub fn hash(data: impl core::Hashable) -> Digest {
    core::hash::<SHA2_224>(data)
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_sha2_224 as sha2_224;
///
/// let data = b"example data";
/// if let Ok(digest) = sha2_224::chksum(data) {
///     assert_eq!(
///         digest.to_hex_lowercase(),
///         "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
///     );
/// }
/// ```
pub fn chksum(data: impl core::Chksumable) -> Result<Digest> {
    core::chksum::<SHA2_224>(data)
}

/// The SHA-2 224 hash instance.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SHA2_224 {
    inner: hash::Update,
}

impl SHA2_224 {
    /// Calculates the hash digest of an input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha2_224::SHA2_224;
    ///
    /// let data = b"example data";
    /// let digest = SHA2_224::hash(data);
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
    /// );
    /// ```
    #[must_use]
    pub fn hash<T>(data: T) -> Digest
    where
        T: AsRef<[u8]>,
    {
        let mut hash = Self::new();
        hash.update(data);
        hash.digest()
    }

    /// Creates a new hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha2_224::SHA2_224;
    ///
    /// let mut hash = SHA2_224::new();
    /// hash.update(b"example data");
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
    /// );
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let inner = hash::Update::new();
        Self { inner }
    }

    /// Updates the hash state with an input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha2_224::SHA2_224;
    ///
    /// let mut hash = SHA2_224::new();
    /// hash.update(b"example");
    /// hash.update(" ");
    /// hash.update("data");
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
    /// );
    /// ```
    pub fn update<T>(&mut self, data: T)
    where
        T: AsRef<[u8]>,
    {
        self.inner.update(data);
    }

    /// Resets the hash state to its initial state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha2_224::SHA2_224;
    ///
    /// let mut hash = SHA2_224::new();
    /// hash.update(b"example data");
    /// hash.reset();
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"
    /// );
    /// ```
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Produces the hash digest.
    ///
    /// # Example
    ///
    /// ```
    /// use chksum_sha2_224::SHA2_224;
    ///
    /// let mut hash = SHA2_224::new();
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"
    /// );
    /// ```
    #[must_use]
    pub fn digest(&self) -> Digest {
        self.inner.digest().into()
    }
}

impl core::Hash for SHA2_224 {
    type Digest = Digest;

    fn update<T>(&mut self, data: T)
    where
        T: AsRef<[u8]>,
    {
        self.update(data);
    }

    fn reset(&mut self) {
        self.reset();
    }

    fn digest(&self) -> Self::Digest {
        self.digest()
    }
}

/// A hash digest.
pub struct Digest(hash::Digest);

impl Digest {
    /// Creates a new digest.
    #[must_use]
    pub const fn new(digest: [u8; hash::DIGEST_LENGTH_BYTES]) -> Self {
        let inner = hash::Digest::new(digest);
        Self(inner)
    }

    /// Returns a byte slice of the digest's contents.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        let Self(inner) = self;
        inner.as_bytes()
    }

    /// Consumes the digest, returning the digest bytes.
    #[must_use]
    pub fn into_inner(self) -> [u8; hash::DIGEST_LENGTH_BYTES] {
        let Self(inner) = self;
        inner.into_inner()
    }

    /// Returns a string in the lowercase hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha2_224 as sha2_224;
    ///
    /// #[rustfmt::skip]
    /// let digest = [
    ///     0xD1, 0x4A, 0x02, 0x8C,
    ///     0x2A, 0x3A, 0x2B, 0xC9,
    ///     0x47, 0x61, 0x02, 0xBB,
    ///     0x28, 0x82, 0x34, 0xC4,
    ///     0x15, 0xA2, 0xB0, 0x1F,
    ///     0x82, 0x8E, 0xA6, 0x2A,
    ///     0xC5, 0xB3, 0xE4, 0x2F,
    /// ];
    /// let digest = sha2_224::Digest::new(digest);
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"
    /// );
    /// ```
    #[must_use]
    pub fn to_hex_lowercase(&self) -> String {
        let Self(inner) = self;
        inner.to_hex_lowercase()
    }

    /// Returns a string in the uppercase hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha2_224 as sha2_224;
    ///
    /// #[rustfmt::skip]
    /// let digest = [
    ///     0xD1, 0x4A, 0x02, 0x8C,
    ///     0x2A, 0x3A, 0x2B, 0xC9,
    ///     0x47, 0x61, 0x02, 0xBB,
    ///     0x28, 0x82, 0x34, 0xC4,
    ///     0x15, 0xA2, 0xB0, 0x1F,
    ///     0x82, 0x8E, 0xA6, 0x2A,
    ///     0xC5, 0xB3, 0xE4, 0x2F,
    /// ];
    /// let digest = sha2_224::Digest::new(digest);
    /// assert_eq!(
    ///     digest.to_hex_uppercase(),
    ///     "D14A028C2A3A2BC9476102BB288234C415A2B01F828EA62AC5B3E42F"
    /// );
    /// ```
    #[must_use]
    pub fn to_hex_uppercase(&self) -> String {
        let Self(inner) = self;
        inner.to_hex_uppercase()
    }
}

impl core::Digest for Digest {}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        let Self(inner) = self;
        inner.as_bytes()
    }
}

impl Display for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        Display::fmt(inner, f)
    }
}

impl LowerHex for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        LowerHex::fmt(inner, f)
    }
}

impl UpperHex for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        UpperHex::fmt(inner, f)
    }
}

impl From<[u8; hash::DIGEST_LENGTH_BYTES]> for Digest {
    fn from(digest: [u8; hash::DIGEST_LENGTH_BYTES]) -> Self {
        Self::new(digest)
    }
}

impl From<hash::Digest> for Digest {
    fn from(digest: hash::Digest) -> Self {
        Self(digest)
    }
}
