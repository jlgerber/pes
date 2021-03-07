use std::path::PathBuf;

use crate::manifest::package_target::PackageTarget;
/// custom type mapping a target name to a PackageTarget
pub type TargetMap = indexmap::IndexMap<String, PackageTarget>;

pub type EnvMap = indexmap::IndexMap<String, String>;

pub type DistMap = indexmap::IndexMap<String, PathBuf>;

pub use pubgrub::type_aliases::SelectedDependencies;