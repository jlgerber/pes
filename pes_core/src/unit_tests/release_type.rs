#![allow(non_snake_case)]

use super::*;

#[test]
fn from_str__given_blank__should_parse() {
    let result = ReleaseType::from_str("").expect("could not parse release");
    assert_eq!(result, ReleaseType::Release);
}
#[test]
fn from_str__given_rc__should_parse() {
    let result = ReleaseType::from_str("rc").expect("could not parse rc");
    assert_eq!(result, ReleaseType::ReleaseCandidate);
}

#[test]
fn from_str__given_alpha__should_parse() {
    let result = ReleaseType::from_str("alpha").expect("could not parse alpha");
    assert_eq!(result, ReleaseType::Alpha);
}

#[test]
fn from_str__given_beta__should_parse() {
    let result = ReleaseType::from_str("beta").expect("could not parse beta");
    assert_eq!(result, ReleaseType::Beta);
}
