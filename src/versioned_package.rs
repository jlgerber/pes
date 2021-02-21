//! Defines a VersionedPackage struct use to represent a 
//! versioned package (surprise)

use pubgrub::{
    range::Range,
    version::SemanticVersion
};

use crate::error::PesError;
use crate::parser::parse_consuming_semver_range;
use crate::parser::parse_consuming_package_range;

/// Simple representation of a versioned package
#[derive(PartialEq, Eq,  Debug)]
pub struct VersionedPackage<'a> {
    /// Name of the package
    pub name: &'a str,
    /// Version Range for the package
    pub range: Range<SemanticVersion>
}

impl<'a> VersionedPackage<'a> {
    /// Construct a versioned package from a name and range
    pub fn new(name: &'a str, range: Range<SemanticVersion>) -> Self {
        Self {
            name, range
        }
    }

    /// Construct a VersionedPackage from strs
    pub fn from_strs<'b: 'a>(name: &'b str, range: &str) -> Result<Self, PesError> {
        let range = parse_consuming_semver_range(range)?;
        Ok(
            Self {
                name, 
                range
            }
        )
    }

    // Construct VersionedPackage from str (eg maya-1.2.3+<4)
    pub fn from_str<'b: 'a>(name: &'b str) -> Result<Self, PesError> {
        let (name, range)  = parse_consuming_package_range(name)?;

        Ok(
            Self {
                name,
                range
            }
        )
    }
}


/// Simple representation of a versioned package
#[derive(PartialEq, Eq,  Debug)]
pub struct VersionedPackageOwning {
    /// Name of the package
    pub name: String,
    /// Version Range for the package
    pub range: Range<SemanticVersion>
}

impl VersionedPackageOwning {
    /// Construct a versioned package from a name and range
    pub fn new<N: Into<String>>(name: N, range: Range<SemanticVersion>) -> Self {
        Self {
            name: name.into(), range
        }
    }

    /// Construct a VersionedPackage from strs
    pub fn from_strs<N: Into<String>>(name: N, range: &str) -> Result<Self, PesError> {
        let range = parse_consuming_semver_range(range)?;
        Ok(
            Self {
                name: name.into(), 
                range
            }
        )
    }

    pub fn to_tuple(self) -> (String, Range<SemanticVersion>) {
        let Self{name, range} = self;
        (name, range)
    }
}