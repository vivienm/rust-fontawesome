//! The [Font Awesome][FontAwesome] icon set for Rust.
//!
//! ```
//! use fontawesome::Icon;
//!
//! println!("Hello {}", Icon::Rust);
//! ```
//!
//! [Repository]
//!
//! [FontAwesome]: https://fontawesome.com/
//! [Repository]: https://github.com/vivienm/rust-fontawesome

#![cfg_attr(doc, feature(doc_auto_cfg))]
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
