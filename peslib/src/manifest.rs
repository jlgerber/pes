//! Components used to create, modify, read, and write  a package's manifest.
//!
//! This is achieved primarily through the `PackageManifest` struct
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::PesError;
use crate::distribution_range::DistributionRange;

pub mod package_manifest;
pub(crate) mod package_target;

pub use package_manifest::PackageManifest;

// manifest wraps inner manifest with metadata
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    /// The root directory of the package
    root: PathBuf,
    inner: PackageManifest,
}

impl Manifest {
    pub fn new<P: Into<PathBuf>>(package_root: P, package_manifest: PackageManifest) -> Self {
        Self {
            root: package_root.into(),
            inner: package_manifest,
        }
    }

    /// Construct a manifest given a path to the manifest. We expect the manifest to be found
    /// at the root of the distribution.
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Result<Self, PesError> {
        let manifest = path.into();
        let mut root = manifest.clone();
        root.pop();
        let package_manifest = PackageManifest::from_file(manifest)?;
        Ok(Self::new(root, package_manifest))
    }
    /// Construct a manifest but do not apply additional validation
    pub fn from_path_unchecked<P: Into<PathBuf>>(path: P) -> Result<Self, PesError> {
        let manifest = path.into();
        let mut root = manifest.clone();
        root.pop();
        let package_manifest = PackageManifest::from_file_unchecked(manifest)?;
        Ok(Self::new(root, package_manifest))
    }

    pub fn distribution(&self) -> String {
        self.inner.distribution()
    }

    /// retrieve a list of requires for the supplied target
    pub fn get_requires(&self, target: &str) -> Result<Vec<DistributionRange>, PesError> {
        self.inner.get_requires(target)
    }

    pub fn environment(&self) -> indexmap::map::Iter<String, String> {
        self.inner.environment.iter()
    }

    /// Retrive the path to the package root
    pub fn package_root(&self) -> &Path {
        self.root.as_path()
    }

    /// validate the manifest, making sure that the package manifest is valid and that the
    /// provided root path exists
    pub fn validate(&self) -> Result<(), PesError> {
        let _ = self.inner.validate()?;
        if !self.root.exists() {
            let Manifest { root, .. } = self;
            Err(PesError::MissingPath(root.to_path_buf()))
        } else {
            Ok(())
        }
    }
}
