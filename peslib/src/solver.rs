//! Components which participate in generating a dependency closure for a set of package version constraints

// what does the solver have to do?
// we need to build up a list of packages and package constraints
// ie iterate through the package manifests for each distribution in
// a repo and register it with the dependency provider
// finally create a fake package representing the solver request
// (package + withs) as the solver only allows you to request a single
// package

use std::path::Path;
use std::rc::Rc;
use log::*;

use pubgrub::{
    error::PubGrubError,
    package::Package,
    range::Range,
    report::{DefaultStringReporter, Reporter},
    solver::{resolve, OfflineDependencyProvider},
    version::{Version},
};

pub use pubgrub::type_aliases::SelectedDependencies;

use crate::{
    aliases::{DistMap, SolveResult, DistPathMap}, 
    constants::ROOT_REQUEST,
    distribution_range::DistributionRange, 
    manifest::Manifest,
    manifest::PackageManifest, 
    parser::parse_consuming_package_version,
    PesError, 
    PluginMgr,
    ReleaseType,PackageRepository,
    Repository, 
    SemanticVersion, 
};


/// Given a set of constraints and an instance of the plugin manager, performa solve
pub fn perform_solve(
    plugin_mgr: &PluginMgr,
    constraints: &Vec<&str>, 
    include_pre: bool,
) -> Result<SolveResult, PesError> {
    debug!("user supplied constraints: {:?}.", constraints);

    // construct request from a vector of constraint strings
    let request = constraints
        .iter()
        .map(|x| DistributionRange::from_str(x))
        .collect::<Result<Vec<_>, PesError>>()?;

    let repos = PackageRepository::from_plugin(plugin_mgr)?;
    let min_release_type = if include_pre {ReleaseType::Alpha} else {ReleaseType::Release};
    // if we are not including prereleases, we search for any explicitly defined
    // constraints that happen to be pre-releases, and pass them through as overrides.
    let dist_overrides = if include_pre {Vec::new()} else {
        constraints
            .iter()
            .filter_map(|x| parse_consuming_package_version(x).ok())
            .filter(|(_name, version)| version.release_type < ReleaseType::Release)
            .map(|(name, version)| (name.to_string(), version))
            .collect::<Vec<_>>()
    };
    let dist_overrides = Rc::new(dist_overrides);
    let mut solver = Solver::new_from_repos(repos, min_release_type, dist_overrides)?;
    // calculate the solution
    debug!("Calling solver.solve with request {:?}", &request);
    let mut solution = solver.solve(request)?;

    // // remove the root request from the solution as that is not a real package
    solution.remove(ROOT_REQUEST);
    let mut distpathmap = DistPathMap::new();
    
    solution
        .iter()
        .map(|(ref name, ref version)|{
            let dist = format!("{}-{}", name, version);
            let dist_path = solver.dist_path(&dist).ok_or(PesError::DistributionPathNotFound(dist.clone()))?;
            distpathmap.insert(dist, dist_path.to_string_lossy().to_string()); 
            Ok(())
        }).collect::<Result<(), PesError>>()?; 
    // turns out that we handle this in Solver::new_from_repos(...)
    // I should measure whether the following improves or impedes performance...
    // It isnt incorrect to do. Its just unneccessary from a solution standpoint.
    // if include_pre {
    //     solution
    //     .iter()
    //     .map(|(ref name, ref version)|{
    //         let dist = format!("{}-{}", name, version);
    //         let dist_path = solver.dist_path(&dist).ok_or(PesError::DistributionPathNotFound(dist.clone()))?;
    //         distpathmap.insert(dist, dist_path.to_string_lossy().to_string()); 
    //         Ok(())
    //     }).collect::<Result<(), PesError>>()?; 
    // } else {
    //     solution
    //     .iter()
    //     // we are going to keep the distribution if it is either (a) a Release or 
    //     // (b) it matches one of the preserve_pre items passed in
    //     .filter(|(ref name, version)| version.release_type >= ReleaseType::Release || 
    //                                   preserve_pre.iter().any(|x| name.as_str() == x.0 && *version == &x.1))
    //     .map(|(ref name, ref version)|{
    //         let dist = format!("{}-{}", name, version);
    //         let dist_path = solver.dist_path(&dist).ok_or(PesError::DistributionPathNotFound(dist.clone()))?;
    //         distpathmap.insert(dist, dist_path.to_string_lossy().to_string()); 
    //         Ok(())
    //     }).collect::<Result<(), PesError>>()?; 
    // }
    
    Ok((distpathmap, solution))
}

