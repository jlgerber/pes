//! Defines the PackageManifest struct used to serialize 
//! and deserialize package manifests
pub mod package_manifest;
//pub mod package_range;
pub(crate) mod package_target;

pub use package_manifest::PackageManifest;
//pub use package_range::PackageRange;
pub(crate) use package_target::PackageTarget;
pub type TargetMap = indexmap::IndexMap<String, PackageTarget>;

