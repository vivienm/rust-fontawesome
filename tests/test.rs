use std::convert::TryFrom;

use fontawesome::{Icon, TryFromIconError};

#[test]
fn count() {
    assert_eq!(Icon::count(), Icon::variants().len());
}

#[test]
fn variants() {
    assert_eq!(
        Icon::variants()
            .iter()
            .filter(|icon| *icon == &Icon::Rust)
            .count(),
        1,
    );
}

#[test]
fn name() {
    assert_eq!(Icon::Rust.name(), "rust");
}

#[test]
fn char() {
    assert_eq!(Icon::Rust.char(), '\u{e07a}');
}

#[test]
fn into() {
    assert_eq!(Into::<char>::into(Icon::Rust), '\u{e07a}');
}

#[test]
fn try_from() {
    assert_eq!(Icon::try_from('\u{e07a}'), Ok(Icon::Rust));
    assert_eq!(Icon::try_from('\x00'), Err(TryFromIconError));
}

#[test]
fn display() {
    assert_eq!(format!("hello {}", Icon::Rust), "hello \u{e07a}");
}
