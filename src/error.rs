use thiserror::Error as ThisError;

use nom::error::ErrorKind;
use nom::error::ParseError;
//use nom::Err::Error;
//use nom::IResult;


/// The package error type
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

    /// Indicates an Environment Variable is missing
    #[error("Missing Environment Variable '{0}'")]
    MissingEnvVar(#[from] std::env::VarError),

    /// Problem with version specification
    #[error("Invalid Version '{0}'")]
    InvalidVersion(String),

    /// Path does not exist
    #[error("Path does not exist {0:?}")]
    MissingPath(std::path::PathBuf),
    
    /// Indicates an include specified in the manifest does not 
    /// map to a target
    #[error("Missing Include '{include:?}' for target '{target:?}' ")]
    MissingInclude{
        /// The target recipe
        target: String, 
        /// The include name
        include: String},

    /// Indicates that an io::Error has taken place
    #[error("io::Error {0:?}")]
    IoError(#[from] std::io::Error),

    /// Wraps an opaque error type
    #[error("PesError {0}")]
    PesError(String),

    #[error("No solution for request {0}")]
    NoSolution(String),
}



/// Custom Error wrapper for Nom
#[derive(Debug, PartialEq)]
pub enum PesNomError<I> {
  InvalidKey(String),
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
//From<nom::Err<PesNomError<&str>>>` is not implemented for `PesError

//From<nom::Err<PesNomError<&str>>>` is not implemented for `PesNomError<&str>

// impl From<PesNomError<_>> for nom::Err<PesNomError<&str>> {
//     fn from(err: PesNomError<_>) ->
// }

pub type PNResult<I, T> = nom::IResult<I, T,PesNomError<I>>;
// complete result type
pub type PNCompleteResult<I, T> = Result<T, nom::Err<PesNomError<I>>>;