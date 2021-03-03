//! This module implements the lockfile. A lockfile is a serialization of a solve
use std::collections::HashMap;
use std::path::Path;

use pubgrub::version::SemanticVersion;
use serde::{Serialize, Deserialize};
use toml;

use crate::PesError;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LockFile {
    schema: u32,
    request: String,
    // timestamp: ?
    author: String,
    lock: HashMap<String, HashMap<String, SemanticVersion>>
}

impl LockFile {
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
}

#[cfg(test)]
#[path = "./unit_tests/lock.rs"]
mod unit_tests;