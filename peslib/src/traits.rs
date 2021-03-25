//! Traits defined in the `pes` crate live here
use std::path::{Path, PathBuf};
use std::ffi::CString;

use generator::Generator;
use pes_core::ReleaseType;

use crate::{Manifest, PesError, SemanticVersion};



/// Trait to provide basic metedata for a package manifest
pub trait MetadataProvider {
    type Version;
    type Error;

    fn version(&self) -> Result<Self::Version, Self::Error>;

    fn name(&self) -> Result<&str, Self::Error>;

    fn repository<P: AsRef<Path>>(&self) -> Result<P, Self::Error>;
}

/// Define a base environment as a vector of CStrings of the form "var=value"
pub trait BaseEnv {
    fn base_env(&self) -> Vec<CString>;
    fn keys(&self) -> &'static [&'static str];
}


pub trait Repository: std::fmt::Debug {
    type Manifest: AsRef<Path>;
    type Distribution: AsRef<Path>;
    type Err: std::error::Error;
     
    /// Retrieve the root of the repository
    // todo: shouldnt this be defined as an associated type via something line Borrow<Root>
    fn root(&self) -> &Path;

    /// retrieve a manifest for the provided package and version
    fn manifest<P: AsRef<str>, V: AsRef<str> >(&self, package: P, version: V) -> Result<Self::Manifest, Self::Err>;
    
    /// Retrieve the manifest for the provided distribution
    fn manifest_for<P: AsRef<str> >(&self, distribution: P) -> Result<Self::Manifest, PesError>;

    /// Retrieve manifests for the provided package whose release_type is greater than or equal to the provided `min_release_type`.
    /// This would typlically be used to filter out pre-releases by passing in `ReleaseType::Release`, or return all
    /// release types by specifying `ReleaseType::Alpha`.
    fn manifests_for<P: AsRef<str> >(&self, package: P, min_release_type: ReleaseType) -> Result<Vec<Self::Manifest>, PesError>;

    /// Retrieve a generator over all of the manifests in a repository for which the 
    /// predicate evaluates to true. One may supply a `min_release_type` which filters out distributions whose release types are 
    /// less than the `nim_release_type`. One may, for example, filter out pre-releases, by supplying `ReleaseType::Release`, or 
    /// return all pre-releases by supplying `ReleaseType::Alpha`. One may override the filtering behavior by providing specific  
    /// distributions to return, regardless of `min_release_type` via the `distributions_override`. This would typeically be used
    /// in cases where one is solving for a user supplied distribution which is a pre-release, but one does not want to pick up 
    /// transitive pre-releases. In this case, one may set `min_release_type` to `ReleaseType::Release` and then pass in the
    /// specific pre-release distribution via `distributions_override`.
    fn manifests(&self, min_release_type: ReleaseType, distributions_override: std::rc::Rc<Vec<(String, SemanticVersion)>>) -> Generator<'_, (), Result<Self::Manifest, Self::Err>> ;

    /// Retrieve generator over distributions in repository
    fn distributions(&self, min_release_type: ReleaseType, distributions_override: std::rc::Rc<Vec<(String, SemanticVersion)>>)-> Generator<'_, (), Result<Self::Distribution, Self::Err>> ;

    /// determine whether the repository has the distribution
    fn has_distribution<D: AsRef<str>>(&self, distribution: D) -> Result<bool,Self::Err>;
}

/// Locate a manifest given a path to the root of a distribution. This trait allows us to 
/// define different package layouts. 
pub trait ManifestLocationProvider: std::fmt::Debug {
    /// locate a manifest within a distribution. 
    fn find<P: Into<PathBuf>>(&self, distribution: P) -> PathBuf;
    /// construct a manifest
    fn manifest<P: Into<PathBuf>>(&self, distribution: P) -> Result<Manifest, PesError>;
}