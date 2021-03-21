#![allow(non_snake_case)]

use super::*;
use crate::repository::PackageRepository;
use crate::distribution_range::DistributionRange;
use crate::plugin_mgr::PluginMgr;

use std::path::PathBuf;

//-------------//
//   HELPERS   //
//-------------//

fn get_repo_root(repo_name: &str) -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("../test_fixtures");
    root.push(repo_name);
    root
}

//-------------//
//    TESTS    //
//-------------//

#[test]
fn add_repository__given_a_repository_with_a_package_without_run_target__succeeds() {
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");

    let pkg_repo = PackageRepository::new(get_repo_root("repo2"), &plugin_mgr);
    let mut solver = Solver::new();
    solver
        .add_repository(&pkg_repo, ReleaseType::Release, Rc::new(Vec::new()))
        .expect("should be able to add repository");
    let versions: Vec<&SemanticVersion> = solver.versions("foo").unwrap().collect();
    assert_eq!(versions.len(), 1);
    let solution = solver.solve_from_str("foo-0.1.0");

    assert!(solution.is_ok());
}

#[test]
fn convert_request_str__given_space_separated_list__succeeds() {
    let request = "maya  maya_plugins-1.2.3+<3";
    let result = Solver::convert_request_str(request);
    assert_eq!(
        result,
        vec![
            DistributionRange::from_str("maya").unwrap(),
            DistributionRange::from_str("maya_plugins-1.2.3+<3").unwrap()
        ]
    );
}

#[test]
fn solve_from_str__given_a_valid_distribution__succeeds() {
    let plugin_mgr = PluginMgr::new().expect("unable to load plugin manager");
    let package_repo = PackageRepository::new(get_repo_root("repo"), &plugin_mgr);
    let mut solver = Solver::new();
    solver
        .add_repository(&package_repo, ReleaseType::Release, Rc::new(Vec::new()))
        .expect("should be able to add repository");
    let solution = solver.solve_from_str("bar-0.1.0");
    assert!(solution.is_ok());
}
