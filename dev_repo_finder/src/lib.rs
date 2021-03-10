use pes_interface::RepoFinderService;
use std::path::PathBuf;

#[no_mangle]
pub extern "Rust" fn new_finder_service() -> Box<dyn RepoFinderService> {
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
       println!("find_repo called");
        vec![PathBuf::from("/Users/jgerber/src/rust/pes/src/rust/test_fixtures/repo")]
    }
}