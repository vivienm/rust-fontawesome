use std::fmt;

include!(concat!(env!("OUT_DIR"), "/icon.rs"));

impl From<Icon> for char {
    #[inline]
    fn from(icon: Icon) -> Self {
        icon.to_char()
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_char().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label() {
        assert_eq!(Icon::Rust.label(), "Rust");
    }

    #[test]
    fn test_to_char() {
        assert_eq!(Icon::Rust.to_char(), '\u{e07a}');
    }

    #[test]
    fn test_into() {
        assert_eq!(Into::<char>::into(Icon::Rust), '\u{e07a}');
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("hello {}", Icon::Rust), "hello \u{e07a}");
    }
}
