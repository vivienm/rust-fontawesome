use std::str::FromStr;

use fontawesome::Version;

#[test]
fn version_to_string() {
    assert_eq!(Version::new(5, 14, 4).to_string(), "5.14.4");
    assert_eq!(Version::new(6, 0, 0).to_string(), "6.0.0");
}

#[test]
fn version_from_string() {
    assert_eq!(Version::from_str("5.14.4").unwrap(), Version::new(5, 14, 4));
    assert_eq!(Version::from_str("6.0.0").unwrap(), Version::new(6, 0, 0));
    assert!(Version::from_str("5.14").is_err());
    assert!(Version::from_str("5.1.4.4").is_err());
    assert!(Version::from_str("5.14.").is_err());
    assert!(Version::from_str("5.14.a").is_err());
    assert!(Version::from_str("5.14.1000").is_err());
}

#[test]
fn compare_versions() {
    assert!(Version::new(5, 14, 3) < Version::new(5, 14, 4));
    assert!(Version::new(5, 14, 4) < Version::new(6, 0, 0));
}
