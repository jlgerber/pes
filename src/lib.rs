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
pub mod utils;
pub mod solver;
pub mod range;
pub mod env;
pub mod traits;

pub use range::*;
pub use solver::Solver;
pub use versioned_package::VersionedPackage;
pub use error::PesError;
pub use traits::{VarProvider};

pub mod prelude {
    pub use super::{
        VarProvider, 
        Solver, 
        VersionedPackage, 
        PesError
    };
}
// use pubgrub::range::Range;
// use pubgrub::version::SemanticVersion;

// /// Trait to provide an alternative, falible constructor from a &str
// pub trait FrmStr {
//     type FrmStrErr;

//     /// Given a str, construct an instance of Self
//     fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized;
// }


// impl FrmStr for Range<SemanticVersion> {
//     type FrmStrErr = PesError;

//     fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized {
//         parse_consuming_semver_range(value)
//     }
// }