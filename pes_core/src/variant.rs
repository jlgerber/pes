//! a variant is a wrapper around a Version which introduces an additional name
//! a variant 
use std::{
    fmt::{self, Debug, Display},
    str::FromStr
};
use crate::{error::PesError, ReleaseType, SemanticVersion};
use pubgrub::version::Version;

// //
// pub enum Variant<T: Version> {
//     Anon(Version),
//     Named{name: String, version:Version}
// }


#[derive(Debug, Hash, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Variant<T> {
    version: T, 
    index: u8
}


impl<V: Version> FromStr for Variant<V> {
    type Err = PesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl<T: Version> serde::Serialize for Variant<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}


impl<'de, T: Version> serde::Deserialize<'de> for Variant<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<T: Version> Display for Variant<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _version = Display::fmt(&self.version, f)?;
        write!(f, "@{}", self.index)?;
        Ok(())
        
    }
}

impl Version for Variant<SemanticVersion> {
    // Implement Version for SemanticVersion.
    fn lowest() -> Self {
        Self{ version: Version::lowest(),
            index: 0
        }
    }

    fn bump(&self) -> Self {
        let version = self.version.bump();
        Self::new(version, self.index)
    }

}

impl Default for Variant<SemanticVersion> {
    fn default() -> Self {
        Self {
            version: SemanticVersion::zero(), index: 0
        }
    }
}

impl Variant<SemanticVersion> {
     /// Create a version with "major", "minor", "patch" and "release_type" values.
    /// `version = major.minor.patch-releasetype`
    pub fn new(semver: SemanticVersion, index: u8) -> Self {
        Self {
            version: semver,
            index
        }
    }
    pub fn major(&self) -> u32 {
        self.version.major
    }
    pub fn minor(&self) -> u32 {
        self.version.minor
    }
    pub fn patch(&self) -> u32 {
        self.version.patch
    }
    pub fn index(&self) -> u8 {
        self.index
    }
    pub fn release_type(&self) -> ReleaseType {
        self.version.release_type.clone()
    }
    /// Version 0.0.0.
    pub fn zero() -> Self {
        // technically one could argue that this should be an alpha
        // pre-release, but...
        Self{ version: SemanticVersion::new(0, 0, 0, ReleaseType::Release), index: 0}
    }

    /// Version 1.0.0.
    pub fn one() -> Self {
        Self{ version: SemanticVersion::new(1, 0, 0, ReleaseType::Release), index: 0 }
    }

    /// Version 2.0.0.
    pub fn two() -> Self {
        Self{ version: SemanticVersion::new(2, 0, 0, ReleaseType::Release), index: 0 }
    }

    /// Bump up the ReleaseType. If the current ReleaseType is Release, then bump the patch
    pub fn bump_release_type(self) -> Self {
       let version = self.version.bump_release_type();
       Self::new(version, self.index)
    }
    /// Bump the patch number of a version.
    pub fn bump_patch(self) -> Self {
        let Variant{version, index} = self;
        Self{version: version.bump_patch(), index}
    }

    /// Bump the minor number of a version.
    pub fn bump_minor(self) -> Self {
        let Variant{version, index} = self;
        Self{version: version.bump_minor(), index}
       
    }

    /// Bump the major number of a version.
    pub fn bump_major(self) -> Self {
        let Variant{version, index} = self;
        Self{version: version.bump_major(), index}
    }
    ///
    pub fn release(self) -> Self {
        let Variant{version, index} = self;
        Self{version: version.release(), index}
    }
    /// increment the variant index
    pub fn bump_index(self) -> Self {
        let Variant{version, index} = self;
        Self{version, index: index + 1}
    }
}