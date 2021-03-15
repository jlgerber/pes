//! UNUSED

//use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;

use crate::PesError;
//use crate::manifest::PackageTarget;


#[derive(Debug,  Serialize, Deserialize, PartialEq, Eq)]
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
        let _ = package; let _ = range;
        todo!()
    }
}