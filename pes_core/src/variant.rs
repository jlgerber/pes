//! a variant is a wrapper around a Version which introduces an additional name
//! a variant 
use std::{
    fmt::{self, Debug, Display},
    str::FromStr
};
use crate::{error::PesError, ReleaseType};
use pubgrub::version::Version;

// //
// pub enum Variant<T: Version> {
//     Anon(Version),
//     Named{name: String, version:Version}
// }

#[derive(Debug, Hash)]
pub enum VariantType {
    Any,
    Default,
    Named(String)
}

#[derive(Debug, Hash)]
struct Variant<T> {
    version: T, 
    type: VariantType
}


impl FromStr for SemanticVersion {
    type Err = PesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}

impl<T: Version> serde::Serialize for Version<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}


impl<'de, T: Version> serde::Deserialize<'de> for Version<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Display for VariantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => write!(f, "Any"),
            Self::Default => write(f, "Default"),
            Self::Named(name) => write!(f, "{}", name), 
        }
    }
}

impl<T: Version> Display for Variant<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = self.version.fmt(f);
        match self.type {
            VariantType::Any | VariantType::Default => write!(f, "{}", version),
            VariantType::Named(name) => write!("{}-{}", name, version);
        }
    }
}

impl<T: Version> Version for Variant<T> {
    // Implement Version for SemanticVersion.
    fn lowest() -> Self {
        Self{ version: version::lowest(),
            type: Default
    }

    fn bump(&self) -> Self {
        self.version.bump_release_type()
    }

}