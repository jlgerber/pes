use crate::constants::{ MANIFEST_FINDER_VARNAME, REPO_FINDER_VARNAME };
use crate::PesError;
use pes_interface::{ RepoFinderService, ManifestFinderService };
use std::path::PathBuf;

use log::info;

#[cfg(not(feature="segfault"))]
pub struct PluginMgr {}

#[cfg(feature="segfault")]
pub struct PluginMgr {
    repo_finder: Box<dyn RepoFinderService>,
 }

impl PluginMgr {
    /// retrieve an instance of the Plugin Manager
    #[cfg(not(feature="segfault"))]
    pub fn new() -> Result<Self, PesError> {
        info!("building pluginmgr");
        //let repo_finder = Self::new_repo_finder_service()?;
        //Ok(Self { repo_finder })
        Ok(Self {})
    }

    #[cfg(feature="segfault")]
    pub fn new() -> Result<Self, PesError> {
        info!("building pluginmgr");
        let repo_finder = Self::new_repo_finder_service()?;
        Ok(Self { repo_finder })
    }

    /// retrieve a manifest given a distribution
    pub fn manifest_path_from_distribution<D: Into<PathBuf>>(&self, distribution: D) -> PathBuf {
        #[cfg(target_os = "macos")]
        let dso_path = std::env::var(MANIFEST_FINDER_VARNAME)
            .unwrap_or_else(|_| {
                let mut path = std::env::current_exe().expect("cannot get current executable from env");
                path.pop();
                path.push("../lib");
                path.push("libmanifest_finder.dylib");
                path.into_os_string().into_string().expect("cannot convert path to string")
            });

        #[cfg(target_os = "linux")]
        let dso_path = std::env::var(MANIFEST_FINDER_VARNAME)
            .unwrap_or_else(|_| {
                let mut path = std::env::current_exe().expect("cannot get current executable from env");
                path.pop();
                path.push("../lib");
                path.push("libmanifest_finder.so");
                path.into_os_string().into_string().expect("cannot convert path to string")
            });
        info!("loading {:?}", &dso_path);
        let lib = unsafe { libloading::Library::new(dso_path.as_str()).expect("unable to load lib") };
        
        let new_service: libloading::Symbol<fn() -> Box<dyn ManifestFinderService>> =
            unsafe { lib.get(b"new_finder_service").expect("unable to get service") };
        info!("loaded  new finder service");
        let service = new_service();
        let distribution = distribution.into();
        service.find_manifest(distribution)
    }

    /// retrieve a list of paths to package repositories
    #[cfg(feature="segfault")]
    pub fn repos(&self) -> Vec<std::path::PathBuf> {
        info!("calling find_repo");
        //let repo = self.repo_finder.find_repo();
        //let repo_finder = self.new_repo_finder_service2().expect("unable to get plugin");
        let repo = self.repo_finder.find_repo();
        repo
    }

    /// retrieve a list of paths to package repositories
    #[cfg(not(feature="segfault"))]
    pub fn repos(&self) -> Vec<std::path::PathBuf> {
        #[cfg(target_os = "macos")]
        let dso_path = std::env::var(REPO_FINDER_VARNAME)
            .unwrap_or_else(|_| {
                let mut path = std::env::current_exe().expect("cannot get current executable from env");
                path.pop();
                path.push("../lib");
                path.push("librepo_finder.dylib");
                path.into_os_string().into_string().expect("cannot convert path to string")
            });

        #[cfg(target_os = "linux")]
        let dso_path = std::env::var(REPO_FINDER_VARNAME)
            .unwrap_or_else(|_| {
                let mut path = std::env::current_exe().expect("cannot get current executable from env");
                path.pop();
                path.push("../lib");
                path.push("librepo_finder.so");
                path.into_os_string().into_string().expect("cannot convert path to string")
            });
        info!("loading {:?}", &dso_path);
        let lib = unsafe { libloading::Library::new(dso_path.as_str()).expect("unable to load lib") };
        
        let new_service: libloading::Symbol<fn() -> Box<dyn RepoFinderService>> =
            unsafe { lib.get(b"new_finder_service").expect("unable to get service") };
        info!("loaded  new finder service");
        let service = new_service();
        service.find_repo()
    }

}
