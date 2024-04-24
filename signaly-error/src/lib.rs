#![forbid(unsafe_code)]
#![deny(
    dead_code,
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! internal library to provide structures for errors in Signaly.
//!
//! # Examples
//! ```rust
//! use signaly_error::Result;
//!
//! fn main() -> Result<()> {
//!     Ok(())
//! }
//! ```

use std::error::Error;

/// Boxed error to bypass specific [Error].
type BError = Box<dyn Error>;
/// anyhow-like error handler.
pub type Result<T> = core::result::Result<T, BError>;
