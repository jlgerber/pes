//! The pes_core crate provides the core traits and modules necessary to implmeent 
//! plugins for peslib.
//! 
//! ## Plugins
//! There are currently two types of plugins:
//! - A *repo_finder*, which is responsible for locating package repositories
//! - A *manifest_finder*, which is responsible for finding the manifest within a distribution. 
//!
//! Both of these tasks are highly dependent upon a particular organization's designs, and the 
//! plugin system affords the most flexibility in terms of adapting the system to a given 
//! organization's needs without having to fork the core.
//!
//! ## Other modules 
//! In addition to the traits provided by the crate, *pes_core* also provides the error types
//! used throughout. While it should be noted that *peslib* re-exports all of the error types,
//! one should prefer using the error types from the *pes_core* crate when authoring a plugin.

pub mod traits;
pub mod error;
pub mod semantic_version;
pub mod parser;
pub mod parser_atoms;
pub mod release_type;
pub mod env;
pub mod variant;

pub use error::{PesError, PNResult, PesNomError, PNCompleteResult};
pub use traits::*;
pub use semantic_version::SemanticVersion;
pub use release_type::ReleaseType;
pub use variant::Variant;

pub mod prelude {
    pub use super::*;
}