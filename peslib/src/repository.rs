//! Models a repository of published packages, generally of the form:
//! ```ignore
//! <root>/<package>/<version>/<manifest relpath>
//! ```
//! ## Example
//! ```ignore
//! /repo/foo/0.1.0/METADATA/manifest.yaml
//! ```
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;

// extern imports
use generator::{Generator, Gn};
// crate imports
//use crate::constants::{MANIFEST_NAME, /*PACKAGE_REPO_PATH_VAR_NAME*/ };
use crate::parser::{parse_consuming_package_version, parse_consuming_semver};
use crate::PesError;
use crate::Repository;
use crate::PluginMgr;
use crate::{ReleaseType, SemanticVersion};

/// A collection of package distributions
#[derive(Debug, PartialEq, Eq)]
pub struct PackageRepository<'a> {
    // we expect the repository to be laid out like so:
    // /root/<package>/<version>/manifest.yaml
    root: PathBuf,
    /// plugin manager from which we get the list of potential repository locations
    /// and the relative path to the package manifest from the distribution root
    plugin_mgr: &'a PluginMgr,
}


impl<'a> Repository for PackageRepository<'a> {
    type Manifest = PathBuf;
    type Distribution = PathBuf;
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
        let manifest = self.plugin_mgr.manifest_path_from_distribution(manifest);

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

    fn manifests_for<P: AsRef<str>>(&self, package: P, min_release_type: ReleaseType) -> Result<Vec<Self::Manifest>, PesError> {
        let mut manifest_path = self.root.clone();
        manifest_path.push(package.as_ref());

        let mut manifests = Vec::new();
        for entry in manifest_path.read_dir()? {
            let entry = entry?;
            let manifest_path = self.plugin_mgr.manifest_path_from_distribution(entry.path());
            if min_release_type != ReleaseType::Alpha {
                let version = manifest_path
                                .as_path()
                                .parent().ok_or_else(|| PesError::InvalidPath(manifest_path.clone()))?
                                .file_name().ok_or_else(|| PesError::InvalidVersion(format!("{:?} unable to extract directory from PathBuf via as_path().file_name()", manifest_path)))?;
                let version = version.to_string_lossy().to_string();

                let version = parse_consuming_semver(version.as_str())?;
                if version.release_type < min_release_type {
                    continue;
                }
            }
            manifests.push(manifest_path);
        }
        Ok(manifests)
    }

    fn manifests(
        &self,  
        min_release_type: ReleaseType, 
        distributions_override: Rc<Vec<(String, SemanticVersion)>>
    ) -> Generator<'_, (), Result<Self::Manifest, Self::Err>> {
        let root = self.root.clone();
        
        let overrides = distributions_override.clone();

