//! utils command
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::CString,
    path::PathBuf,
    rc::Rc,
};

use itertools::join;
use log::{debug, info};
use nix::unistd::execve;
//use users::{get_user_by_uid, get_current_uid};

use peslib::{
    prelude::*,
    jsys::*,
    parser::parse_consuming_all_paths_with_provider,
    SelectedDependencies,
};


// setup the solver, adding package repositories
fn setup_solver(repos: Vec<PackageRepository>) -> Result <Solver<String, SemanticVersion>, PesError> 
{
    let mut solver = Solver::new();
    for repo in repos {
        solver.add_repository(&repo)?;
    }
    Ok(solver)
}


/// given a set of constraints, calculate a solution
pub fn perform_solve(constraints: Vec<String>) -> Result<SelectedDependencies<String, SemanticVersion>,PesError> {
   
    debug!("user supplied constraints: {:?}.", constraints );
    
        // construct request
    let request = constraints.iter().map(|x| VersionedPackage::from_str(x.as_str())).collect::<Result<Vec<_>,PesError>>()?;
    let repos = PackageRepository::from_env()?;
    let mut solver = setup_solver(repos)?;
    // calculate the solution
    let solution = solver.solve(request)?;
    debug!("Solver solution:\n{:#?}", solution);
    Ok(solution)
}
///
pub fn solve_for_distribution_and_target(distribution: &str, target: &str) -> Result<SelectedDependencies<String, SemanticVersion>,PesError> {
    debug!("distribution: {} target: {}", distribution, target);
    let repos = PackageRepository::from_env()?;
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
    let mut solver = setup_solver(repos)?;
    let solution = solver.solve(request)?;
    debug!("Solver solution:\n{:#?}", solution);
    Ok(solution)
}

/// Initialize the log given the provided level
pub fn init_log(level: &str) {
    match level {
        "trace" | "debug" | "info" | "warn" | "error" | "critical" => std::env::set_var("RUST_LOG", level),
        _ => std::env::set_var("RUST_LOG","warn"),
    }
    pretty_env_logger::init();
}

/// launch an interactive shell given a solution
pub fn launch_shell(solution: SelectedDependencies<String, SemanticVersion>) -> Result<(), PesError> {
   
    // construct a list of repositories
    let repos = PackageRepository::from_env()?;
    // iterate through the solve. For each package version, find it in a repository and store it
    // in a hashmap
    let mut manifests = HashMap::<String, (PathBuf, Manifest)>::new();
    // define a var to hold a list of distributions for which we cannot find manifests
    let mut missing_manifests = Vec::new();
    // solution is a HashMap of (package,version) pairs
    for (package, version) in solution.iter() {
        // search through repositories for registered manifests                
        let mut manifest_path = None;
        for repo in &repos {
            let version_str = version.to_string();
            // let distribution = PathBuf::from("")
            match repo.manifest(package, &version_str) {
                Ok(path) => manifest_path = Some(path),
                Err(_) => (),
            }
        }

        // if we found a manifest path, construct the actual manifest and 
        // add it to the hashmap tracking manifests
        if let Some(mut path) = manifest_path {
            let distribution = format!("{}-{}", package, version);
            let mani = Manifest::from_path(&path)?;
            // remove manifest from path
            // todo: introduce abstraction for finding manifest & root of package
            path.pop();
            manifests.insert(distribution, (path, mani));
        } else if package.as_str() != "ROOT_REQUEST" {
            let distribution = format!("{}-{}", package, version);
            // if we were unable to find the manifest, add it to the list of missing manifests
            missing_manifests.push(distribution);
        }
    }
    
    if missing_manifests.len() > 0 {
        return Err(PesError::MissingManifests(missing_manifests));
    }

    // hashmap to store env vars
    //let mut env_vars = HashMap::new();
    let jsys = JsysCleanEnv::new();
    // TODO: change base_env2 call to base_env
    let mut env_vars = jsys.base_env2();
    // instantiate provider
    let provider = std::rc::Rc::new(RefCell::new(BasicVarProvider::new()));
    
    // iterate through package manifests, building environment
    for (name, (root, manifest)) in manifests {
        debug!("name: {}", name);
        {
            let mut prov = provider.borrow_mut();
            prov.insert_var("root", root.as_path().display().to_string());
        }
        for (key, value) in manifest.environment() {
            debug!("{} {}", &key, value);
            let result = parse_consuming_all_paths_with_provider(Rc::clone(&provider), value)?;
            debug!("{:?}", result);
            {
                if let Some(val) = env_vars.get_mut(key) {
                    *val += result;
                } else {    
                    env_vars.insert(key.clone(), result);
                }
            }
        }
    }
    let mut c_env_vars: Vec<std::ffi::CString> = Vec::with_capacity(env_vars.len());
    debug!("OUTPUT VARS");
    // construct environment vec<CString> for execve call
    for (k, v ) in env_vars {
       
        let existing_paths = v.inner();
                                //.into_iter()
                                //.filter(|x| x.exists())
                                //.map(|x| x.display().to_string())
                                //.collect::<Vec<_>>();
        // construct required format for execve
        let existing_paths = format!("{}={}",k, join(existing_paths, ":")) ;
        info!("{}", &existing_paths);
        //let existing_paths : std::ffi::CString = existing_paths.bytes().into();
        let existing_paths = std::ffi::CString::new(&existing_paths[..]).expect("unable to convert to cstring");
        c_env_vars.push(existing_paths);
    }

    // identify shell
    let shell = CString::new("/usr/bin/env").unwrap();
    let args = vec![CString::new("-i").unwrap(),CString::new("bash").unwrap(),CString::new("--noprofile").unwrap(), CString::new("--norc").unwrap()];
    // call execve with environment vec
    execve(&shell, &args[..], &c_env_vars[..]).unwrap();
    Ok(())
}
