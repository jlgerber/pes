//! Components used to create, modify, read, and write  a package's manifest.
//!
//! This is achieved primarily through the `PackageManifest` struct

pub mod package_manifest;
//pub mod package_range;
pub(crate) mod package_target;

pub use package_manifest::PackageManifest;
//pub use package_range::PackageRange;
pub(crate) use package_target::PackageTarget;
/// custom type mapping a target name to a PackageTarget
pub type TargetMap = indexmap::IndexMap<String, PackageTarget>;

