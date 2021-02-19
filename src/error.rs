use thiserror::Error as ThisError;
//use pubgrub::error::PubGrubError;

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
}