use indexmap;
use serde::{Serialize, Deserialize};
use crate::error::PesError;

#[derive(Debug,  Serialize, Deserialize)]
pub struct PackageManifest {
    pub schema: u32,
    pub name: String,
    pub version: SemanticVersion,
    pub description: String,
    pub targets: HashMap<String, PackageTarget>
}

#[derive(Debug,  Serialize, Deserialize)]
pub struct PackageRange {
    pub package: String,
    pub range: Range<SemanticVersion>
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
            requires: IndexMap<String, Range<SemanticVersion>>::new()
        }
    }

    /// Add a new include to the vec of existing includes. If an existing
    /// target is supplied, an Error is returned
    pub fn include<I: Into<String>>(&mut self, target: I) -> Result<(), PesError> {
        let target = target.into();
        if !self.include.iter().any(|&x| x == target.as_ref()) {
            self.include.entry(target);
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
    pub fn requires<K, V>(&mut self, key: K, value: V) -> Option<V>
    where
        K: Into<String>,
        V: Range<SemanticVersion>
    {
        self.insert(key.into(), value)
    }
}