pub mod package_manifest;
pub mod package_range;
pub mod package_target;

pub use package_manifest::PackageManifest;
pub use package_range::PackageRange;
pub use package_target::PackageTarget;
pub type TargetMap = indexmap::IndexMap<String, PackageTarget>;


#[cfg(test)]
#[path = "./unit_tests/manifest.rs"]
mod unit_tests;