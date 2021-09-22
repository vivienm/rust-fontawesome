//! The [Font Awesome](https://fontawesome.com/) icon set for Rust.
//!
//! ```
//! println!("Hello {}", fontawesome::CHAR_RUST);
//! ```

include!(concat!(env!("OUT_DIR"), "/icon.rs"));

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

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(
            Icon::variants()
                .iter()
                .filter(|icon| *icon == &Icon::Rust)
                .count(),
            1,
        );
    }

    #[test]
    fn test_name() {
        assert_eq!(Icon::Rust.name(), "rust");
    }

    #[test]
    fn test_char() {
        assert_eq!(Icon::Rust.char(), '\u{e07a}');
    }

    #[test]
    fn test_into() {
        assert_eq!(Into::<char>::into(Icon::Rust), '\u{e07a}');
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Icon::try_from('\u{e07a}'), Ok(Icon::Rust));
        assert_eq!(Icon::try_from('\x00'), Err(TryFromIconError));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("hello {}", Icon::Rust), "hello \u{e07a}");
    }
}
