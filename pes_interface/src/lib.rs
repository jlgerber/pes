use std::path::PathBuf;
pub trait RepoFinderService {
    /// Find repository paths
    fn find_repo(&self) -> Vec<PathBuf>;
}
