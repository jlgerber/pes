//! Components which participate in generating a dependency closure for a set of package version constraints

// what does the solver have to do?
// we need to build up a list of packages and package constraints
// ie iterate through the package manifests for each distribution in
// a repo and register it with the dependency provider
// finally create a fake package representing the solver request
// (package + withs) as the solver only allows you to request a single
// package

use std::path::Path;

use pubgrub::{
    error::PubGrubError,
    package::Package,
    range::Range,
    report::{DefaultStringReporter, Reporter},
    solver::{resolve, OfflineDependencyProvider},
    //type_aliases::SelectedDependencies,
    version::{SemanticVersion, Version},
};

pub use pubgrub::type_aliases::SelectedDependencies;

use crate::{
    aliases::DistMap, distribution_range::DistributionRange, manifest::Manifest,
    manifest::PackageManifest, PesError, Repository,
};

// todo: add hashmap to store full paths to dependencies
#[derive(Debug)]
pub struct Solver<P: Package, V: Version> {
    pub dependency_provider: OfflineDependencyProvider<P, V>,
    dist_cache: DistMap,
}

const ROOT: &str = "ROOT_REQUEST";

impl Default for Solver<String, SemanticVersion> {
    fn default() -> Self {
        Self {
            dependency_provider: OfflineDependencyProvider::new(),
            dist_cache: DistMap::new(),
        }
    }
}

impl Solver<String, SemanticVersion> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieve an iterator over package names that have been registered via ```add_repository```
    pub fn packages(&self) -> impl Iterator<Item = &str> {
        self.dependency_provider.packages().map(|x| x.as_str())
    }

    /// Retrieve an iterator over SemanticVersions for a provided package that have been registered
    pub fn versions(&self, package: &str) -> Option<impl Iterator<Item = &SemanticVersion>> {
        self.dependency_provider.versions(&package.to_string())
    }
    /// Retrieve the path to the supplied distribution, assuming it exists
    pub fn dist_path(&self, distribution: &str) -> Option<&Path> {
        self.dist_cache.get(distribution).map(|x| x.as_path())
    }
    /// Add packages from a repository to the dependency provider
    pub fn add_repository<R: Repository>(&mut self, repository: &R) -> Result<(), PesError> {
        for manifest_path in repository.manifests() {
            let manifest_path =
                manifest_path.map_err(|e| PesError::PesError(format!("{:?}", e)))?;
            let mut dist_path = manifest_path.as_ref().to_path_buf();
            // we currently need to know the details of manifest location. we should change repository to return
            // dist paths and then have a manifestfactory
            dist_path.pop();
            let manifest = PackageManifest::from_file(manifest_path)?;
            let requires: Vec<(String, Range<SemanticVersion>)> = manifest
                .get_requires("run")
                .unwrap_or_else(|_| Vec::<DistributionRange>::new())
                .into_iter()
                .map(|distribution_range| {
                    let DistributionRange { name, range, .. } = distribution_range;
                    (name.to_string(), range)
                })
                .collect();
            let PackageManifest { name, version, .. } = manifest;
            // update distribution cache so that we can print it out later
            self.dist_cache
                .insert(format!("{}-{}", name.as_str(), &version), dist_path);
            self.dependency_provider
                .add_dependencies(name, version, requires);
        }
        Ok(())
    }

    /// calculate a solution
    pub fn solve(
        &mut self,
        request: Vec<DistributionRange>,
    ) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        let requires: Vec<(String, Range<SemanticVersion>)> = request
            .into_iter()
            .map(|distribution_range| {
                let DistributionRange { name, range, .. } = distribution_range;
                (name.to_string(), range)
            })
            .collect();
        // create a fake package to house the requested version constraints
        self.dependency_provider.add_dependencies(
            ROOT.to_string(),
            SemanticVersion::new(1, 0, 0),
            requires,
        );
        match resolve(
            &self.dependency_provider,
            ROOT.to_string(),
            SemanticVersion::new(1, 0, 0),
        ) {
            Ok(solution) => Ok(solution),
            Err(PubGrubError::NoSolution(mut derivation_tree)) => {
                derivation_tree.collapse_no_versions();
                Err(PesError::NoSolution(DefaultStringReporter::report(
                    &derivation_tree,
                )))
            }
            Err(err) => Err(PesError::PesError(err.to_string())),
        }
    }

    /// Given the path to a manifest and the name of a target within the manifest, calculate the solution
    pub fn solve_target_from_manifest(
        &mut self,
        target: &str,
        manifest: &Path,
    ) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        let manifest = Manifest::from_path(manifest)?;
        let request = manifest.get_requires(target)?;
        self.solve(request)
    }

    // utility function facilitating unit testing
    pub(crate) fn convert_request_str(request: &str) -> Vec<DistributionRange> {
        let requested: Vec<DistributionRange> = request
            .split(' ')
            .map(|s| DistributionRange::from_str(s))
            .filter_map(|s| s.ok())
            .collect();
        requested
    }

    /// Solve for the requested constraints in the provided ```requestr``` string
    /// # Example
    /// ```ignore
    /// maya-1.0.1,maya-plugins,maya-startup-1.2.3+<4
    /// or
    /// maya-1 maya-plugins maya-startup-1.2.3+<4
    /// ```
    pub fn solve_from_str(
        &mut self,
        request: &str,
    ) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        // replace comma separation with space separation in case the request string is comming from the command line
        let request = request.replace(",", " ");
        // convert the request string to a Vec<VersionPackage>
        let requested = Self::convert_request_str(&request);
        // call the solve method with the vector of VersionPackage
        self.solve(requested)
    }

    /// Generate a solve for a package using the provided manaifest and target to identify the dependencies.
    pub fn solve_from_manifest(
        &mut self,
        manifest_path: impl AsRef<Path>,
        target: &str,
    ) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        // get an instance of PackageManifest from the provided manifest path
        let manifest = PackageManifest::from_file(manifest_path.as_ref())?;
        // get_requires returns a Vec<DistributionRange>
        let requested = manifest.get_requires(target)?;
        // call the solve method with the vector of versioned packages
        self.solve(requested)
    }
}

#[cfg(test)]
#[path = "./unit_tests/solver.rs"]
mod unit_tests;
