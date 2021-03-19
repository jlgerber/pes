//! The release type enumerates the types of releases supported by the system
//! and their relative precedence. While semver2.0 spec seemingly allows aribtrary 
//! prerelease designators, we define a number of specific variants of a ReleaseType enum:
//! - alpha
//! - beta
//! - release candidate (rc)
//! - release
//! These variants are listed in sort order. 
//! 
//! We also provide conversions from and to strings
use std::{
    fmt::{self, Debug, Display},
    str::FromStr
};
use crate::error::PesError;


/// The `ReleaseType`, as the name suggests, defines the type of release,
/// which is either a release, or some form of pre-release. This is more restrictive
/// than the full 2.0 semanticversion spec, but in practice, covers prelease names,
/// and has the further advanctage of specifying an explicit order.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ReleaseType {
    Alpha,
    Beta,
    ReleaseCandidate,
    Release,
}

impl FromStr for ReleaseType {
    type Err = PesError;

    fn from_str(input: &str) -> Result<Self, PesError> {
        match input.to_lowercase().as_str() {
            "" | "release" => Ok(Self::Release),
            "rc" | "releasecandidate" => Ok(Self::ReleaseCandidate),
            "beta" => Ok(Self::Beta),
            "alpha" => Ok(Self::Alpha),
            _ => Err(PesError::UnknownReleaseType(input.to_string()))
        }
    }
}

impl Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseType::Release =>     write!(f, ""),
            ReleaseType::ReleaseCandidate => write!(f, "rc"),
            ReleaseType::Beta => write!(f, "beta"),
            ReleaseType::Alpha => write!(f, "alpha")
        }
    }
}

