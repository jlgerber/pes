
//! Traits and implementations to create and compare versions.

use std::fmt::{self, Debug, Display};
use std::str::FromStr;
use crate::error::PesError;
//use thiserror::Error;
use pubgrub::version::Version;

// /// Versions have a minimal version (a "0" version)
// /// and are ordered such that every version has a next one.
// pub trait Version: Clone + Ord + Debug + Display {
//     /// Returns the lowest version.
//     fn lowest() -> Self;
//     /// Returns the next version, the smallest strictly higher version.
//     fn bump(&self) -> Self;
// }

// /// Error creating [SemanticVersion] from [String].
// #[derive(Error, Debug, PartialEq)]
// pub enum VersionParseError {
//     /// [SemanticVersion] must contain major, minor, patch versions.
//     #[error("version {full_version} must contain 3 numbers separated by dot")]
//     NotThreeParts {
//         /// [SemanticVersion] that was being parsed.
//         full_version: String,
//     },
//     /// Wrapper around [ParseIntError](core::num::ParseIntError).
//     #[error("cannot parse '{version_part}' in '{full_version}' as u32: {parse_error}")]
//     ParseIntError {
//         /// [SemanticVersion] that was being parsed.
//         full_version: String,
//         /// A version part where parsing failed.
//         version_part: String,
//         /// A specific error resulted from parsing a part of the version as [u32].
//         parse_error: String,
//     },
// }

/// The `ReleaseType`, as the name suggestgs, defines the type of release,
/// which is generally either a release, or some form of pre-release.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ReleaseType {
    Release,
    ReleaseCandidate,
    Beta,
    Alpha
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

/// Type for semantic versions: major.minor.patch.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SemanticVersion {
    major: u32,
    minor: u32,
    patch: u32,
    release_type: ReleaseType
}


impl serde::Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}


impl<'de> serde::Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

// Constructors
impl SemanticVersion {
    /// Create a version with "major", "minor", "patch" and "release_type" values.
    /// `version = major.minor.patch-releasetype`
    pub fn new(major: u32, minor: u32, patch: u32, release_type: ReleaseType) -> Self {
        Self {
            major,
            minor,
            patch,
            release_type
        }
    }
    
    /// Version 0.0.0.
    pub fn zero() -> Self {
        Self::new(0, 0, 0, ReleaseType::Release)
    }

    /// Version 1.0.0.
    pub fn one() -> Self {
        Self::new(1, 0, 0, ReleaseType::Release)
    }

    /// Version 2.0.0.
    pub fn two() -> Self {
        Self::new(2, 0, 0, ReleaseType::Release)
    }
}

// Convert a tuple (major, minor, patch) into a version.
impl From<(u32, u32, u32)> for SemanticVersion {
    fn from(tuple: (u32, u32, u32)) -> Self {
        let (major, minor, patch) = tuple;
        Self::new(major, minor, patch, ReleaseType::Release)
    }
}

impl From<(u32, u32, u32, ReleaseType)> for SemanticVersion {
    fn from(tuple: (u32, u32, u32, ReleaseType)) -> Self {
        let (major, minor, patch, rt) = tuple;
        Self::new(major, minor, patch, rt)
    }
}


impl From<(u32, u32, u32, &str)> for SemanticVersion {
    fn from(tuple: (u32, u32, u32, &str)) -> Self {
        let (major, minor, patch, rt) = tuple;
        Self::new(major, minor, patch, ReleaseType::from_str(rt).unwrap_or_else(|_| ReleaseType::Release))
    }
}

// Convert a version into a tuple (major, minor, patch).
impl Into<(u32, u32, u32, ReleaseType)> for SemanticVersion {
    fn into(self) -> (u32, u32, u32, ReleaseType) {
        (self.major, self.minor, self.patch, self.release_type)
    }
}

// Bump versions.
impl SemanticVersion {
    /// Bump the patch number of a version.
    pub fn bump_patch(self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1, self.release_type)
    }

    /// Bump the minor number of a version.
    pub fn bump_minor(self) -> Self {
        Self::new(self.major, self.minor + 1, 0, self.release_type)
    }

    /// Bump the major number of a version.
    pub fn bump_major(self) -> Self {
        Self::new(self.major + 1, 0, 0, self.release_type)
    }
    ///
    pub fn release(self) -> Self {
        Self::new(self.major, self.minor, self.patch, ReleaseType::Release)
    }
}


impl FromStr for SemanticVersion {
    type Err = PesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_u32 = |part: &str| {
            part.parse::<u32>().map_err(|e| Self::Err::ParseIntError {
                full_version: s.to_string(),
                version_part: part.to_string(),
                parse_error: e.to_string(),
            })
        };

        let pieces = s.split('-').collect::<Vec<_>>();
        match &pieces[..] {
            [version, release_type] => {
                let mut parts = version.split('.');
                match (parts.next(), parts.next(), parts.next()) {
                    (Some(major), Some(minor), Some(patch)) => {
                        let major = parse_u32(major)?;
                        let minor = parse_u32(minor)?;
                        let patch = parse_u32(patch)?;
                        let release_type = ReleaseType::from_str(release_type)?;
                        Ok(Self {
                            major,
                            minor,
                            patch,
                            release_type
                        })
                    },
                    _ => Err(Self::Err::InvalidSemanticVersion(s.to_string())),
                }
            }
            [version] => {
                let mut parts = version.split('.');
                match (parts.next(), parts.next(), parts.next()) {
                    (Some(major), Some(minor), Some(patch)) => {
                        let major = parse_u32(major)?;
                        let minor = parse_u32(minor)?;
                        let patch = parse_u32(patch)?;
            
                        Ok(Self::new(
                            major,
                            minor,
                            patch,
                            ReleaseType::Release
                        ))
                    },
                    _ => Err(Self::Err::InvalidSemanticVersion(s.to_string())),
                }
            },
            _ => Err(Self::Err::InvalidSemanticVersion(s.to_string()))
        }
        
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.release_type {
            ReleaseType::Release =>     write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
            _ => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, self.release_type),
        }
    }
}

// Implement Version for SemanticVersion.
impl Version for SemanticVersion {
    fn lowest() -> Self {
        Self::zero()
    }
    fn bump(&self) -> Self {
        self.bump_patch()
    }
}