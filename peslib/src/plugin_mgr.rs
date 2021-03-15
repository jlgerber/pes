use crate::PesError;

use libloading::Library;
use log::info;
use pes_core::{ RepoFinderService, ManifestFinderService };
use std::path::PathBuf;


/// Load and store plugins
#[derive(Debug)]
pub struct PluginMgr {
    repo_finder: Library,
    manifest_finder: Library
 }



impl PartialEq for PluginMgr {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for PluginMgr {}

impl PluginMgr {
    
    /// new up an instance of PluginMgr, loading the plugins managed by the instance.
    pub fn new() -> Result<Self, PesError> {
        info!("building pluginmgr");
        let repo_finder = Self::new_repo_finder_service()?;
        let manifest_finder = Self::new_manifest_finder_service()?;
        Ok(Self { repo_finder, manifest_finder })
    }

    // in the test version, we build the path to the plugins using target/<debug|release>/
    #[cfg(test)]
    fn plugin_dir(plugin_name: &str) -> Result<std::path::PathBuf, PesError> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("target");
        #[cfg(debug_assertions)]
        path.push("debug");
        #[cfg(not(debug_assertions))]
        path.push("release");
        path.push(plugin_name);
        Ok(path)
    }

    /// in the non-test version, we build the path to the plugins, assuming that they
    /// are in a lib directory which is a peer of the parent directory. Of course, this 
    /// is terribly brittle. TODO: fetch the path to the plugin via a path lookup
    #[cfg(not(test))]
    fn plugin_dir(plugin_name: &str) -> Result<std::path::PathBuf, PesError> {
        let mut path = std::env::current_exe()?;
        path.pop();
        path.push("../lib");
        path.push(plugin_name);

        Ok(path)
    }

    fn new_repo_finder_service() -> Result<Library, PesError> {
        #[cfg(target_os = "macos")]
        let path = Self::plugin_dir("librepo_finder.dylib")?;

        #[cfg(target_os = "linux")]
        let path = Self::plugin_dir("librepo_finder.so")?;

        info!("Loading RepoFinder Library: {:?}", &path);
        let lib = unsafe { libloading::Library::new(path)? };
        
        Ok(lib)
    }

    fn new_manifest_finder_service() -> Result<Library, PesError> {
        #[cfg(target_os = "macos")]
        let path = Self::plugin_dir("libmanifest_finder.dylib")?;

        #[cfg(target_os = "linux")]
        let path = Self::plugin_dir("libmanifest_finder.so")?;

        info!("Loading ManifestFinder Library: {:?}", &path);
        let lib = unsafe { libloading::Library::new(path)? };

        Ok(lib)
    }

    /// retrieve a manifest given a distribution
    pub fn manifest_path_from_distribution<D: Into<PathBuf>>(&self, distribution: D) -> PathBuf {
        let new_service: libloading::Symbol<extern "Rust" fn() -> Box<dyn ManifestFinderService>> =
            unsafe { self.manifest_finder.get(b"new_finder_service").expect("unable to load finder service") };
        let manifest_finder = new_service();
        let distribution = distribution.into();
        manifest_finder.find_manifest(distribution)
    }

    /// retrieve a list of paths to package repositories
    pub fn repos(&self) -> Vec<std::path::PathBuf> {
        let new_service: libloading::Symbol<extern "Rust" fn() -> Box<dyn RepoFinderService>> =
        unsafe { self.repo_finder.get(b"new_finder_service").expect("unable to get finder service from plugin") };
        let repo_finder = new_service();
        let repo = repo_finder.find_repo();
        repo
    }

}
