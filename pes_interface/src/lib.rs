//! pes_interface crate provides the core traits necessary to implmeent 
//! dynamic libraries used to define location specific behavior for tasks
//! such as finding package repositories on disk and finding the manifest
//! within a distribution. These sorts of tasks tend to be specific to the 
//! deployment; different jobsystem architects will want to make different
//! decisions based on pre-existing usage patterns.

pub mod traits;

pub use traits::*;
