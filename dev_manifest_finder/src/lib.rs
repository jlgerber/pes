use pes_interface::ManifestFinderService;
use std::path::PathBuf;

#[no_mangle]
pub extern "Rust" fn new_finder_service() -> Box<dyn ManifestFinderService> {
    Box::new(DevManifestFinder::new())
}

/// This plugin is responsible for finding the manifest within a package. 
/// In our case, we expect the manifest to live under the root of the distribution.
/// However, the plugin affords the ability to change this arbitrarily. 
// TODO: return a Manifest instance instead of a path. This will allow us to use a database to store the data.

pub struct DevManifestFinder;
   
impl DevManifestFinder {
    fn new() -> DevManifestFinder {
        DevManifestFinder {}
    }
}

impl ManifestFinderService for DevManifestFinder {
    // This implementation is dead simple. The manifest.yaml file is 
    // expected to be in the root of the package.
    fn find_manifest(&self, mut distribution: PathBuf) -> PathBuf {
        distribution.push("manifest.yaml");
        distribution
    }
}