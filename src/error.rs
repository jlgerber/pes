use thiserror::Error as ThisError;
//use pubgrub::error::PubGrubError;

#[derive(Debug, ThisError)]
pub enum PesError {
    #[error("Unable to convert str to Range {0}")]
    ConvertToRangeFailure(String),

    // #[error("Pubgrub Error {0}")]
    // PubGrubError(#[from] PubGrubError),
    #[error("unknown err")]
    UnknownErr,

    #[error("Duplicate key '{0}'")]
    DuplicateKey(String),
}