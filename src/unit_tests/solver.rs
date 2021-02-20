#![allow(non_snake_case)]

use super::*;
use std::path::PathBuf;
use pubgrub::report::DefaultStringReporter;
use pubgrub::error::PubGrubError;

use crate::versioned_package::VersionedPackage;
use crate::repository::PackageRepository;
use crate::error::PesError;

fn get_repo_root(repo_name: &str) -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("test_fixtures");
    root.push(repo_name);
    root
}

const MANI: &'static str = "manifest.yaml";

#[test]
fn add_repository__given_a_repository_with_a_package_without_run_target() {
    let pkg_repo = PackageRepository::new(get_repo_root("repo2"), MANI);
    let mut solver = Solver::new();
    solver.add_repository(&pkg_repo);
    let versions: Vec<&SemanticVersion> = solver.versions("foo").unwrap().collect();
    assert_eq!(versions.len(),1);
    let solution =  solver.solve_from_str("foo-0.1.0");
      
    assert!(solution.is_ok());
}

#[test]
fn convert_request_str__given_space_separated_list() {
    let request = "maya  maya_plugins-1.2.3+<3";
    let result = Solver::convert_request_str(request);
    assert_eq!(result, vec![
        VersionedPackage::from_str("maya").unwrap(),
        VersionedPackage::from_str("maya_plugins-1.2.3+<3").unwrap()
    ]);
}


// #[test]
// fn solver_test_01() {
    
//     let package_repo = PackageRepository::new(get_repo_root("repo"), MANI);
//     // let manifests: Vec<PathBuf> = package_repo.manifests().filter_map(|x| x.ok()).collect();
//     // assert_eq!(manifests, Vec::<PathBuf>::new());
//     let mut solver = Solver::new();
//     solver.add_repository(&package_repo);
//     let versions: Vec<&SemanticVersion> = solver.versions("foo").unwrap().collect();
//     assert_eq!(versions, Vec::<&SemanticVersion>::new());
// }
#[test]
fn solver_test_02() {
    
    let package_repo = PackageRepository::new(get_repo_root("repo"), MANI);
    let mut solver = Solver::new();
    solver.add_repository(&package_repo);
    let solution =  solver.solve_from_str("bar-0.1.0");
    assert!(solution.is_ok());

   // assert_eq!(solution.unwrap_err().to_string(), "foo".to_string());
}