#![allow(non_snake_case)]
use super::*;
use std::path::PathBuf;

//---------------------//
//    UTILITIES        //
//---------------------//

// Retrieve the root of the testing package repo, which 
// should be located in $ROOT/test_fixtures/repo
fn get_repo_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("test_fixtures/repo");
    root
}

// construct a list of manifests for the supplied packages and versions,
// assuming that they are located in ROOT/test_fixtures/repo
fn expected_manifests_for(
    packages: &[&str], 
    // slice of vecs - because a slice of slices would require the inner slices 
    // to have the same size, we go with a a slice of Vec of &str
    versions: &[Vec<&str>]
) -> Vec<PathBuf> {
    let mut returns = Vec::with_capacity(versions.len() * packages.len());
    for (cnt, package) in packages.iter().enumerate() {
        let mut root =get_repo_root();
        root.push(package);
        for version in &versions[cnt] {
            let mut root = root.clone();
            root.push(version);
            root.push("manifest.yaml");
            returns.push(root);
        }
    }

    returns
}

//------------//
//   TESTS    //
//------------//

#[test]
fn get_root__returns_path() {
   let package_repo = PackageRepository::new(get_repo_root());
   assert_eq!(package_repo.root(), get_repo_root().as_path());
}

#[test]
fn manifest__returns_manifest_when_provided_with_extant_package_and_version() {
    let package_repo = PackageRepository::new(get_repo_root());
    let manifest = package_repo.manifest("foo", "0.1.0");
    assert!(manifest.is_ok());
    let mut expect = get_repo_root();
    expect.push("foo/0.1.0/manifest.yaml");
    assert_eq!(manifest.unwrap(), expect);
}

#[test]
fn manifest__returns_err_when_provided_with_a_nonextant_package_and_version() {
    // invalid package
    let package_repo = PackageRepository::new(get_repo_root());
    let manifest = package_repo.manifest("dontexist", "0.1.0");
    assert!(manifest.is_err());
    // invalid version
    let package_repo = PackageRepository::new(get_repo_root());
    let manifest = package_repo.manifest("foo", "10000.0.0");
    assert!(manifest.is_err());
    // invalid package and version
    let package_repo = PackageRepository::new(get_repo_root());
    let manifest = package_repo.manifest("dontexist", "10000000.1.0");
    assert!(manifest.is_err());
}

#[test]
fn manifests_for__returns_vec_of_pathbuf_to_manifest_files() {
    let root = get_repo_root();
    let package_repo = PackageRepository::new(root.clone());
    let manifests = package_repo.manifests_for("foo").unwrap();
    // list of versions in ROOT/test_fixtures/repo/foo
    let versions = vec!["0.1.0", "0.2.0", "0.2.1"];
    // 
    let expected = expected_manifests_for(&["foo"], &[versions]);
    for manifest in manifests {
        assert!(expected.iter().any(|x| &manifest == x));
    }
}

#[test]
fn manifests__returns_vec_of_pathbuf_to_manifest_files() {
    let root = get_repo_root();
    let package_repo = PackageRepository::new(root.clone());
    let manifests: Vec<PathBuf> = package_repo.manifests().map(|x| x.unwrap()).collect();
    // list of versions in ROOT/test_fixtures/repo/foo
    let packs = &["foo", "bar"];
    let versions = &[vec!["0.1.0", "0.2.0", "0.2.1"], vec!["0.1.0", "1.0.1"]];
    
    let expected = expected_manifests_for(packs, versions);
    assert_eq!(expected.len(), manifests.len());
    for manifest in manifests {
        assert!(expected.iter().any(|x| &manifest == x));
    }
}
