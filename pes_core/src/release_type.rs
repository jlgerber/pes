use std::{
    fmt::{self, Debug, Display},
    str::FromStr
};
use crate::error::PesError;


/// The `ReleaseType`, as the name suggestgs, defines the type of release,
/// which is generally either a release, or some form of pre-release.
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
