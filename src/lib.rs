//! The [Font Awesome](https://fontawesome.com/) icon set for Rust.
//!
//! ```
//! use fontawesome::Icon;
//!
//! println!("Hello {}", Icon::Rust);
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::{
    error::{ParseVersionError, TryFromCharError},
    icon::Icon,
    plan::Plan,
    version::{Version, VERSION},
};

mod error;
mod icon;
mod plan;
mod version;
