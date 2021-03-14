//! Models a repository of published packages, generally of the form:
//! ```ignore
//! <root>/<package>/<version>/<manifest relpath>
//! ```
//! ## Example
//! ```ignore
//! /repo/foo/0.1.0/METADATA/manifest.yaml
//! ```

use std::path::{Path, PathBuf};
// extern imports
use generator::{Generator, Gn};
// crate imports
use crate::constants::{MANIFEST_NAME, /*PACKAGE_REPO_PATH_VAR_NAME*/ };
use crate::parser::parse_consuming_package_version;
use crate::PesError;
use crate::Repository;
use crate::PluginMgr;

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

    fn manifests_for<P: AsRef<str>>(&self, package: P) -> Result<Vec<Self::Manifest>, PesError> {
        let mut manifest_path = self.root.clone();
        manifest_path.push(package.as_ref());

        let mut manifests = Vec::new();
        for entry in manifest_path.read_dir()? {
            let entry = entry?;
            let manifest_path = self.plugin_mgr.manifest_path_from_distribution(entry.path());
           
            manifests.push(manifest_path);
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
                        let path2 = dir2.unwrap().path();
                        if path2.is_dir() {
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

}

#[cfg(test)]
#[path = "./unit_tests/repository.rs"]
mod unit_tests;
