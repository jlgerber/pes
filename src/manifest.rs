use std::path::Path;

use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;

use crate::error::PesError;

#[derive(Debug,  Serialize, Deserialize)]
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
        todo!()
    }

    /// Construct a PackageManifest from a readable file
    pub fn from_file<F>(value: F) -> Result<Self, PesError> 
    where
        F: AsRef<Path> 
    {
        todo!()
    }

}


#[derive(Debug,  Serialize, Deserialize)]
pub struct PackageRange {
    pub package: String,
    pub range: Range<SemanticVersion>
}

impl PackageRange {
    /// Construct a PackageRange given a package name and version range
    pub fn from_str<S1, S2>(package: S1, range: S2) -> Result<Self, PesError>
    where
        S1: Into<String>,
        S2: AsRef<str>
    {
        todo!()
    }
}

#[derive(Debug,  Serialize, Deserialize)]
pub struct PackageTarget {
    pub include: Vec<String>,
    pub requires: IndexMap<String, Range<SemanticVersion>>
}

impl PackageTarget {
    /// Construct a new, empty PackageTarget
    pub fn new() -> Self {
        Self {
            include: Vec::new(),
            requires: IndexMap::new()
        }
    }

    /// Add a new include to the vec of existing includes. If an existing
    /// target is supplied, an Error is returned
    pub fn include<I: Into<String>>(&mut self, target: I) -> Result<(), PesError> {
        let target = target.into();
        if !self.include.iter().any(|x| x == target.as_str()) {
            self.include.push(target);
        } else {
            return Err(PesError::DuplicateKey(target))
        }
        Ok(())
    }

    /// Given  a key and a value insert the value into the requires map 
    /// value. If the key already exists in the map, return the old value
    /// wrapped in an Option. Otherwise return None.
    /// It should be noted that the requires instance var retains insertion order,
    /// as does the method. If the key supplied to ```requires``` is already extant, 
    /// the value in the map is updated, and the original insertion order
    /// is preserved.
    pub fn requires<K>(&mut self, key: K, value: Range<SemanticVersion>) -> Option<Range<SemanticVersion>>
    where
        K: Into<String>
    {
        self.requires.insert(key.into(), value)
    }
}


#[cfg(test)]
#[path = "./unit_tests/manifest.rs"]
mod unit_tests;