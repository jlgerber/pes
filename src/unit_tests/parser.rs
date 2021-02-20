
#![allow(non_snake_case)]

use super::*;

use nom::error::ErrorKind;
use nom::error::Error as NomError;
use nom::Err::Error as NomErr;

#[test]
fn parse_semver_goodinput() {
    let result = parse_semver("1.2.3");
    assert_eq!(result, Ok(("",SemanticVersion::new(1,2,3))));
}

#[test]
fn parse_semver_between_given_goodinput() {
    let result = parse_semver_between("1.2.3+<3.4.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,4,5)))));
}

#[test]
fn parse_semver_between_given_badinput() {
    let result = parse_semver_between("1.2.3<3.4.5");
    assert_eq!(result, Err(NomErr(NomError { input: "<3.4.5", code: ErrorKind::Tag })));
}

#[test]
fn parse_range_with_spaces() {
    let result = parse_semver_between("1.2.3 +< 3.4.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,4,5)))));
}

#[test]
fn parse_str_starting_with_carot_major() {
    let result = parse_semver_carrot("^1");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,0,0), SemanticVersion::new(2,0,0)))));
}

#[test]
fn parse_str_starting_with_carot_minor() {
    let result = parse_semver_carrot("^2.5");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(2,5,0), SemanticVersion::new(2,6,0)))));
}

#[test]
fn parse_str_starting_with_carot_path() {
    let result = parse_semver_carrot("^3.4.2");
    assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(3,4,2), SemanticVersion::new(3,4,3)))));
}

#[test]
fn parse_str_exact() {
    let result = parse_semver_exact("1.2.3");
    assert_eq!(result, Ok(("", Range::exact(SemanticVersion::new(1,2,3)))));
}

#[test]
fn parse_semver_from_table() {
    let versions = vec![
        //("   1.23.4   ", Ok(("", Range::exact(SemanticVersion::new(1,23,4))))) ,
        ("1.23.4", Ok(("", Range::exact(SemanticVersion::new(1,23,4))))) ,
        // (" 1.2.3 +< 3 ", Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))),
        ("1.2.3+<3", Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))),
        ("^2.2", Ok(("", Range::between(SemanticVersion::new(2,2,0), SemanticVersion::new(2,3,0) ))))
    ];

    for (input,expected) in versions {
        assert_eq!(parse_semver_range(input), expected);
    }
}

#[test]
fn parsee_package_name_and_version() {
    let result = parse_package_version("maya-1.2.3");
    assert_eq!(result, Ok(("", ("maya", SemanticVersion::new(1,2,3)))));
}

#[test]
fn parsee_package_name_and_range() {
    let versions = vec![
        ("maya-1.23.4", Ok(("", ("maya",Range::exact(SemanticVersion::new(1,23,4)))))) ,
        ("houdini-1.2.3+<3", Ok(("",("houdini", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0)))))),
        ("houdini-1.2.3 +< 3.0.0", Ok(("",("houdini", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0)))))),
        ("nuke-^2.2", Ok(("", ("nuke", Range::between(SemanticVersion::new(2,2,0), SemanticVersion::new(2,3,0) )))))
    ];
    for (input, expected) in versions {
        let result = parse_package_range(input);
        assert_eq!(result, expected);
    
    }
}

