//! The [Font Awesome](https://fontawesome.com/) icon set for Rust.
//!
//! ```
//! println!("Hello {}", fontawesome::char::RUST);
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use core::fmt;

impl From<Icon> for char {
    #[inline]
    fn from(icon: Icon) -> Self {
        icon.char()
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.char().fmt(f)
    }
}

/// The error type returned when a character conversion fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IconTryFromError(());

impl fmt::Display for IconTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        "invalid icon character".fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IconTryFromError {}
