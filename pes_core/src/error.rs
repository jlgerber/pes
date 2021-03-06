//! Custom error types and type aliases for Pes
use std::path::PathBuf;
use thiserror::Error as ThisError;
use toml;
use serde_yaml;
use libloading;
use nom::error::{ErrorKind, ParseError};
use nom;

/// The pes crate error type - a standard enum error which wraps other error types as well as providing custom pes specific variants.
#[derive(Debug, ThisError)]
pub enum PesError {
    /// Failure to convert from a str to a Range<SemanticVersion>
    #[error("Unable to convert str to Range {0}")]
    ConvertToRangeFailure(String),

    /// Failure to deserialize Yaml via serde_yaml
    #[error("serde_yaml deserialization error {0:?}")]
    SerdeYamlDeserializeError(#[from] serde_yaml::Error),

    /// Duplicate key exists in a map type
    #[error("Duplicate key '{0}'")]
    DuplicateKey(String),

    /// General failure to parse
    #[error("Parsing Failure {0}")]
    ParsingFailure(String),

    /// Indicates a Map type is missing the provided key
    #[error("Missing key '{0}'")]
    MissingKey(String),

    // Indicates a Map type is missing the provided target
    #[error("Missing target '{0}'")]
    MissingTarget(String),

    /// Indicates an Environment Variable is missing
    #[error("Missing Environment Variable '{0}'")]
    MissingEnvVar(#[from] std::env::VarError),

    /// Problem with version specification
    #[error("Invalid Version '{0}'")]
    InvalidVersion(String),

    /// Path does not exist
    #[error("Path does not exist {0:?}")]
    MissingPath(std::path::PathBuf),

    /// Manifests do not exist for distributions
    #[error("Manifests missing for {0:?}")]
    MissingManifests(Vec<String>),

    /// manifest notfound
    #[error("Manifest Not Found starting here: '{0:?}'")]
    ManifestNotFound(PathBuf),

    /// Error converting from OsStr to Str
    #[error("unable to convert {0:?} to string")]
    ConversionError(std::ffi::OsString),

    /// Distribution not found
    #[error("Distribution not found: {0}")]
    DistributionNotFound(String),

    /// The path associated with a distribution was not found
    #[error("Distribution path not found for distribution: {0}")]
    DistributionPathNotFound(String),

    /// Indicates an include specified in the manifest does not
    /// map to a target
    #[error("Missing Include '{include:?}' for target '{target:?}' ")]
    MissingInclude {
        /// The target recipe
        target: String,
        /// The include name
        include: String,
    },
    /// Problem parsing path
    #[error("Invalid Path: {0:?}")]
    InvalidPath(PathBuf),

    /// Indicates that an io::Error has taken place
    #[error("io::Error {0:?}")]
    IoError(#[from] std::io::Error),
    
    /// Error parsing cli args
    #[error("CliArgError - error parsing cli arguments: {0}")]
    CliArgError(String),

    /// Wraps an opaque error type
    #[error("PesError {0}")]
    PesError(String),

    #[error("No solution for request {0}")]
    NoSolution(String),

    #[error("No Repositories Found at Path(2): {0}")]
    NoRepositories(String),

    #[error("PesNomError {0}")]
    PesNomError(String),

    #[error("libloading error {0:?}")]
    LibLoadingError(#[from] libloading::Error),

    #[error("Toml::de::Error {0:#?}")]
    TomlDeserializeError(#[from] toml::de::Error),

    #[error("Toml::ser::Error {0:#?}")]
    TomlSerializeError(#[from] toml::ser::Error),

    #[error("Unknown ReleaseType {0}")]
    UnknownReleaseType(String),

    #[error("invalid semantic Version {0}")]
    InvalidSemanticVersion(String),
    #[error("cannot parse '{version_part}' in '{full_version}' as u32: {parse_error}")]
    ParseIntError {
        /// [SemanticVersion] that was being parsed.
        full_version: String,
        /// A version part where parsing failed.
        version_part: String,
        /// A specific error resulted from parsing a part of the version as [u32].
        parse_error: String,
    }
    
}

/// Custom Nom Error for the `pes` crate, implementing the required `nom::error::ParseError` trait.
#[derive(Debug, PartialEq)]
pub enum PesNomError<I> {
    InvalidKey(String),
    PesError(String),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for PesNomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        PesNomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> From<(&'a str, ErrorKind)> for PesNomError<&'a str> {
    fn from((i, ek): (&'a str, ErrorKind)) -> Self {
        PesNomError::Nom(i, ek)
    }
}

impl<'a> From<PesNomError<&'a str>> for nom::Err<PesNomError<&'a str>> {
    fn from(err: PesNomError<&'a str>) -> Self {
        nom::Err::Error(err)
    }
}

impl<'a> From<nom::Err<PesNomError<&'a str>>> for PesError {
    fn from(err: nom::Err<PesNomError<&'a str>>) -> Self {
        PesError::PesNomError(err.to_string())
    }
}


impl<'a> From<PesError> for nom::Err<PesNomError<&'a str>> {
    fn from(err: PesError) -> Self {
        nom::Err::Error(PesNomError::PesError(err.to_string()))
    }
}
//From<nom::Err<PesNomError<&str>>>` is not implemented for `PesError

//From<nom::Err<PesNomError<&str>>>` is not implemented for `PesNomError<&str>

// impl From<PesNomError<_>> for nom::Err<PesNomError<&str>> {
//     fn from(err: PesNomError<_>) ->
// }

/// Type alias for a Pes Nom Result - that is a result for a non-consuming custom nom parser
pub type PNResult<I, T> = nom::IResult<I, T, PesNomError<I>>;

/// Type alias for a Pes Nom Complete Result - that is a result for a consuming style nom parser. The big difference is that the type aliases `Result` instead of `nom::IResult`.
pub type PNCompleteResult<I, T> = Result<T, nom::Err<PesNomError<I>>>;