/// Generate a solution for the provided distribution and target
pub fn perform_solve_for_distribution_and_target(
    plugin_mgr: &PluginMgr,
    distribution: &str,
    target: &str,
    // indicate whether or not you wish to include prereleases in the solution space.
    // By default, the function ignores all pre-releases other than potentially 
    // the supplied distribution.
    include_pre: bool
) -> Result<SolveResult, PesError> {
    debug!("distribution: {} target: {}", distribution, target);
    let repos = PackageRepository::from_plugin(plugin_mgr)?;
    let mut path = None;
    for repo in &repos {
        let manifest = repo.manifest_for(distribution);
        if manifest.is_ok() {
            path = Some(manifest.unwrap());
            break;
        }
    }
   
    if path.is_none() {
        return Err(PesError::DistributionNotFound(distribution.to_string()));
    }
    let manifest = Manifest::from_path(path.unwrap())?;
    let request = manifest.get_requires(target)?;

    let min_release_type = if include_pre {ReleaseType::Alpha} else {ReleaseType::Release};
    let dist_overrides = if include_pre {Rc::new(Vec::new())} else {
        let (name, version) = parse_consuming_package_version(distribution)?;
        Rc::new(vec![(name.to_string(), version)])
    };

    let mut solver = Solver::new_from_repos(repos, min_release_type, dist_overrides)?;
    let mut solution = solver.solve(request)?;
    solution.remove(ROOT_REQUEST);
    // store a mapping between distributions and their paths on disk
    let mut distpathmap = DistPathMap::new();
    // get the path to the requested distribution and then insert requested distribution and its path into the map
    let dist_path = solver.dist_path(distribution).ok_or(PesError::DistributionNotFound(distribution.to_string()))?;
    distpathmap.insert(distribution.to_string(), dist_path.to_string_lossy().to_string());
   
    if include_pre {
        solution
        .iter()
        .map(|(ref name, ref version)|{
            let dist = format!("{}-{}", name, version);
            let dist_path = solver.dist_path(&dist).ok_or(PesError::DistributionPathNotFound(dist.clone()))?;
            distpathmap.insert(dist, dist_path.to_string_lossy().to_string()); 
            Ok(())
        }).collect::<Result<(), PesError>>()?; 
    } else {
        // if we are not going to include pre-releases
        let dist_pair = parse_consuming_package_version(distribution)?;

        solution
        .iter()
        // in this case we need to filter out any pre-release versions
        .filter(|(ref name, version)| version.release_type >= ReleaseType::Release|| 
                     (dist_pair.0 == name.as_str() && &dist_pair.1 == *version))
        .map(|(ref name, ref version)|{
            let dist = format!("{}-{}", name, version);
            let dist_path = solver.dist_path(&dist).ok_or(PesError::DistributionPathNotFound(dist.clone()))?;
            distpathmap.insert(dist, dist_path.to_string_lossy().to_string()); 
            Ok(())
        }).collect::<Result<(), PesError>>()?; 
    }
    
    Ok((distpathmap, solution))
}

/// Solver holds needed state to perform dependency closure solve
#[derive(Debug)]
pub struct Solver<P: Package, V: Version> {
    pub dependency_provider: OfflineDependencyProvider<P, V>,
    dist_cache: DistMap,
}


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
    /// Construct a new Solver instacne from a vec of repositories. All of the 
    /// distributions within each repository will be appropriately registered with
    /// the solver so that they may be considered in calculating the dependency
    /// closure when `solve` is later invoked.
    pub fn new_from_repos(
        repos: Vec<PackageRepository>,
        min_release_type: ReleaseType, 
        distributions_override: std::rc::Rc<Vec<(String, SemanticVersion)>>

    ) -> Result<Solver<String, SemanticVersion>, PesError> {
        let mut solver = Solver::new();

        for repo in repos {
            solver.add_repository(&repo, min_release_type, distributions_override.clone())?;
        }
        Ok(solver)
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
    pub fn add_repository<R: Repository>(
        &mut self, 
        repository: &R, 
        min_release_type: ReleaseType, 
        distributions_override: std::rc::Rc<Vec<(String, SemanticVersion)>>
    ) -> Result<(), PesError> {
        for manifest_path in repository.manifests(min_release_type, distributions_override) {
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
            ROOT_REQUEST.to_string(),
            SemanticVersion::new(1, 0, 0, ReleaseType::Release),
            requires,
        );
        match resolve(
            &self.dependency_provider,
            ROOT_REQUEST.to_string(),
            SemanticVersion::new(1, 0, 0, ReleaseType::Release),
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
