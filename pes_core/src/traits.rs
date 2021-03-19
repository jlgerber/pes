use std::path::PathBuf;
use std::str::FromStr;
use std::fmt::{Display, Debug};
use std::hash::Hash;
/// trait to find repositories in the job system
pub trait RepoFinderService {
    /// Find repository paths
    fn find_repo(&self) -> Vec<PathBuf>;
}

/// trait to locate the manifest within a distribution
pub trait ManifestFinderService {
    /// assuming the path to the distribution is valid, find_manifest constructs the path to 
    /// the manifest
    fn find_manifest(&self, distribution: PathBuf) -> PathBuf;
}

/// trait which must be implemented to satisfy releasetype. This trait is not currently used. 
/// The plan is to use it and perhaps provide a plugin capability as well.
pub trait ReleaseTypeProvider: Debug + Display + FromStr + Copy + Clone + Ord + PartialOrd + Eq + PartialEq + Hash {
    type Err: std::error::Error;
    /// Return the variant which represents a release.
    fn release() -> Self;
}