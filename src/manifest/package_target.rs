//use std::path::Path;

use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;
use crate::error::PesError;
use crate::parser::parse_consuming_semver_range;
use crate::VersionedPackage;


/// Struct used to simplify serialization & deserialization of manifest
#[derive(Debug,  Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageTarget {
    pub include: Option<Vec<String>>,
    // Range<SemanticVersion> (Todo: newtype wrapper)
    pub requires: IndexMap<String, String>
}

impl PackageTarget {
    /// Construct a new, empty PackageTarget
    pub fn new() -> Self {
        Self {
            include: None,
            requires: IndexMap::new()
        }
    }

    /// Add a new include to the vec of existing includes. If an existing
    /// target is supplied, an Error is returned
    pub fn include<I: Into<String>>(&mut self, target: I) -> Result<(), PesError> {
        let target = target.into();
        if let Some(ref mut include) = &mut self.include {
            if !include.iter().any(|x| x == target.as_str()) {
                include.push(target);
            } else {
                return Err(PesError::DuplicateKey(target))
            }
        } else {
            self.include = Some(vec![target])
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
    //pub fn requires<K>(&mut self, key: K, value: Range<SemanticVersion>) -> Option<Range<SemanticVersion>>
    pub fn requires<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>
    {
        self.requires.insert(key.into(), value.into())
    }

    /// Retrieve the SemanticVersion Range associated with the provided key
    pub fn get_requires(&self, key: &str) -> Result<Range<SemanticVersion>, PesError> {
        let result = self.requires.get(key);
        if let Some(result) = result {
            
            Ok(parse_consuming_semver_range(result)?)
        } else {
            Err(PesError::MissingKey(key.into()))
        }
    }

    /// Retrieve all the requires
    pub fn get_all_requires(&self) -> Result<Vec<VersionedPackage>, PesError> {
        let mut retval = Vec::with_capacity(self.requires.len());
        for (k,ref v) in self.requires.iter() {
            retval.push(VersionedPackage::from_strs(k.as_str(),v)?);
        }

        Ok(retval)
    }

    /// Retrieve a vector of included targets
    pub fn get_includes(&self) -> Vec<&str> {
        if let Some(ref includes) = self.include {
            includes.iter().map(|v| v.as_str()).collect()
        } else {
            Vec::new()
        }
    }

    /// Validate that all of the requires are valid semver ranges
    pub fn validate_requires(&self) -> Result<(), PesError> {
        for v in self.requires.values() {
            let _ = parse_consuming_semver_range(v)?;
        }
        Ok(())
    }
}
