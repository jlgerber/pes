//! Model a repository of published packages
// use pubgrub::{
//     version::Version,
//     package::Package,
// };
// std imports
use std::path::{Path, PathBuf};
// extern imports
use generator::{Gn, Generator};
// crate imports
use crate::PesError;

pub trait Repository {
    type Manifest;
    type Err;

    /// retrieve a manifest for the provided package and version
    fn manifest<P: AsRef<str>, V: AsRef<str> >(&self, package: P, version: V) -> Result<Self::Manifest, Self::Err>;
    
    /// retrieve manifests for the provided package
    fn manifests_for<P: AsRef<str> >(&self, package: P) -> Result<Vec<Self::Manifest>, PesError>;

    /// retrieve a generator over all of the manifests in a repository
    fn manifests(&self) -> Generator<'_, (), Result<Self::Manifest, Self::Err>> ;
}


#[derive(Debug, PartialEq, Eq)]
pub struct PackageRepository {
    // we expect the repository to be laid out like so:
    // /root/<package>/<version>/manifest.yaml
    root: PathBuf,
    /// the manifest name, including any subdirectories under the version
    manifest: String
}


impl Repository for PackageRepository {
    type Manifest = PathBuf;
    type Err = PesError;

    fn manifest<P: AsRef<str>, V: AsRef<str> >(&self, package: P, version: V) -> Result<Self::Manifest, Self::Err> {
        // construct path
        let mut manifest = self.root.clone();
        manifest.push(package.as_ref());
        manifest.push(version.as_ref());
        manifest.push("manifest.yaml");

        if manifest.exists() {
            Ok(manifest)
        } else {
            Err(PesError::MissingPath(manifest))
        }
    }

    fn manifests_for<P: AsRef<str> >(&self, package: P) -> Result<Vec<Self::Manifest>, PesError> {
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

        let g = Gn::new_scoped(move |mut s| {
           
            for dir in root.read_dir().unwrap() {
                let  path = dir.unwrap().path();
                if path.is_dir() {
                    for dir2 in path.read_dir().unwrap() {
                        let  mut path2 = dir2.unwrap().path();
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
        });
        return g
    }
}

impl PackageRepository {
    /// construct a new PackageRepository instance 
    pub fn new<P: Into<PathBuf>, M: Into<String> >(root: P, manifest: M) -> Self {
        Self {
            root: root.into(),
            manifest: manifest.into()
        }
    }
    /// return the root of the repository
    pub fn root(&self) -> &Path {
        return &self.root.as_path()
    }
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