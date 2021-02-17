use std::path::Path;

use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use pubgrub::version::SemanticVersion;

use crate::error::PesError;
use crate::manifest::PackageTarget;

/// Struct representation of manifest for package
#[derive(Debug,  Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageManifest {
    pub schema: u32,
    pub name: String,
    pub version: SemanticVersion,
    pub description: String,
    pub targets: IndexMap<String, PackageTarget>
}

impl PackageManifest {

    /// Construct a PackageManifest from a str
    pub fn from_str(value: &str) -> Result<Self, PesError> {
        Ok(serde_yaml::from_str(value)?)
    }

    /// Construct a PackageManifest from a readable file
    pub fn from_file<F>(value: F) -> Result<Self, PesError> 
    where
        F: AsRef<Path> 
    {
        let manifest = std::fs::read_to_string(value.as_ref())?;
        Ok(serde_yaml::from_str(&manifest)?)
    }

}
