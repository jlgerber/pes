#![allow(non_snake_case)]

use super::*;

#[test]
fn from_strs__given_good_input__parses() {
    let result = Distribution::from_strs("maya", "1.2.3").expect("should parse");
    
    let expected = Distribution {
        name: "maya",
        version: SemanticVersion::new(1,2,3)
    };
    assert_eq!(result,expected);
}

#[test]
fn from_str__given_good_input__parses() {
    let result = Distribution::from_str("maya-3.2.1").expect("should parse");
    let expected = Distribution {
        name: "maya",
        version: SemanticVersion::new(3,2,1)
    };
    assert_eq!(result, expected);
}

#[test]
fn eq__given_two_equivalent_Distributions__works() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("maya-1.2.3").expect("should parse");
    assert_eq!(d1,d2);
}

#[test]
fn eq__given_two_different_Distributions__works() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("maya-2.2.3").expect("should parse");
    assert_ne!(d1,d2);
}

#[test]
fn eq__given_two_Distributions_with_same_package_and_different_versions__works() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("fauxmaya-1.2.3").expect("should parse");
    assert_ne!(d1,d2);
}

#[test]
fn ord__given_two_orded_distributions_with_same_packge__works() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("maya-1.2.4").expect("should parse");
    assert!(d1 < d2);
}

#[test]
fn ord__given_two_orded_distributions_with_different_package__works() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("amaya-1.2.3").expect("should parse");
    assert!(d1 > d2);
}

#[test]
fn package_eq__given_two_distributions_with_same_name__returns_true() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("maya-3.3.3").expect("should parse");
    assert!(d1.package_eq(&d2));
    assert!(d2.package_eq(&d1));
}

#[test]
fn package_eq__given_two_distributions_with_different_name__returns_true() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let d2 = Distribution::from_str("houdini-1.2.3").expect("should parse");
    assert!(!d1.package_eq(&d2));
    assert!(!d2.package_eq(&d1));
}

#[test]
fn display__implements_to_string() {
    let d1 = Distribution::from_str("maya-1.2.3").expect("should parse");
    let display_rep = d1.to_string();
    assert_eq!(display_rep.as_str(), "maya-1.2.3");
}