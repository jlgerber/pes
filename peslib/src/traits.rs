//! Traits defined in the `pes` crate live here
use std::path::{Path, PathBuf};
use std::ffi::CString;

use generator::Generator;

use crate::{Manifest, PesError};

/// Trait to provide a means to retrieve variables
pub trait VarProvider<'a> {
    type Returns;
    type Key;
    type Value;

    fn insert<K: Into<Self::Key>, V: Into<Self::Value> >(&mut self, k: K, v: V) -> Option<Self::Value>; 
    fn get(&'a self, value: impl AsRef<str>) -> Option<Self::Returns>;
}


/// Trait to provide an alternative, falible constructor from a &str
pub trait FrmStr {
    type FrmStrErr;

    /// Given a str, construct an instance of Self
    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized;
}


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

    fn root(&self) -> &Path;

    /// retrieve a manifest for the provided package and version
    fn manifest<P: AsRef<str>, V: AsRef<str> >(&self, package: P, version: V) -> Result<Self::Manifest, Self::Err>;
    
    /// Retrieve the manifest for the provided distribution
    fn manifest_for<P: AsRef<str> >(&self, distribution: P) -> Result<Self::Manifest, PesError>;

    /// retrieve manifests for the provided package
    fn manifests_for<P: AsRef<str> >(&self, package: P) -> Result<Vec<Self::Manifest>, PesError>;

    /// retrieve a generator over all of the manifests in a repository
    fn manifests(&self) -> Generator<'_, (), Result<Self::Manifest, Self::Err>> ;

    /// Retrieve generator over distributions in repository
    fn distributions(&self)-> Generator<'_, (), Result<Self::Distribution, Self::Err>> ;

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