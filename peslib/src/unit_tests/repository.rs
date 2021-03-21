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
    root.push("../test_fixtures/repo");
    root
}

// construct a list of manifests for the supplied packages and versions,
// assuming that they are located in ROOT/test_fixtures/repo
fn expected_manifests_for(
    packages: &[&str], 
    // slice of vecs - because a slice of slices would require the inner slices 
    // to have the same size, we go with a a slice of Vec of &str
    versions: &[Vec<&str>],
    // manifest name (eg manifest.yaml or metadata/mani.yaml)
    manifest: &str
) -> Vec<PathBuf> {
    let mut returns = Vec::with_capacity(versions.len() * packages.len());
    for (cnt, package) in packages.iter().enumerate() {
        let mut root =get_repo_root();
        root.push(package);
        for version in &versions[cnt] {
            let mut root = root.clone();
            root.push(version);
            root.push(manifest);
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
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");
    let package_repo = PackageRepository::new(get_repo_root(), &plugin_mgr);
    assert_eq!(package_repo.root(), get_repo_root().as_path());
}

#[test]
fn manifest__returns_manifest_when_provided_with_extant_package_and_version() {
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");

    let package_repo = PackageRepository::new(get_repo_root(), &plugin_mgr);
    let manifest = package_repo.manifest("foo", "0.1.0");
    assert!(manifest.is_ok());
    let mut expect = get_repo_root();
    expect.push("foo/0.1.0/manifest.yaml");
    assert_eq!(manifest.unwrap(), expect);
}

#[test]
fn manifest__returns_err_when_provided_with_a_nonextant_package_and_version() {
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");
    
    // invalid package
    let package_repo = PackageRepository::new(get_repo_root(), &plugin_mgr);
    let manifest = package_repo.manifest("dontexist", "0.1.0");
    assert!(manifest.is_err());
    // invalid version
    let package_repo = PackageRepository::new(get_repo_root(), &plugin_mgr);
    let manifest = package_repo.manifest("foo", "10000.0.0");
    assert!(manifest.is_err());
    // invalid package and version
    let package_repo = PackageRepository::new(get_repo_root(), &plugin_mgr);
    let manifest = package_repo.manifest("dontexist", "10000000.1.0");
    assert!(manifest.is_err());
}

#[test]
fn manifests_for__returns_vec_of_pathbuf_to_manifest_files() {
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");
    let root = get_repo_root();
    let package_repo = PackageRepository::new(root.clone(), &plugin_mgr);
    let manifests = package_repo.manifests_for("foo").unwrap();
    // list of versions in ROOT/test_fixtures/repo/foo
    let versions = vec!["0.1.0", "0.2.0", "0.2.1", "0.2.2-beta"];
    // 
    let expected = expected_manifests_for(&["foo"], &[versions], "manifest.yaml");
    assert_eq!(expected.len(), manifests.len());
    for manifest in manifests {
        assert!(expected.iter().any(|x| &manifest == x));
    }
}

// this tests the case where we are asking the manifests method to ignore any non-release distributions. We 
// ensure that we are not picking up the foo-0.2.2-beta distribution, which should be getting filtered out.
#[test]
fn manifests__returns_vec_of_pathbuf_to_manifest_files_with_release_type_Release() {
    let root = get_repo_root();
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");

    let package_repo = PackageRepository::new(root.clone(), &plugin_mgr);
    let overrides = std::rc::Rc::new(Vec::new());
    let manifests: Vec<PathBuf> = package_repo.manifests(ReleaseType::Release,overrides).filter_map(|x| x.ok()).collect();
    // list of versions in ROOT/test_fixtures/repo/foo
    let packs = &["foo", "bar"];
    let versions = &[vec!["0.1.0", "0.2.0", "0.2.1"], vec!["0.1.0", "1.0.1"]];
    
    let expected = expected_manifests_for(packs, versions, "manifest.yaml");
    assert_eq!(expected.len(), manifests.len());
    for manifest in manifests {
        assert!(expected.iter().any(|x| &manifest == x));
    }
}


// this tests the case where we are setting the release_type to allow Beta release types. In our fixture data
// we have a 0.2.2-beta, which we test to make sure exists in the returned manifest path vec
#[test]
fn manifests__returns_vec_of_pathbuf_to_manifest_files_that_include_prereleases__when_release_type_is_alpha() {
    let root = get_repo_root();
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");

    let package_repo = PackageRepository::new(root.clone(), &plugin_mgr);
    let overrides = std::rc::Rc::new(Vec::new());
    let manifests: Vec<PathBuf> = package_repo.manifests(ReleaseType::Alpha, overrides).filter_map(|x| x.ok()).collect();
    // list of versions in ROOT/test_fixtures/repo/foo
    let packs = &["foo", "bar"];
    let versions = &[vec!["0.1.0", "0.2.0", "0.2.1", "0.2.2-beta"], vec!["0.1.0", "1.0.1"]];
    
    let expected = expected_manifests_for(packs, versions, "manifest.yaml");
    assert_eq!(expected.len(), manifests.len());
    for manifest in manifests {
        assert!(expected.iter().any(|x| &manifest == x));
    }
}

// This tests the case where we are generally filtering out any non Release ReleaseTypes, but where we explicitly 
// set the override for foo-0.2.2-beta via the distriubtions_override parameter. We assure that 0.2.2-beta, from the 
// test fixture data, is getting returned
#[test]
fn manifests__returns_vec_of_pathbuf_to_manifest_files_with_release_type_Release_and_explicit_override() {
    let root = get_repo_root();
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");

    let package_repo = PackageRepository::new(root.clone(), &plugin_mgr);
    let override_version = ("foo".to_string(), SemanticVersion::new(0,2,2, ReleaseType::Beta));
    let overrides = std::rc::Rc::new(vec![override_version]);
    let manifests: Vec<PathBuf> = package_repo.manifests(ReleaseType::Release,overrides).filter_map(|x| x.ok()).collect();
    // list of versions in ROOT/test_fixtures/repo/foo
    let packs = &["foo", "bar"];
    let versions = &[vec!["0.1.0", "0.2.0", "0.2.1", "0.2.2-beta"], vec!["0.1.0", "1.0.1"]];
    
    let expected = expected_manifests_for(packs, versions, "manifest.yaml");
    assert_eq!(expected.len(), manifests.len());
    for manifest in manifests {
        assert!(expected.iter().any(|x| &manifest == x));
    }
}