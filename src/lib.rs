//! The [Font Awesome](https://fontawesome.com/) icon set for Rust.
//!
//! ```
//! println!("Hello {}", fontawesome::CHAR_RUST);
//! ```

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use std::error::Error;
use std::fmt;

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
pub struct TryFromIconError;

impl fmt::Display for TryFromIconError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        "invalid icon character".fmt(f)
    }
}

impl Error for TryFromIconError {}
