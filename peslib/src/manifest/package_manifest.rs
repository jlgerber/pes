//! PackageManifest struct
use crate::{EnvMap, TargetMap};
use std::path::Path;
//use indexmap::IndexMap;
//use pubgrub::version::SemanticVersion;
use serde::{Deserialize, Serialize};

use crate::{
    PesError,
    //manifest::PackageTarget,
    SemanticVersion,
    DistributionRange,
};

/// Models a manifest for package
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageManifest {
    /// schema version of the manifest
    pub schema: u32,
    /// Name of the package
    pub name: String,
    /// Version of the package
    pub version: SemanticVersion,
    /// Description of the package
    pub description: String,
    /// Map of targets for the manifest (eg build, run, lint, etc)
    #[serde(default)]
    pub targets: TargetMap, //IndexMap<String, PackageTarget>,
    /// environment vars (doesnt really have to be an index map)
    #[serde(default)]
    pub environment: EnvMap, //IndexMap<String, String>
}

impl PackageManifest {
    /// Construct a PackageManifest from a str
    pub fn from_str_unchecked(value: &str) -> Result<Self, PesError> {
        Ok(serde_yaml::from_str(value)?)
    }

    /// Construct a PackageManifest from a readable file
    pub fn from_file_unchecked<F>(value: F) -> Result<Self, PesError>
    where
        F: AsRef<Path>,
    {
        let manifest = std::fs::read_to_string(value.as_ref())?;
        Ok(Self::from_str_unchecked(&manifest)?)
    }

    /// Construct a PackageManifest from a str
    pub fn from_str(value: &str) -> Result<Self, PesError> {
        let manifest: PackageManifest = serde_yaml::from_str(value)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Construct a PackageManifest from a readable file
    pub fn from_file<F>(value: F) -> Result<Self, PesError>
    where
        F: AsRef<Path>,
    {
        let manifest = std::fs::read_to_string(value.as_ref())?;
        Ok(serde_yaml::from_str(&manifest)?)
    }

    /// Retrieve a vector of SemanticVersion Ranges associated with the provided target
    pub fn get_requires(&self, target: &str) -> Result<Vec<DistributionRange>, PesError> {
        let rtarget = self
            .targets
            .get(target)
            .ok_or_else(|| PesError::MissingKey(target.into()))?;

        let mut results = Vec::new();

        // incorporate any included targets package ranges
        for include in rtarget.get_includes() {
            let inc_target = self.targets.get(include);
            if let Some(inc_target) = inc_target {
                results.append(&mut inc_target.get_all_requires()?);
            }
        }
        results.append(&mut rtarget.get_all_requires()?);
        Ok(results)
    }

    // looks like version is already a SemanticVersion
    // /// Retrieve the version for a package
    // pub fn get_version(&self) -> Result<SemanticVersion, PesError> {
    //     parse_consuming_semver(self.name.as_str())
    // }
    /// Validate the contents of the manifest, making sure that all versions's
    /// can parse
    pub fn validate(&self) -> Result<(), PesError> {
        // not needed as self.version is already a SemanticVersion
        //let _ = parse_consuming_semver(self.version.as_str()).map_err(|| PesError::InvalidVersion(self.version.as_str()))?;
        for (key, target) in self.targets.iter() {
            for include in target.get_includes() {
                if !self.targets.contains_key(include) {
                    return Err(PesError::MissingInclude {
                        target: key.into(),
                        include: include.into(),
                    });
                }
            }
            target.validate_requires()?;
        }

        Ok(())
    }

    /// Retrieve the name of the distribution - ie <name>-<SemanticVersion>. so `foo-0.1.0`
    pub fn distribution(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

#[cfg(test)]
#[path = "../unit_tests/package_manifest.rs"]
mod unit_tests;
