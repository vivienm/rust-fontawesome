use core::fmt::{self, Display};

include!(concat!(env!("OUT_DIR"), "/icon.rs"));

impl Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        icon.char()
    }
}
