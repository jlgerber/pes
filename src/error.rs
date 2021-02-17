use thiserror::Error as ThisError;
//use pubgrub::error::PubGrubError;

#[derive(Debug, ThisError)]
pub enum PesError {
    #[error("Unable to convert str to Range {0}")]
    ConvertToRangeFailure(String),

    // #[error("Pubgrub Error {0}")]
    // PubGrubError(#[from] PubGrubError),
    
    #[error("serde_yaml deserialization error {0:?}")]
    SerdeYamlDeserializeError(#[from] serde_yaml::Error),

    #[error("Duplicate key '{0}'")]
    DuplicateKey(String),

    #[error("io::Error {0:?}")]
    IoError(#[from] std::io::Error),
}