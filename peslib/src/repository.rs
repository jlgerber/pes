//! Models a repository of published packages, generally of the form:
//! ```ignore
//! <root>/<package>/<version>/<manifest relpath>
//! ```
//! ## Example
//! ```ignore
//! /repo/foo/0.1.0/METADATA/manifest.yaml
//! ```
use pes_interface::RepoFinderService;
use log::{info,debug};
use std::path::{Path, PathBuf};
// extern imports
use generator::{Generator, Gn};
// crate imports
use crate::constants::{MANIFEST_NAME, PACKAGE_REPO_PATH_VAR_NAME, REPO_FINDER_VARNAME};
use crate::parser::parse_consuming_package_version;
use crate::PesError;
use crate::Repository;
use crate::PluginMgr;
// use lazy_static::lazy_static;

// lazy_static! {
//     /// This is an example for using doc comment attributes
//     static ref LIB: libloading::Library::new("target/debug/my_plugin.dll")
//     .expect("load library");
// }
/// A collection of package distributions
#[derive(Debug, PartialEq, Eq)]
pub struct PackageRepository {
    // we expect the repository to be laid out like so:
    // /root/<package>/<version>/manifest.yaml
    root: PathBuf,
    /// the manifest name, including any subdirectories under the version
    manifest: String,
}

// todo: Repository should be responsible for finding the path to a distribution (specific package
// version), not the manifest itself. a ManifestLocator should be responsible for taking a pathbuf
// to a distribution and producing a pathbuf pointing at a manifest
impl Repository for PackageRepository {
    type Manifest = PathBuf;
    type Err = PesError;

    fn root(&self) -> &Path {
        self.root.as_path()
    }
    fn manifest<P: AsRef<str>, V: AsRef<str>>(
        &self,
        package: P,
        version: V,
    ) -> Result<Self::Manifest, Self::Err> {
        // construct path
        let mut manifest = self.root.clone();
        manifest.push(package.as_ref());
        manifest.push(version.as_ref());
        manifest.push(MANIFEST_NAME);

        if manifest.exists() {
            Ok(manifest)
        } else {
            Err(PesError::MissingPath(manifest))
        }
    }

    fn manifest_for<P: AsRef<str>>(&self, distribution: P) -> Result<Self::Manifest, PesError> {
        let (name, version) = parse_consuming_package_version(distribution.as_ref())?;
        let version_str = version.to_string();
        let manifest_path = self.manifest(name, version_str.as_str())?;
        Ok(manifest_path)
    }

    fn manifests_for<P: AsRef<str>>(&self, package: P) -> Result<Vec<Self::Manifest>, PesError> {
        let mut manifest_path = self.root.clone();
        manifest_path.push(package.as_ref());

        let mut manifests = Vec::new();
        for entry in manifest_path.read_dir()? {
            let entry = entry?;
            let mut newpath = entry.path();
            newpath.push(&self.manifest);
            manifests.push(newpath);
        }
        Ok(manifests)
    }

    fn manifests(&self) -> Generator<'_, (), Result<Self::Manifest, Self::Err>> {
        let root = self.root.clone();

        Gn::new_scoped(move |mut s| {
            for dir in root.read_dir().unwrap() {
                let path = dir.unwrap().path();
                if path.is_dir() {
                    for dir2 in path.read_dir().unwrap() {
                        let mut path2 = dir2.unwrap().path();
                        if path2.is_dir() {
                            path2.push(&self.manifest);
                            if path2.is_file() {
                                s.yield_(Ok(path2));
                            } else {
                                s.yield_(Err(PesError::MissingPath(path2)));
                            }
                        }
                    }
                }
            }
            done!();
        })
    }
}

impl PackageRepository {
    /// construct a new PackageRepository instance
    pub fn new<P: Into<PathBuf>, M: Into<String>>(root: P, manifest: M) -> Self {
        Self {
            root: root.into(),
            manifest: manifest.into(),
        }
    }
    /// return the root of the repository
    pub fn root(&self) -> &Path {
        return &self.root.as_path();
    }

    /// Retrieve the location(s) of package repositories from the environment and
    /// return a vector of them, assuming they exist. If no repos are found, then
    /// return an Error.
    pub fn from_env() -> Result<Vec<PackageRepository>, PesError> {
        let repos_env = std::env::var(PACKAGE_REPO_PATH_VAR_NAME)?;
        // construct a vector of repos
        let repos = repos_env
            .split(":")
            .map(|x| Path::new(x))
            .filter_map(|x| (if x.exists() { Some(x) } else { None }))
            .map(|x| Self::new(x, MANIFEST_NAME))
            .collect::<Vec<_>>();
        if repos.len() == 0 {
            Err(PesError::NoRepositories(repos_env))
        } else {
            Ok(repos)
        }
    }

    /// Retrieve the locatons of package repositories from the plugin
    pub fn from_plugin(plugin_mgr: &PluginMgr) -> Result<Vec<PackageRepository>, PesError> {
        //let repos = Self::find_repos_via_plugin()?;
        let repos = plugin_mgr.repos();
        let repos = repos
            .iter()
            .filter_map(|x| (if x.exists() { Some(x) } else { None }))
            .map(|x| Self::new(x, MANIFEST_NAME))
            .collect::<Vec<_>>();
        Ok(repos)
    }

    // // find the repositories using the RepoFinderService plugin
    // fn find_repos_via_plugin() -> Result<Vec<PathBuf>, PesError> {

    //     #[cfg(target_os = "macos")]
    //     let dso_path = std::env::var(REPO_FINDER_VARNAME).unwrap_or_else(|_| "target/release/librepo_finder.dylib".to_string());
        
    //     #[cfg(target_os = "linux")]
    //     let dso_path = std::env::var(REPO_FINDER_VARNAME).unwrap_or_else(|_| "target/release/librepo_finder.dylib".to_string());
        
    //     #[cfg(target_os = "macos")]
    //     let lib = unsafe { libloading::Library::new(dso_path.as_str())?};
        
    //     #[cfg(target_os = "linux")]
    //     let lib = unsafe { libloading::Library::new("target/release/librepo_finder.so")?};

    //     let new_service: libloading::Symbol<extern "Rust" fn() -> Box<dyn RepoFinderService>> =
    //         unsafe {lib.get(b"new_finder_service")?};
    //     let service = new_service();
    
    //     let repo = service.find_repo();
    //     info!("found {:?}", &repo);
    //     Ok(repo)
    // }
}

// using generator instead
//
// /// Iterator for the Packagerepository
// pub struct PackageRepositoryIterator<'a> {
//         root: &'a std::path::Path,
//         package: Option<std::fs::ReadDir>, //<std::path::Iter<'a>>,
//         versions: Option<std::fs::ReadDir>
// }
// impl<'a> Iterator for PackageRepositoryIterator<'a> {
//     type Item = PathBuf;

//     fn next(&mut self) -> Option<PathBuf> {
// ... using generators instead. far simpler. wish that they would
// stabilize generators....
// }

#[cfg(test)]
#[path = "./unit_tests/repository.rs"]
mod unit_tests;
