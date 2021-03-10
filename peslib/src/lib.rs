//! pes - The Package Environment System 
//! 
//! Provides 
//! - A means of defining a semantically versioned package, including target specific package dependency ranges, and target specific environments
//! - A means of solving a dependency closure given a set of package requirements
//! - A means of defining a set of environment mutations for a package version
//! - A means of building a custom environment based on the set of envrionment mutations defined by the versioned packages within a dependency closure
//! - A means of invoking a subshell with a specific environment either solved on the fly or via a cache
//! - A means of invoking a runtime within a specific environment, either solved on the fly or via a cache
#[macro_use]
extern crate generator;

// #[macro_use]
// extern crate lazy_static;

pub mod env;
pub mod error;
pub mod manifest;
pub mod parser;
pub mod parser_atoms;
pub mod range;
pub mod repository;
pub mod solver;
pub mod traits;
pub mod utils;
pub mod versioned_package;
pub mod constants;
pub mod jsys;
pub mod aliases;
pub mod lock;

pub use error::PesError;
pub use range::*;
pub use solver::Solver;
pub use traits::{VarProvider, BaseEnv, Repository, ManifestLocator};
pub use versioned_package::VersionedPackage;
pub use repository::PackageRepository;
pub use pubgrub::version::SemanticVersion;
pub use manifest::Manifest;
pub use env::BasicVarProvider;
pub use aliases::*;
pub use solver::SelectedDependencies;
pub use lock::LockFile;

pub mod prelude {
    pub use super::{
        BasicVarProvider,
        PesError,
        Solver, 
        VarProvider, 
        BaseEnv,
        VersionedPackage, 
        Repository,
        SemanticVersion,
        PackageRepository,
        ManifestLocator,
        Manifest,
        aliases::*,
        LockFile,
    };
}
