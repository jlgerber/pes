
#![allow(non_snake_case)]

use super::*;
use nom::error::ErrorKind;
use crate::PesNomError;
use crate::{SemanticVersion, ReleaseType};
use nom::Err::Error as NomErr;

//TODO: add tests for consuming variants

#[test]
fn parse_semver_goodinput() {
    let result = parse_semver("1.2.3");
    assert_eq!(result, Ok(("",SemanticVersion::new(1,2,3,ReleaseType::Release))));
}

#[test]
fn parse_semver__with_prerelease__goodinput() {
    let result = parse_semver("1.2.3-rc");
    assert_eq!(result, Ok(("",SemanticVersion::new(1,2,3,ReleaseType::ReleaseCandidate))));
}

#[test]
fn parse_semver_between_given_goodinput() {
    let result = parse_semver_between("1.2.3+<3.4.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,4,5,ReleaseType::Release)))));
}

#[test]
fn parse_semver_between_given_badinput() {
    let result = parse_semver_between("1.2.3<3.4.5");
    assert_eq!(result, Err(NomErr(PesNomError::Nom( "<3.4.5", ErrorKind::Tag ))));
}

#[test]
fn parse_range_with_spaces() {
    let result = parse_semver_between("1.2.3 +< 3.4.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,4,5,ReleaseType::Release)))));
}

#[test]
fn parse_str_starting_with_carot_major() {
    let result = parse_semver_caret("^1");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,0,0,ReleaseType::Release), SemanticVersion::new(2,0,0,ReleaseType::Release)))));
}

#[test]
fn parse_str_starting_with_carot_minor_with_major_one_or_greater() {
    // eg ^1.2    :=  >=1.2.0, <2.0.0
    let result = parse_semver_caret("^1.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,5,0,ReleaseType::Release), SemanticVersion::new(2,0,0,ReleaseType::Release)))));
    let result = parse_semver_caret("^2.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(2,5,0,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release)))));
}

#[test]
fn parse_str_starting_with_carot_minor_with_major_zero_greater() {
    // eg ^0.2    :=  >=0.2.0, <0.3.0
    let result = parse_semver_caret("^0.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(0,5,0,ReleaseType::Release), SemanticVersion::new(0,6,0,ReleaseType::Release)))));
}

#[test]
fn parse_str_starting_with_caret_path_and_major_one_or_greater() {
    // eg ^1.2.3  :=  >=1.2.3, <2.0.0
    let result = parse_semver_caret("^1.4.2");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,4,2,ReleaseType::Release), SemanticVersion::new(2,0,0,ReleaseType::Release)))));
    let result = parse_semver_caret("^3.4.2");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(3,4,2,ReleaseType::Release), SemanticVersion::new(4,0,0,ReleaseType::Release)))));
}

#[test]
fn parse_str_starting_with_caret_path_and_major_zero() {
    // eg ^0.2.3  :=  >=0.2.3, <0.3.0
    let result = parse_semver_caret("^0.4.2");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(0,4,2,ReleaseType::Release), SemanticVersion::new(0,5,0,ReleaseType::Release)))));
}

#[test]
fn parse_str_exact() {
    let result = parse_semver_exact("1.2.3");
    assert_eq!(result, Ok(("", Range::exact(SemanticVersion::new(1,2,3,ReleaseType::Release)))));
}

#[test]
fn parse_semver_from_table() {
    let versions = vec![
        ("1.23.4", Ok(("", Range::exact(SemanticVersion::new(1,23,4,ReleaseType::Release))))) ,
        ("1.2.3+<3", Ok(("", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release))))),
        ("^2.2.1", Ok(("", Range::between(SemanticVersion::new(2,2,1,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release) )))),
        ("^2.2", Ok(("", Range::between(SemanticVersion::new(2,2,0,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release) )))),
        ("^2", Ok(("", Range::between(SemanticVersion::new(2,0,0,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release) )))),
        ("^0.0.2", Ok(("", Range::between(SemanticVersion::new(0,0,2,ReleaseType::Release), SemanticVersion::new(0,0,3,ReleaseType::Release) )))),
        ("^0.1.2", Ok(("", Range::between(SemanticVersion::new(0,1,2,ReleaseType::Release), SemanticVersion::new(0,2,0,ReleaseType::Release) )))),
        ("^0.2", Ok(("", Range::between(SemanticVersion::new(0,2, 0,ReleaseType::Release), SemanticVersion::new(0,3,0,ReleaseType::Release) )))),
    ];

    for (input,expected) in versions {
        assert_eq!(parse_semver_range(input), expected);
    }
}

#[test]
fn parse_package_name_and_version() {
    let result = parse_package_version("maya-1.2.3");
    assert_eq!(result, Ok(("", ("maya", SemanticVersion::new(1,2,3,ReleaseType::Release)))));
}

#[test]
fn parse_package_name_and_range() {
    /* should conform to:
    ^1.2.3  :=  >=1.2.3, <2.0.0
    ^1.2    :=  >=1.2.0, <2.0.0
    ^1      :=  >=1.0.0, <2.0.0
    ^0.2.3  :=  >=0.2.3, <0.3.0
    ^0.2    :=  >=0.2.0, <0.3.0
    ^0.0.3  :=  >=0.0.3, <0.0.4
    ^0.0    :=  >=0.0.0, <0.1.0
    ^0      :=  >=0.0.0, <1.0.0
    */
    let versions = vec![
        ("maya-1.23.4", Ok(("", ("maya",Range::exact(SemanticVersion::new(1,23,4,ReleaseType::Release)))))) ,
        ("houdini-1.2.3+<3", Ok(("",("houdini", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release)))))),
        ("houdini-1.2.3 +< 3.0.0", Ok(("",("houdini", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release)))))),
        ("nuke-^2.2", Ok(("", ("nuke", Range::between(SemanticVersion::new(2,2,0,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release) ))))),
        ("nuke-^2", Ok(("", ("nuke", Range::between(SemanticVersion::new(2,0,0,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release) ))))),
        ("nuke-^0.2.3", Ok(("", ("nuke", Range::between(SemanticVersion::new(0,2,3,ReleaseType::Release), SemanticVersion::new(0,3,0,ReleaseType::Release) ))))),
        ("nuke-^0.0.3", Ok(("", ("nuke", Range::between(SemanticVersion::new(0,0,3,ReleaseType::Release), SemanticVersion::new(0,0,4,ReleaseType::Release) )))))
    ];
    for (input, expected) in versions {
        let result = parse_package_range(input);
        assert_eq!(result, expected);
    
    }
}

#[test]
fn parse_variant_semver__when_given_release__succeeds() {
    let result = parse_variant_semver("1.2.3@1");
    assert_eq!(result, Ok(("",Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),1))));
}

#[test]
fn parse_variant_semver__when_given_prerelease__succeeds() {
    let result = parse_variant_semver("1.2.3-beta@1");
    assert_eq!(result, Ok(("",Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta),1))));
}
    
