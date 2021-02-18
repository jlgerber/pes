use std::path::Path;

use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use pubgrub::version::SemanticVersion;
//use pubgrub::range::Range;

use crate::error::PesError;
use crate::manifest::PackageTarget;
use crate::VersionedPackage;


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

    /// Retrieve a vector of SemanticVersion Ranges associated with the provided target
    pub fn get_requires(&self, target: &str) -> Result<Vec<VersionedPackage>, PesError> {
        let rtarget = self.targets.get(target);
        let mut results = Vec::new();
        if let Some(target) = rtarget {
            // incorporate any included targets package ranges
            for include in target.get_includes() {
                let inc_target = self.targets.get(include);
                if let Some(inc_target) = inc_target {
                    results.append(&mut inc_target.get_all_requires()?);
                }
            }
            results.append(&mut target.get_all_requires()?);
            Ok(results)
        } else {
            Err(PesError::MissingKey(target.into()))
        }
    }

}


#[cfg(test)]
#[path = "../unit_tests/package_manifest.rs"]
mod unit_tests;