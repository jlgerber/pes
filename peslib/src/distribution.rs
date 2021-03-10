//! Represents an individual versioned package, with a name and a semantic version

use std::fmt;

use pubgrub::{
    version::SemanticVersion,
};

use crate::{ 
    error::PesError,
   parser::{
        parse_consuming_package_version,
        parse_consuming_semver,
    },
};

/// Simple representation of a versioned package
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Distribution<'a> {
    /// Name of the package
    pub name: &'a str,
    /// Version Range for the package
    pub version: SemanticVersion
}

impl<'a> Distribution<'a> {
    /// Construct a versioned package from a name and range
    pub fn new(name: &'a str, version: SemanticVersion) -> Self {
        Self {
            name, version
        }
    }

    /// Construct a VersionedPackage from strs
    pub fn from_strs<'b: 'a>(name: &'b str, version: &str) -> Result<Self, PesError> {
        let version = parse_consuming_semver(version)?;
        Ok(
            Self {
                name, 
                version
            }
        )
    }

    /// Construct VersionedPackage from str (eg maya-1.2.3+<4)
    pub fn from_str<'b: 'a>(name: &'b str) -> Result<Self, PesError> {
        let (name, version)  = parse_consuming_package_version(name)?;

        Ok(
            Self {
                name,
                version
            }
        )
    }

    /// given two different distributions, do they share the same package name?
    pub fn package_eq(&self, other: &Distribution) -> bool {
        self.name == other.name
    }
}

impl<'a> fmt::Display for Distribution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}-{}", self.name, self.version)
    }
}


#[cfg(test)]
#[path = "./unit_tests/distribution.rs"]
mod unit_tests;