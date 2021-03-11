use std::path::PathBuf;
pub trait RepoFinderService {
    /// Find repository paths
    fn find_repo(&self) -> Vec<PathBuf>;
}

pub trait ManifestFinderService {
    /// assuming the path to the distribution is valid, find_manifest constructs the path to 
    /// the manifest
    fn find_manifest(&self, distribution: PathBuf) -> PathBuf;
}
