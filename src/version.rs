use core::{
    fmt::{self, Display},
    str::FromStr,
};

use super::error::ParseVersionError;

/// A version number for the Font Awesome icon set.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    /// Creates a new `Version`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Version;
    ///
    /// let version = Version::new(5, 14, 4);
    /// ```
    #[inline]
    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// The version major number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Version;
    ///
    /// let version = Version::new(5, 14, 4);
    /// assert_eq!(version.major(), 5);
    /// ```
    #[inline]
    pub const fn major(&self) -> u8 {
        self.major
    }

    /// The version minor number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Version;
    ///
    /// let version = Version::new(5, 14, 4);
    /// assert_eq!(version.minor(), 14);
    /// ```
    #[inline]
    pub const fn minor(&self) -> u8 {
        self.minor
    }

    /// The version patch number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Version;
    ///
    /// let version = Version::new(5, 14, 4);
    /// assert_eq!(version.patch(), 4);
    /// ```
    #[inline]
    pub const fn patch(&self) -> u8 {
        self.patch
    }

    /// Returns the version components as a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Version;
    ///
    /// let version = Version::new(5, 14, 4);
    /// assert_eq!(version.as_tuple(), (5, 14, 4));
    /// ```
    #[inline]
    pub const fn as_tuple(&self) -> (u8, u8, u8) {
        (self.major, self.minor, self.patch)
    }

    fn parse_next_component<'a, I>(components: &mut I) -> Result<u8, ParseVersionError>
    where
        I: Iterator<Item = &'a str>,
    {
        let component: u8 = components
            .next()
            .ok_or(ParseVersionError { _priv: () })?
            .parse()
            .map_err(|_| ParseVersionError { _priv: () })?;
        Ok(component)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split('.');
        let (major, minor, patch) = (
            Self::parse_next_component(&mut components)?,
            Self::parse_next_component(&mut components)?,
            Self::parse_next_component(&mut components)?,
        );
        if components.next().is_some() {
            return Err(ParseVersionError { _priv: () });
        }
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/version.rs"));
