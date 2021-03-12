use pes_interface::RepoFinderService;
use std::path::PathBuf;
use log::debug;

#[no_mangle]
pub fn new_finder_service() -> Box<dyn RepoFinderService> {
    Box::new(DevRepoFinder::new())
}

pub struct DevRepoFinder;

impl DevRepoFinder {
    fn new() -> DevRepoFinder {
        DevRepoFinder
    }
}

impl RepoFinderService for DevRepoFinder {
    fn find_repo(&self) -> Vec<PathBuf> {
        vec![PathBuf::from(
            "/Users/jgerber/src/rust/pes/test_fixtures/repo_test",
           
        ), 
        PathBuf::from( "/home/jgerber/src/rust/pes/test_fixtures/repo_test")]
    }
}

// impl Drop for DevRepoFinder {
//     fn drop(&mut self) {
//         println!("\tDROPPING DevRepoFinder");
//     }
// }
