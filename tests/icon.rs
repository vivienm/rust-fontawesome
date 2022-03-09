use std::convert::TryFrom;

use fontawesome::Icon;

#[test]
fn char_from_icon() {
    assert_eq!(char::from(Icon::Rust), '\u{e07a}')
}

#[test]
fn icon_into_char() {
    assert_eq!(Into::<char>::into(Icon::Rust), '\u{e07a}');
}

#[test]
fn icon_from_char() {
    assert_eq!(Icon::try_from('\u{e07a}'), Ok(Icon::Rust));
    assert!(Icon::try_from('\x00').is_err());
}

#[test]
fn char_into_icon() {
    assert_eq!(char::try_into('\u{e07a}'), Ok(Icon::Rust));
    assert!(TryInto::<Icon>::try_into('\x00').is_err());
}

#[test]
fn icon_to_string() {
    assert_eq!(format!("Hello {}", Icon::Rust), "Hello \u{e07a}");
}
