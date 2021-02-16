//! pes - The Package Environment System 
//!
//! Provides a means to define a set of versioned packages with package dependencies
//! and calculate a dependency closure for one or more requirements. 
//! Furthermore, it provides a means to initialize an environment customized to the set of dependencies
//! and invoke an executable in this known environment.

/// Define a trait that will allow us to implement a conversion from a string
/// to a Pubgrub::Range. We cannot use FromStr as defined in the standard library, as 
/// this will violate the Orphan rule
pub mod error;
pub mod parser;

use error::PesError;
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;

pub trait FrmStr {
    type FrmStrErr;

    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized;
}

impl FrmStr for Range<SemanticVersion> {
    type FrmStrErr = error::PesError;

    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized {
        Err(PesError::UnknownErr)
        
    }
}