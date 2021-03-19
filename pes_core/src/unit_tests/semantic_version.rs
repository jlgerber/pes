#![allow(non_snake_case)]

use super::*;
use nom::combinator::complete;
use nom::error::ErrorKind;
use nom::Err::Error as NomErr;
use crate::PesNomError;

mod ReleaseTypeTests {
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
}
#[test]
fn bump__given_release__should_bump_patch() {
    let semver = SemanticVersion::new(1,2,3,ReleaseType::Release);
    let semver = semver.bump();
    let expect = SemanticVersion::new(1,2,4,ReleaseType::Release);
    assert_eq!(semver, expect);
}

#[test]
fn bump__given_ReleaseCandidate__should_bump_release_type_to_Release() {
    let semver = SemanticVersion::new(1,2,3,ReleaseType::ReleaseCandidate);
    let semver = semver.bump();
    let expect = SemanticVersion::new(1,2,3,ReleaseType::Release);
    assert_eq!(semver, expect);
}

#[test]
fn bump__given_Beta__should_bump_release_type_to_ReleaseCandidate() {
    let semver = SemanticVersion::new(1,2,3,ReleaseType::Beta);
    let semver = semver.bump();
    let expect = SemanticVersion::new(1,2,3,ReleaseType::ReleaseCandidate);
    assert_eq!(semver, expect);
}

#[test]
fn bump__given_Alpha__should_bump_release_type_to_Beta() {
    let semver = SemanticVersion::new(1,2,3,ReleaseType::Alpha);
    let semver = semver.bump();
    let expect = SemanticVersion::new(1,2,3,ReleaseType::Beta);
    assert_eq!(semver, expect);
}
