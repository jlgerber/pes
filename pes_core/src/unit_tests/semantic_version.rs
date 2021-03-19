#![allow(non_snake_case)]

use super::*;

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
