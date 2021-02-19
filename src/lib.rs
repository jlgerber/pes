//! pes - The Package Environment System 
//!
//! Provides a means to define a set of versioned packages with package dependencies
//! and calculate a dependency closure for one or more requirements. 
//! Furthermore, it provides a means to initialize an environment customized to the set of dependencies
//! and invoke an executable in this known environment.
#[macro_use]
extern crate generator;

/// Defines a custom error - PesError for the crate
pub mod error;
pub mod parser;
pub mod parser_atoms;
pub mod manifest;
pub mod versioned_package;
pub mod repository;

use versioned_package::VersionedPackage;
use error::PesError;
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;

pub trait FrmStr {
    type FrmStrErr;

    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized;
}


impl FrmStr for Range<SemanticVersion> {
    type FrmStrErr = PesError;

    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized {
        let _ = value;
        todo!()
        
    }
}