use pes_interface::ManifestFinderService;
use std::path::PathBuf;

#[no_mangle]
pub extern "Rust" fn new_finder_service() -> Box<dyn ManifestFinderService> {
    Box::new(DevManifestFinder::new())
}

pub struct DevManifestFinder;
   
impl DevManifestFinder {
    fn new() -> DevManifestFinder {
        DevManifestFinder {}
    }
}

impl ManifestFinderService for DevManifestFinder {
    fn find_manifest(&self, mut distribution: PathBuf) -> PathBuf {
        
        distribution.push("manifest.yaml");
        distribution
    }
}