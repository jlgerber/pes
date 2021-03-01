//! Components used to create, modify, read, and write  a package's manifest.
//!
//! This is achieved primarily through the `PackageManifest` struct
use std::path::{
    Path,
    PathBuf,
};
use serde::{
    Serialize, 
    Deserialize,
};

use crate::versioned_package::VersionedPackage;
use crate::error::PesError;


pub mod package_manifest;
//pub mod package_range;
pub(crate) mod package_target;

pub use package_manifest::PackageManifest;
//pub use package_range::PackageRange;
pub(crate) use package_target::PackageTarget;
/// custom type mapping a target name to a PackageTarget
pub type TargetMap = indexmap::IndexMap<String, PackageTarget>;

// manifest wraps inner manifest with metadata
#[derive(Debug,  Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    /// The root directory of the package
    root: PathBuf,
    inner: PackageManifest
}

impl Manifest {
    pub fn new<P: Into<PathBuf>>(package_root: P, package_manifest: PackageManifest) -> Self {
        Self {
            root: package_root.into(),
            inner: package_manifest
        }
    }
    
    pub fn distribution(&self) -> String {
        self.inner.distribution()
    }

    /// retrieve a list of requires for the supplied target
    pub fn get_requires(&self, target: &str) -> Result<Vec<VersionedPackage>, PesError> {
        self.inner.get_requires(target)
    }

    pub fn package_root(&self) -> &Path {
        self.root.as_path()
    }

    /// validate the manifest, making sure that the package manifest is valid and that the 
    /// provided root path exists
    pub fn validate(&self) -> Result<(), PesError> {
        let _ = self.inner.validate()?;
        if !self.root.exists() {
            let Manifest{root, ..} = self;
            Err(PesError::MissingPath(root.to_path_buf()))
        } else {

            Ok(())
        }
    }

}