        Gn::new_scoped(move |mut s| {
            let overrides = overrides
            .iter()
            .filter(|(_name, version)| version.release_type != ReleaseType::Release)
            .collect::<Vec<_>>();
            for dir in root.read_dir().unwrap() {
                let path = dir.unwrap().path();
                if path.is_dir() {
                    for dir2 in path.read_dir().unwrap() {
                        let path2 = dir2.unwrap().path();
                        if path2.is_dir() {
                            if min_release_type > ReleaseType::Alpha {
                                let version_string = path2.file_name().unwrap().to_string_lossy().to_string();
                                let version = match SemanticVersion::from_str(version_string.as_str()) {
                                    Ok(version) => version,
                                    Err(e) => panic!(format!("unable to extract semantic version from {}. Error: {}", version_string.as_str(), e))//s.yield_(Err(PesError::InvalidVersion(version_string)))
                                };
                                if version.release_type < min_release_type {
                                    // for example. min_release_type = Release & version = Beta. 
                                    // Beta < Release so we skip.
                                    let mut found_match = false;
                                    if overrides.len() > 0 {
                                        let package = path2.parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
                                        if overrides.iter().any(|(candidate_name, candidate_version)| candidate_name == &package && candidate_version == &version) {
                                            found_match = true;
                                        }
                                    }
                                    // we see if any of the overrides match the current package and version
                                    // if so, we dont skip
                                    if !found_match {
                                        continue;
                                    }
                                }
                            }
                            let manifest_path = self.plugin_mgr.manifest_path_from_distribution(path2);
                            if manifest_path.is_file() {
                                s.yield_(Ok(manifest_path));
                            } else {
                                s.yield_(Err(PesError::MissingPath(manifest_path)));
                            }
                        }
                    }
                }
            }
            done!();
        })
    }

    fn distributions(&self, min_release_type: ReleaseType, distributions_override: std::rc::Rc<Vec<(String, SemanticVersion)>>)-> Generator<'_, (), Result<Self::Distribution, Self::Err>> {
        let root = self.root.clone();
        let overrides = distributions_override.clone();
        Gn::new_scoped(move |mut s| {
            let overrides = overrides
            .iter()
            .filter(|(_name, version)| version.release_type != ReleaseType::Release)
            .collect::<Vec<_>>();

            for dir in root.read_dir().unwrap() {
                let package = dir.unwrap().path();
                if package.is_dir() {
                    for version in package.read_dir().unwrap() {
                        let version = version.unwrap().path();
                        if version.is_dir() {
                            if min_release_type < ReleaseType::Release {
                                let semver_str = version.file_name().unwrap().to_string_lossy().to_string();
                                let semver = match SemanticVersion::from_str(semver_str.as_str()) {
                                    Ok(version) => version,
                                    Err(e) => panic!(format!("unable to extract semantic version from {}. Error: {}", semver_str.as_str(), e))//s.yield_(Err(PesError::InvalidVersion(version_string)))
                                };
                                if semver.release_type < min_release_type {
                                    let pkg = package.file_name().unwrap().to_string_lossy().to_string();
                                    // if none of the overrides match the current name and version, then we continue looping without
                                    // yielding the distribution. We do this because we have already established that the current
                                    // distribution's verison's release type is less than the minimim release type specified
                                    if !overrides.iter().any(|(name,version)| name == pkg.as_str() && version == &semver)  {
                                        continue
                                    }
                                }
                            }
                            s.yield_(Ok(version));
                        } else {
                            s.yield_(Err(PesError::MissingPath(version)));
                        }

                    }
                }
            }
            done!();
        })
    }

    fn has_distribution<D: AsRef<str>>(&self, distribution: D) -> Result<bool, Self::Err> {
        let (name, version) = parse_consuming_package_version(distribution.as_ref())?;
        let mut root = self.root.clone();
        root.push(name);
        let version = version.to_string();
        root.push(version.as_str());
        Ok(root.exists())
    }

}

impl<'a> PackageRepository<'a> {
    /// construct a new PackageRepository instance
    pub fn new<P: Into<PathBuf>>(root: P,  plugin_mgr: &'a PluginMgr) -> Self {
        Self {
            root: root.into(),
            plugin_mgr
        }
    }
    /// return the root of the repository
    pub fn root(&self) -> &Path {
        return &self.root.as_path();
    }

    /// Retrieve the locatons of package repositories from the plugin
    pub fn from_plugin(plugin_mgr: &'a PluginMgr) -> Result<Vec<PackageRepository>, PesError> {
        //let repos = Self::find_repos_via_plugin()?;
        let repos = plugin_mgr.repos();
        let repos = repos
            .iter()
            .filter_map(|x| (if x.exists() { Some(x) } else { None }))
            .map(|x| Self::new(x, &plugin_mgr))
            .collect::<Vec<_>>();
        Ok(repos)
    }

    pub fn packages(&self) -> Generator<'_, (), Result<PathBuf, PesError>> {
        let root = self.root.clone();
        Gn::new_scoped(move |mut s| {
            for dir in root.read_dir().unwrap() {
                let path = dir.unwrap().path();
                if path.is_dir() {
                    s.yield_(Ok(path));
                }
            }
            done!();
        })
    }
}

#[cfg(test)]
#[path = "./unit_tests/repository.rs"]
mod unit_tests;
