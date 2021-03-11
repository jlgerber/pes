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

pub mod aliases;
pub mod constants;
pub mod distribution;
pub mod env;
pub mod error;
pub mod jsys;
pub mod lock;
pub mod manifest;
pub mod parser;
pub mod parser_atoms;
pub mod plugin_mgr;
pub mod range;
pub mod repository;
pub mod solver;
pub mod traits;
pub mod utils;
pub mod distribution_range;

pub use aliases::*;
pub use distribution::Distribution;
pub use env::BasicVarProvider;
pub use error::PesError;
pub use lock::LockFile;
pub use manifest::Manifest;
pub use plugin_mgr::PluginMgr;
pub use pubgrub::version::SemanticVersion;
pub use range::*;
pub use repository::PackageRepository;
pub use solver::SelectedDependencies;
pub use solver::Solver;
pub use traits::{BaseEnv, ManifestLocationProvider, Repository, VarProvider};
pub use distribution_range::DistributionRange;

pub mod prelude {
    pub use super::{
        aliases::*, BaseEnv, BasicVarProvider, Distribution, DistributionRange, LockFile, Manifest,
        ManifestLocationProvider, PackageRepository, PesError, Repository, SemanticVersion, Solver,
        VarProvider,
    };
}
