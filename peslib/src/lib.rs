//! The name *pes* is an acronym which stands for the Package Environment System. It is designed to address problems common to visual effects requirements, where 
//! software is typically served over NFS to users, and a means is required to configure a large number of environments independently, at both runtime 
//! and build time. 
//!
//! The *pes* crates provide 
//! - A means of defining a semantically versioned package, including target specific package dependency ranges, and target specific environments
//! - A means of solving a dependency closure given a set of package requirements
//! - A means of defining a set of environment mutations for a package version
//! - A means of building a custom environment based on the set of envrionment mutations defined by the versioned packages within a dependency closure
//! - A means of invoking a subshell with a specific environment either solved on the fly or via a cache
//! - A means of invoking a runtime within a specific environment, either solved on the fly or via a cache
//!
//! ## Parts
//! The *pes* crates consist of:
//! - *pes_core* - The core traits and modules necessary to implement various plugins which influence the behavior of the system
//! - *peslib* - The primary library implementing most of the business logic
//! - *pes* - The runtime / cli and associated functions
//! - *repo_finder* - The default implementation of a plugin to find package repositories
//! - *manifest_finder* - The default implementation of a plugin to find a manifest within a distribution

#[macro_use]
extern crate generator;
pub use pes_core::{PesError, PNResult, PesNomError, PNCompleteResult};

pub mod aliases;
pub mod constants;
pub mod distribution;
pub mod env;
pub mod jsys;
pub mod lock;
pub mod manifest;
pub mod parser;
//pub mod parser_atoms;
pub mod plugin_mgr;
pub mod range;
pub mod repository;
pub mod solver;
pub mod traits;
pub mod utils;
pub mod distribution_range;

pub use pes_core::parser_atoms;

pub use aliases::*;
pub use distribution::Distribution;
pub use env::BasicVarProvider;
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
        ManifestLocationProvider, PackageRepository, PesError, PluginMgr, Repository, SemanticVersion, Solver,
        VarProvider,
    };
}
