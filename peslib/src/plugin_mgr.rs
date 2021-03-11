use pes_interface::RepoFinderService;
use crate::PesError;

pub struct PluginMgr {
    repo_finder: Box<dyn  RepoFinderService>
}

impl PluginMgr {
    /// retrieve an instance of the Plugin Manager
    pub fn new() -> Result<Self, PesError> {
        let repo_finder = Self::new_repo_finder_service()?;
        Ok(
            Self {
                repo_finder
            }
        )
    }

    /// retrieve a list of paths to package repositories
    pub fn repos(&self) -> Vec<std::path::PathBuf> {
        let repo = self.repo_finder.find_repo();
        repo
    }
    
    fn new_repo_finder_service() -> Result<Box<dyn RepoFinderService>, PesError> {
        #[cfg(target_os = "macos")]
        let dso_path = std::env::var(REPO_FINDER_VARNAME).unwrap_or_else(|_| "target/release/librepo_finder.dylib".to_string());
        
        #[cfg(target_os = "linux")]
        let dso_path = std::env::var(REPO_FINDER_VARNAME).unwrap_or_else(|_| "target/release/librepo_finder.dylib".to_string());
        
        #[cfg(target_os = "macos")]
        let lib = unsafe { libloading::Library::new(dso_path.as_str())?};
        
        #[cfg(target_os = "linux")]
        let lib = unsafe { libloading::Library::new("target/release/librepo_finder.so")?};

        let new_service: libloading::Symbol<extern "Rust" fn() -> Box<dyn RepoFinderService>> =
            unsafe {lib.get(b"new_finder_service")?};
        Ok(new_service())
    }
}