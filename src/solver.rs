// what does the solver have to do?
// we need to build up a list of packages and package constraints
// ie iterate through the package manifests for each distribution in 
// a repo and register it with the dependency provider
// finally create a fake package representing the solver request 
// (package + withs) as the solver only allows you to request a single 
// package 

use pubgrub::solver::OfflineDependencyProvider;
//use pubgrub::package::Package;
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;
use crate::repository::Repository;
use crate::PesError;
use crate::manifest::PackageManifest;
use crate::versioned_package::VersionedPackage;
use pubgrub::solver::resolve;
use pubgrub::type_aliases::SelectedDependencies;
use pubgrub::error::PubGrubError;
use pubgrub::report::DefaultStringReporter;
use pubgrub::report::Reporter;


#[derive(Debug)]
pub struct Solver {
   pub  dependency_provider: OfflineDependencyProvider<String, SemanticVersion>,
}

const ROOT: &'static str = "ROOT_REQUEST";

impl Solver {
    pub fn new() -> Self {
        Self {
            dependency_provider: OfflineDependencyProvider::<String, SemanticVersion>::new() 
        }
    }
    /// iterate over package names
    pub fn packages(&self) -> impl Iterator<Item = &str> {
        self.dependency_provider.packages().map(|x| x.as_str())
    }
    pub fn versions(&self, package: &str) -> Option<impl Iterator<Item = &SemanticVersion>> {
        self.dependency_provider.versions(&package.to_string())
    }
    /// add packages from a repository to the dependency provider
    pub fn add_repository<R: Repository>(&mut self, repository: &R) -> Result<(), PesError> {

        for manifest_path in repository.manifests() {
            let manifest_path = manifest_path
                                .map_err(|e| PesError::PesError(format!("{:?}", e)) )?;
            let manifest = PackageManifest::from_file(manifest_path)?; 
            let requires: Vec<(String, Range<SemanticVersion>)> = manifest
                                                                    .get_requires("run")
                                                                    .unwrap_or_else(|_| Vec::<VersionedPackage>::new())
                                                                    .into_iter()
                                                                    .map(|versioned_package| {
                let VersionedPackage{name, range, ..} = versioned_package; 
                (name.to_string(), range)
            }).collect();
            let PackageManifest{name, version, ..} = manifest; 
            self.dependency_provider.add_dependencies(name, version, requires);
        }
        Ok(())
    }

    /// calculate a solution
    pub fn solve(&mut self, request: Vec<VersionedPackage>) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        let requires: Vec<(String, Range<SemanticVersion>)> = request
                                                                .into_iter()
                                                                .map(|versioned_package| {
                let VersionedPackage{name, range, ..} = versioned_package; 
                (name.to_string(), range)
            }).collect();
            // create a fake package to house the requested version constraints
            self.dependency_provider.add_dependencies(ROOT.to_string(), SemanticVersion::new(1,0,0), requires);
            match resolve(&self.dependency_provider, ROOT.to_string(), SemanticVersion::new(1,0,0))  {
                Ok(solution) => Ok(solution),
                Err(PubGrubError::NoSolution(mut derivation_tree)) => {
                    derivation_tree.collapse_no_versions();
                    Err(PesError::NoSolution(format!("{}", DefaultStringReporter::report(&derivation_tree))))
                },
                Err(err) => Err(PesError::PesError(format!("{:?}", err))),
            }
    }

    // utility function facilitating unit testing
    pub(crate) fn convert_request_str(request: &str) -> Vec<VersionedPackage> {
        let requested: Vec<VersionedPackage> = request
            .split(' ')
            .map(|s| VersionedPackage::from_str(s))
            .filter_map(|s| s.ok())
            .collect();
        requested
    }
    
    /// solve for the requested constraints in the provided str
    /// # Example
    /// ```ignore
    /// maya-1.0.1,maya-plugins,maya-startup-1.2.3+<4
    /// or 
    /// maya-1 maya-plugins maya-startup-1.2.3+<4
    /// ```
    pub fn solve_from_str(&mut self, request: &str) -> Result<SelectedDependencies<String, SemanticVersion>, PesError> {
        // split between 
        let request = request.replace(",", " ");
        let requested = Self::convert_request_str(&request);
        self.solve(requested)
    }
}



#[cfg(test)]
#[path = "./unit_tests/solver.rs"]
mod unit_tests;