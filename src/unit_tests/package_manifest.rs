use super::*;
use PackageTarget;
use pubgrub::version::SemanticVersion;
//use pubgrub::range::Range;
use crate::VersionedPackage;
use crate::manifest::TargetMap;

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


#[test]
fn can_deserialize_from_str() {
    let manifest = PackageManifest::from_str(P1);
    let mut run_target = PackageTarget::new();
    run_target.requires(
        "maya-plugins", 
        "^4.3"
    );
    run_target.requires(
        "maya-core", 
        "2+<4"
    );

    let mut build_target = PackageTarget::new();
    build_target.include("run").unwrap();
    build_target.requires(
        "maya", "1.2.3+<4"
        );
    let mut target_map = TargetMap::new();
    target_map.insert("run".into(), run_target);
    target_map.insert("build".into(), build_target);

    assert_eq!(manifest.unwrap(), 
        PackageManifest {
            schema: 1,
            name: "mypackage".into(),
            version: SemanticVersion::new(1,2,3),
            description: "this is the description".into(),
            targets: target_map,
        }
    );
}


#[test]
fn can_get_requires_without_includes() {
    let manifest = PackageManifest::from_str(P1).unwrap();
    let requires = manifest.get_requires("run");
    let expected = vec![
        VersionedPackage::from_strs("maya-plugins", "^4.3").unwrap(),
        VersionedPackage::from_strs("maya-core", "2+<4").unwrap(),
    ];
    assert_eq!(requires.unwrap(), expected );
}

#[test]
fn can_get_requires_when_includes_are_present() {
    let manifest = PackageManifest::from_str(P1).unwrap();
    let requires = manifest.get_requires("build");
    let expected = vec![
        VersionedPackage::from_strs("maya-plugins", "^4.3").unwrap(),
        VersionedPackage::from_strs("maya-core", "2+<4").unwrap(),
        VersionedPackage::from_strs("maya", "1.2.3+<4").unwrap()
    ];
    assert_eq!(requires.unwrap(), expected );
}