use std::path::PathBuf;

use crate::manifest::package_target::PackageTarget;
use crate::SemanticVersion;

/// custom type mapping a target name to a PackageTarget
pub type TargetMap = indexmap::IndexMap<String, PackageTarget>;

pub type EnvMap = indexmap::IndexMap<String, String>;

pub type DistMap = indexmap::IndexMap<String, PathBuf>;

pub use pubgrub::type_aliases::SelectedDependencies;

/// A Map whose key is a distribution and whose value is a path to the distribution
pub type DistPathMap = indexmap::IndexMap<String, String>;

// a map of package to distribution
pub type PackageDistMap = indexmap::IndexMap<String, String>;

/// 
pub type SolveDistributions = SelectedDependencies<String, SemanticVersion>;

/// Tuple returned by perform_solve function
pub type SolveResult = (DistPathMap, SolveDistributions);
pub type SolveRefResult<'a> = (&'a DistPathMap, &'a SolveDistributions);
