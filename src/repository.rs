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
    root: PathBuf
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
            newpath.push("manifest.yaml");
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
                            path2.push("manifest.yaml");
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
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self {
            root: root.into()
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

//        if self.package.is_none() {
//            // returns a Result<ReadDir>
//            let mut dir_iter = self.root.read_dir().ok()?;
//            // returns a Result<DirEntry>
//            let  first_package_entry = dir_iter.next()?.ok()?; 
//            // get PathBuf
//            let  first_package = first_package_entry.path();`~
//            // get Result<ReadDir>
//            let mut version_iter = match first_package.read_dir()  {
//                 Ok(val) => val,
//                 Err(_) => {self.package = Some(dir_iter); return None}
//            };
//            // get Result<DirEntry>
//            let  first_version_entry = version_iter.next()?.ok()?;
//            // get pathbuf
//            let mut first_version = first_version_entry.path();
//            // add manifest
//            first_version.push("manifest.yaml");
//            self.versions = Some(version_iter);
//            self.package = Some(dir_iter);
//            return Some(first_version)
//        }
       
//        if self.versions.is_none() {return None;}
//        let  versions = self.versions.as_mut().unwrap();
//        let next_version = versions.next();
//        if next_version.is_none() {
//             // unwrap package and get the next
//             let  package_iter = self.package.as_mut().unwrap();
//             let  next_package_entry =  package_iter.next()?.ok()?;
//             // get PathBuf
//             let  next_package = next_package_entry.path();
//             // get Result<ReadDir>
//             let mut version_iter = match next_package.read_dir() {
//                 Ok(val) => val,
//                 Err(_) => { return None}
//            };
//            // get Result<DirEntry>
//            let  version_entry = version_iter.next()?.ok()?;
//            // get pathbuf
//            let  next_version = version_entry.path();
//            self.versions = Some(version_iter);
//            return Some(next_version);
//        } else {
//            let next_version = next_version.unwrap().ok()?.path();
//             return Some(next_version);
//        }
//     }
// }


#[cfg(test)]
#[path = "./unit_tests/repository.rs"]
mod unit_tests;