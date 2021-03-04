//! tests for the lockfile
#![allow(non_snake_case)]

use super::*;
use testutils::{rand_file, tempfile};
use std::fs::File;
//use std::io::prelude::*;


const LOCKFILE1: &str = 
r#"
schema = 1
request = "pez env -p bar-2.4.3 -t run -t build"
author = "jgerber"

[lock.run]
foo = "1.2.3"
bar = "2.4.3"

[lock.build]
foo = "1.2.4"
bar = "2.0.1"
somelib = "1.2.3"
"#;

#[test]
fn from_str__when_given_good_data__succeeds() {
    let lf = LockFile::from_str(LOCKFILE1);
    assert!(lf.is_ok());
}

#[test]
fn from_file__when_given_good_data__succeeds() {
    let path = rand_file(Some("pes_lockfile_"), Some(".yaml"), &Path::new("/tmp"));
    let mut file = File::create(path.as_path()).unwrap();
    file.write_all(LOCKFILE1.as_bytes()).expect("could not write lockfile to temp file");
    let lf = LockFile::from_file(path.as_path()).expect("Could not load LockFile from file");
    let expected = LockFile::from_str(LOCKFILE1).expect("Could not load lockfile from string");
    assert_eq!(lf, expected);
}

#[test]
fn add_dist__when_given_a_new_target_and_dist__succeeds() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    
    // build up expected
    let mut lock = LockMap::new();
    let mut versionmap = VersionMap::new();
    versionmap.insert("maya".to_string(), SemanticVersion::new(1,0,0));
    lock.insert("run".to_string(), versionmap);

    let expect = LockFile {
        schema: 1,
        request: String::new(),
        author: "jgerber".to_string(),
        lock,
    };
    assert_eq!(lockfile, expect);
}

#[test]
fn add_dist__when_given_a_existing_target_and_dist__succeeds() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("run", "maya-2.0.0").unwrap();
    // build up expected
    let mut lock = LockMap::new();
    let mut versionmap = VersionMap::new();
    versionmap.insert("maya".to_string(), SemanticVersion::new(2,0,0));
    lock.insert("run".to_string(), versionmap);

    let expect = LockFile {
        schema: 1,
        request: String::new(),
        author: "jgerber".to_string(),
        lock,
    };
    assert_eq!(lockfile, expect);
}

#[test]
fn version__when_given_extant_target_and_package__returns_some_version() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("run", "nuke-2.4.0").unwrap();

    let version = lockfile.version("run", "nuke");
    let expected = SemanticVersion::new(2,4,0);
    assert_eq!(version, Some(&expected));

}

#[test]
fn version__when_given_non_extant_target_and_package__returns_none() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("run", "nuke-2.4.0").unwrap();

    let version = lockfile.version("run", "nothing");
    assert_eq!(version, None);

}

#[test]
fn targets__returns_iterator_over_targets() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("build", "nuke-2.4.0").unwrap();

    let mut targets = lockfile.targets().collect::<Vec<_>>();
    targets.sort();
    assert_eq!(targets, vec!["build", "run"]);
}

#[test]
fn dists_for__when_given_valid_target__returns_some_iter() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("run", "nuke-2.4.0").unwrap();

    let result = lockfile.dists_for("run").unwrap();
    let result = result.collect::<Vec<_>>();

    let expected = vec![
        ("maya", SemanticVersion::new(1,0,0)),
        ("nuke", SemanticVersion::new(2,4,0))
    ];

    // a bit of a pain because the iterator doesnt return in a known order.
    assert_eq!(result.len(), expected.len());
    for record in result {
        let mut exists = false;
        for e in &expected {
            if e.0 == record.0.as_str() && record.1 == &e.1 {
                exists = true;
            }
        }
        assert!(exists);
    }
}

#[test]
fn dists_for__when_given_invalid_target__returns_None() {
    let mut lockfile = LockFile::new("", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("run", "nuke-2.4.0").unwrap();

    let result = lockfile.dists_for("walk");
    assert!(result.is_none());
}

#[test]
fn to_writer__when_given_valid_file__writes_file() {
    let mut file = tempfile::NamedTempFile::new().expect("unable to create temp file");
    let mut lockfile = LockFile::new("pes rocks", "jgerber");
    lockfile.add_dist("run", "maya-1.0.0").unwrap();
    lockfile.add_dist("run", "nuke-2.4.0").unwrap();

    lockfile.to_writer(&mut file, true).expect("unable to write tempfile");
    let lockfile2 = LockFile::from_file(file.path()).expect("unable to open temp file");
    assert_eq!(lockfile, lockfile2);
}