//! utils command
use std::{
    cell::RefCell, collections::HashMap, env, ffi::CString, path::PathBuf, rc::Rc, str::FromStr,
};

use itertools::join;
use log::{debug, info, trace};
use nix::unistd::execve;
use peslib::{
    constants::MANIFEST_NAME, jsys::*, parser::parse_consuming_all_paths_with_provider, prelude::*,
    PluginMgr, SelectedDependencies, SemanticVersion
};

/// validate that a distribution exists
pub fn check_distribution<D: AsRef<str>>(plugin_mgr: &PluginMgr, dist: D) -> Result<bool, PesError> {
    let dist = dist.as_ref();
    
    for repo in PackageRepository::from_plugin(&plugin_mgr)? {
        if repo.has_distribution(dist)? {
            return Ok(true);
        }
    }
    Err(PesError::DistributionNotFound(dist.to_string()))
}

/// audit manifest file for issues
pub fn audit_manifest_file<M: Into<PathBuf>>(manifest: M) -> Result<bool, PesError> {
    let manifest = Manifest::from_path_unchecked(manifest)?;
    manifest.validate()?;

    Ok(true)
}

/// Given the CWD, find the manifest, assuming you are calling from within a package, and then
/// perform an audit.
pub fn audit_manifest_for_current_location() -> Result<bool, PesError> {
    let manifest = find_manifest()?;
    audit_manifest_file(manifest)
}

/// find the manifest
pub fn find_manifest() -> Result<PathBuf, PesError> {
    let mut cwd = env::current_dir()?;

    info!("searching for manifest in {:?}", &cwd);

    loop {
        cwd.push(MANIFEST_NAME);
        if cwd.exists() {
            info!("find_manifest() - Found manifest: {:?}", &cwd);
            return Ok(cwd);
        }
        // pop off manifest name and parent levbel
        if cwd.pop() == false {
            break;
        };
        if cwd.pop() == false {
            break;
        };
        trace!("loop. current cwd: {:?}", cwd);
    }
    Err(PesError::ManifestNotFound(env::current_dir()?))
}

/// Initialize the log given the provided level
pub fn init_log(level: &str) {
    match level {
        "trace" | "debug" | "info" | "warn" | "error" | "critical" => {
            std::env::set_var("RUST_LOG", level)
        }
        _ => std::env::set_var("RUST_LOG", "warn"),
    }
    pretty_env_logger::init();
}

/// launch an interactive shell given a solution
pub fn launch_shell(
    plugin_mgr: &PluginMgr,
    solution: SelectedDependencies<String, SemanticVersion>,
) -> Result<(), PesError> {
    // construct a list of repositories
    let repos = PackageRepository::from_plugin(plugin_mgr)?;
    // iterate through the solve. For each package version, find it in a repository and store it
    // in a hashmap
    fn build_manifest_hashmap(solution: SelectedDependencies<String, SemanticVersion>, repos: &Vec<PackageRepository>) 
    -> Result< HashMap::<String, (PathBuf, Manifest)>, PesError> {
        let mut manifests = HashMap::<String, (PathBuf, Manifest)>::new();
        // define a var to hold a list of distributions for which we cannot find manifests
        let mut missing_manifests = Vec::new();
        // solution is a HashMap of (package,version) pairs
        for (package, version) in solution.iter() {
            // search through repositories for registered manifests
            let mut manifest_path = None;
            for repo in repos {
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
            Err(PesError::MissingManifests(missing_manifests))
        } else {
            Ok(manifests)
        }
    }
    let manifests = build_manifest_hashmap(solution, &repos)?;
    
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
    for (k, v) in env_vars {
        let existing_paths = v.inner();
        // construct required format for execve
        let existing_paths = format!("{}={}", k, join(existing_paths, ":"));
        info!("{}", &existing_paths);
        //let existing_paths : std::ffi::CString = existing_paths.bytes().into();
        let existing_paths =
            std::ffi::CString::new(&existing_paths[..]).expect("unable to convert to cstring");
        c_env_vars.push(existing_paths);
    }

    // identify shell
    let env_cmd = CString::new("/usr/bin/env").unwrap();
    let shell = std::env::var("SHELL").unwrap_or("bash".to_string());
    let shell = Shell::from_str(shell.as_str())?;
    let args = shell.env_args();

    // call execve with environment vec
    execve(&env_cmd, &args[..], &c_env_vars[..]).unwrap();
    Ok(())
}

// store the args needed to launch a shell for the shell subcommand
enum Shell {
    Bash,
    Tcsh,
    Sh,
}

impl FromStr for Shell {
    type Err = PesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" | "/bin/bash" => Ok(Self::Bash),
            "tcsh" | "-csh" => Ok(Self::Tcsh),
            "sh" => Ok(Self::Sh),
            _ => Err(PesError::ParsingFailure(format!("Shell::from_str {}", s))),
        }
    }
}

impl Shell {
    fn env_args(&self) -> Vec<CString> {
        match self {
            Self::Bash => vec![
                CString::new("-i").expect("Unable to convert -i into CString"),
                CString::new("bash").expect("unable to convert shell str into CString"),
                CString::new("--noprofile").expect("unable to convert --noprofile into CString"),
                CString::new("--norc").expect("unable to convert --norc into CString"),
            ],
            Self::Tcsh => vec![
                CString::new("-i").expect("Unable to convert -i into CString"),
                CString::new("tcsh").expect("unable to convert shell str into CString"),
                CString::new("-f").expect("unable to convert -f into CString"),
            ],
            _ => panic!("shell not supported"),
        }
    }
}
