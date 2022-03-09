use core::fmt::{self, Display};

/// The error type returned when a character conversion fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryFromCharError(pub(crate) ());

impl Display for TryFromCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "invalid icon character".fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromCharError {}

/// An error which can be returned when parsing a [`Version`](crate::Version).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseVersionError {
    pub(crate) _priv: (),
}

impl Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "could not parse version string".fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseVersionError {}
