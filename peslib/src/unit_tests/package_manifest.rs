#![allow(non_snake_case)]

use super::*;
use crate::manifest::package_target::PackageTarget;
use crate::{SemanticVersion, ReleaseType};
//use pubgrub::range::Range;
use crate::DistributionRange;
use crate::EnvMap;
use crate::TargetMap;
use std::path::PathBuf;

fn get_repo_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("../test_fixtures/repo");
    root
}

const P1: &str = r#"
---
schema: 1
name: mypackage
version: 1.2.3
description: this is the description

targets:
    run:
        requires:
            maya-plugins: ^4.3
            maya-core: 2+<4
    build:
        include:
            - run
        requires:
            maya: 1.2.3+<4
"#;

const P2: &str = r#"
---
schema: 1
name: mypackage
version: 1.2.3
description: this is the description
"#;

const INVALID_MANIFEST_1: &str = r#"
---
schema: 1
name: mypackage
version: 1.2.3
description: this is the description

targets:
    run:
        requires:
            maya-plugins: flopsy
    build:
        include:
            - run
        requires:
            maya: 1.2.3+<4
"#;

const INVALID_MANIFEST_VER: &str = r#"
---
schema: 1
name: mypackage
version: invalid
description: this is the description

targets:
    run:
        requires:
            maya-plugins: 1.2.3
    build:
        include:
            - run
        requires:
            maya: 1.2.3+<4
"#;

#[test]
fn from_str_unchecked__succeeds() {
    let manifest = PackageManifest::from_str_unchecked(P1);
    let mut run_target = PackageTarget::new();
    run_target.requires("maya-plugins", "^4.3");
    run_target.requires("maya-core", "2+<4");

    let mut build_target = PackageTarget::new();
    build_target.include("run").unwrap();
    build_target.requires("maya", "1.2.3+<4");
    let mut target_map = TargetMap::new();
    target_map.insert("run".into(), run_target);
    target_map.insert("build".into(), build_target);

    assert_eq!(
        manifest.unwrap(),
        PackageManifest {
            schema: 1,
            name: "mypackage".into(),
            version: SemanticVersion::new(1, 2, 3,ReleaseType::Release),
            description: "this is the description".into(),
            targets: target_map,
            environment: EnvMap::new()
        }
    );
}

#[test]
fn from_str__succeeds_when_given_valid_manifest_str() {
    let manifest = PackageManifest::from_str(P1);
    let mut run_target = PackageTarget::new();
    run_target.requires("maya-plugins", "^4.3");
    run_target.requires("maya-core", "2+<4");

    let mut build_target = PackageTarget::new();
    build_target.include("run").unwrap();
    build_target.requires("maya", "1.2.3+<4");
    let mut target_map = TargetMap::new();
    target_map.insert("run".into(), run_target);
    target_map.insert("build".into(), build_target);

    assert_eq!(
        manifest.unwrap(),
        PackageManifest {
            schema: 1,
            name: "mypackage".into(),
            version: SemanticVersion::new(1, 2, 3,ReleaseType::Release),
            description: "this is the description".into(),
            targets: target_map,
            environment: EnvMap::new()
        }
    );
}

#[test]
fn from_str__succeeds_when_given_valid_manifest_str_without_targets() {
    let manifest = PackageManifest::from_str(P2);
    let target_map = TargetMap::new();

    assert_eq!(
        manifest.unwrap(),
        PackageManifest {
            schema: 1,
            name: "mypackage".into(),
            version: SemanticVersion::new(1, 2, 3,ReleaseType::Release),
            description: "this is the description".into(),
            targets: target_map,
            environment: EnvMap::new()
        }
    );
}

// if we provide a manifest with a version that is not valid, from_str
// should return a Result::Err
#[test]
fn from_str__errors_when_given_invalid_manifest_str() {
    let manifest = PackageManifest::from_str(INVALID_MANIFEST_1);
    assert!(manifest.is_err())
}

// If we provide a target requires with an invalid version str, from_str
// should return a Result::Err
#[test]
fn from_str__errors_when_given_manifest_str_with_invalid_version() {
    let manifest = PackageManifest::from_str(INVALID_MANIFEST_VER);
    assert!(manifest.is_err())
}

// Test basic target without an include
#[test]
fn get_requires__succeeds_when_called_on_target_without_includes() {
    let manifest = PackageManifest::from_str(P1).unwrap();
    let requires = manifest.get_requires("run");
    let expected = vec![
        DistributionRange::from_strs("maya-plugins", "^4.3").unwrap(),
        DistributionRange::from_strs("maya-core", "2+<4").unwrap(),
    ];
    assert_eq!(requires.unwrap(), expected);
}

// verify that get_requries works when called on target that includes requirements
// from other targets in addition to its on requires
#[test]
fn get_requires__succeeds_when_called_on_target_with_includes() {
    let manifest = PackageManifest::from_str(P1).unwrap();
    let requires = manifest.get_requires("build");
    let expected = vec![
        DistributionRange::from_strs("maya-plugins", "^4.3").unwrap(),
        DistributionRange::from_strs("maya-core", "2+<4").unwrap(),
        DistributionRange::from_strs("maya", "1.2.3+<4").unwrap(),
    ];
    assert_eq!(requires.unwrap(), expected);
}

// validate_manifest() should return Result::Ok when called on a valid manifest instance
#[test]
fn validate_manifest__is_ok_when_called_on_good_PackageManifest() {
    let manifest = PackageManifest::from_str_unchecked(P1).unwrap();
    assert!(manifest.validate().is_ok());
}

// However, validate_manifest should return a Result::Err if called on a manifest
// which has invalid version range for one of a target's requires
#[test]
fn validate_manifest__returns_err_when_called_on_invalid_manifest() {
    let manifest = PackageManifest::from_str_unchecked(INVALID_MANIFEST_1).unwrap();
    let valid = manifest.validate();
    assert!(valid.is_err());
}

#[test]
fn from_file__when_given_path_to_valid_manifest_produces_valid_manifest() {
    let mut path = get_repo_root();
    path.push("foo/0.1.0/manifest.yaml");
    let manifest = PackageManifest::from_file(path);
    assert!(manifest.is_ok());
    let manifest = manifest.unwrap();
    let valid = manifest.validate();
    assert!(valid.is_ok());
}

#[test]
fn distribution__returns_name_version() {
    let mut path = get_repo_root();
    path.push("foo/0.1.0/manifest.yaml");
    let manifest = PackageManifest::from_file(path);
    assert!(manifest.is_ok());
    let manifest = manifest.unwrap();
    let dist = manifest.distribution();
    assert_eq!(dist.as_str(), "foo-0.1.0");
}
