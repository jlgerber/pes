//! This module implements the lockfile. A lockfile is a serialization of a solve
use std::{
    collections::{
        HashMap,
        hash_map::{Iter, Keys}
    },
    io::{Read, Write},
    path::Path,
};

use pubgrub::{version::SemanticVersion};
use serde::{Serialize, Deserialize};
use toml;

use crate::{
    PesError,
    parser::parse_consuming_package_version,
    SelectedDependencies,
};

pub type VersionMap = HashMap<String, SemanticVersion>;
pub type LockMap = HashMap<String, VersionMap>;


/// The lockfile stores resolved dependency closures for targets
#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LockFile {
    schema: u32,
    request: String,
    // timestamp: ?
    author: String,
    lock: LockMap
}

impl LockFile {

    /// Construct a new, empty LockFile
    pub fn new<R: Into<String>, A: Into<String>>(request:R, author: A) -> Self {
        Self {
            schema: 1,
            request: request.into(),
            author: author.into(),
            lock: HashMap::new()
        }
    }

    /// Construct a LockFile from a str
    pub fn from_str(input: &str) -> Result<Self, PesError> {
        let lf: LockFile = toml::from_str(input)?;
        Ok(lf)
    }

    /// Read a lockfile from a path
    pub fn from_file<I: AsRef<Path>>(input: I) -> Result<Self, PesError> {
        let file = std::fs::read_to_string(input)?;
        Self::from_str(&file)
    }

    /// Read a lockfile from a type that implements Read
    pub fn from_reader<R: Read>(&self, input: &mut R) -> Result<Self, PesError> {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer)?;
        Self::from_str(buffer.as_str())
    }

    /// write lockfile to file
    pub fn to_file<I: AsRef<Path>>(&self, output: I, pretty: bool) -> Result<(), PesError> {
        let mut file = std::fs::File::create(output.as_ref())?;
        self.to_writer(&mut file, pretty)
    }

    /// Write lockfile given an implementer of the Write trait 
    pub fn to_writer<W: Write>(&self, output: &mut W, pretty: bool) -> Result<(), PesError> {
        let tomlstr = if pretty { toml::to_string_pretty(&self)? } else { toml::to_string(&self)? };
        output.write_all(tomlstr.as_bytes())?;
        Ok(())
    }


    /// Insert a new distribution for the target
    pub fn add_dist(&mut self, target: &str, dist: &str) -> Result<(), PesError> {
        let (name, version) = parse_consuming_package_version(dist)?;
        match self.lock.get_mut(target) {
            Some(map) => {
                map.insert(name.to_string(), version);
            },
            None => {
                let mut map = HashMap::new();
                map.insert(name.to_string(), version);
                self.lock.insert(target.to_string(), map);
            }
        }
        Ok(())
    }


    /// Does the Lockfile contain a target?
    pub fn has_target(&self, target: &str) -> bool {
        self.lock.contains_key(target)
    } 

    /// Retrieve the version of the package stored in the target, should it exist
    pub fn version(&self, target: &str, package: &str) -> Option<&SemanticVersion> {
        if let Some(map) = self.lock.get(target) {
            map.get(package) 
                
        } else {
            None
        }
    }

    /// Retrieve an iterator over the targets in the LockFile
    pub fn targets(&self) -> Keys<'_, String, VersionMap> {
        self.lock.keys()
    }

    /// Retrieve an Option wrapped iterator over items in target
    pub fn dists_for(&self, target: &str) -> Option<Iter<'_, String, SemanticVersion>> {
        match self.lock.get(target) {
            Some(map) => Some(map.iter()),
            None => None
        }
    }

    pub fn selected_dependencies_for(&self, target: &str) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        let mut selected_deps: SelectedDependencies<String, SemanticVersion> = SelectedDependencies::default();
        match self.dists_for(target) {
            Some(iter) => {
                iter.for_each(|(k,v)| {selected_deps.insert(k.into(), v.clone()); ()});
            },
            None => return Err(PesError::MissingTarget(target.to_string()))
        }
        Ok(selected_deps)
    }
    /// convenience function to convert the tuple returned by dist_for to a string
    pub fn dist_tuple_to_string(input: (&String, &SemanticVersion)) -> String {
        format!("{}-{}", input.0, input.1)
    }
}



#[cfg(test)]
#[path = "./unit_tests/lock.rs"]
mod unit_tests;