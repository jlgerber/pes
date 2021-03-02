use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;

use peslib::prelude::*;
use peslib::jsys::*;
use peslib::parser::parse_consuming_all_paths_with_provider;

// must bring the StructOpt trait into scope
use structopt::StructOpt;
//use main_error::MainError;

//use std::fmt;
//use std::error::Error;
// pub struct MainError(Box<dyn Error>);

// impl<E: Into<Box<dyn Error>>> From<E> for MainError {
//     fn from(e: E) -> Self {
//         MainError(e.into())
//     }
// }

// // impl Debug (to satisfy trait bound for main()-Result error reporting), but use Display of wrapped
// // error internally (for nicer output).
// impl fmt::Debug for MainError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         //fmt::Debug::fmt(&self.0, f)?;
//         write!(f, "{}", format!("{}", &self.0));
        
//         let mut source = self.0.source();
//         while let Some(error) = source {
//             write!(f, "\ncaused by: {:#?}", error)?;
//             source = error.source();
//         }
//         Ok(())
//     }
// }


mod cli_opts;
use cli_opts::*;

// setup the solver, adding package repositories
fn setup_solver() -> Result <Solver<String, SemanticVersion>, PesError> 
{
    let repos = PackageRepository::from_env()?;
    let mut solver = Solver::new();
    for repo in repos {
        solver.add_repository(&repo)?;
    }
    Ok(solver)
}

fn env_cmd(subcmd: SubCmds, global_debug: bool) -> Result<(), PesError> {
    
    match subcmd {
        SubCmds::Env{ output: Some(output), debug, ..} => println!("User supplied output: {:?}. debug mode? {}", output, debug || global_debug),
        SubCmds::Env {constraints, output: None, debug, ..} => println!("user supplied constraints: {:?}. Debug mode? {}", constraints, debug || global_debug),
        _ => panic!("SubCmd expected to be SubCmds::Env variant"),
        
    };
    Ok(())
}


fn shell_cmd(subcmd: SubCmds, global_debug: bool) -> Result<(), PesError> {
    
    match subcmd {
        SubCmds::Shell{ lockfile: Some(lockfile),debug, ..} => {
            // eprintln!("User supplied lockfile: {:?}. debug mode? {}", lockfile, debug || global_debug);
            // let request = constraints.iter().map(|x| VersionedPackage::from_str(x.as_str())).collect::<Vec<_>>()?;

            // let mut solver = setup_solver()?;
            // // calculate the solution
            // let solution = solver.solve(request)?;
            // eprintln!("{:#?}", solution);
            todo!()
        },
        SubCmds::Shell {constraints, lockfile: None, debug, ..} => {
            if debug == true { 
                eprintln!("user supplied constraints: {:?}. Debug Mode? {}", constraints, debug || global_debug);
            }
                // construct request
            let request = constraints.iter().map(|x| VersionedPackage::from_str(x.as_str())).collect::<Result<Vec<_>,PesError>>()?;
            let mut solver = setup_solver()?;
            // calculate the solution

            let solution = solver.solve(request)?;
            if debug == true || debug == false { eprintln!("{:#?}", solution); }
            // construct clean environment
            let clean_env = JsysCleanEnv::new().base_env();

            // construct a list of repositories
            let repos = PackageRepository::from_env()?;
            // iterate through the solve. For each package version, find it in a repository and store it
            // in a hashmap
            let mut manifests = HashMap::<String, (PathBuf, Manifest)>::new();
            // list of distributions for which we cannot find manifests
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
                if let Some(path) = manifest_path {
                    let distribution = format!("{}-{}", package, version);
                    let mani = Manifest::from_path(&path)?;
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
            let mut env_vars = HashMap::new();

            // instantiate provider
            let  provider = std::rc::Rc::new(RefCell::new(BasicVarProvider::new()));
            // iterate through package manifests, building environment
            for (name, (root, manifest)) in manifests {
                println!("name: {}", name);
                //println!("{:#?}", manifest);
                {
                    let mut prov = provider.borrow_mut();
                    prov.insert_var("root", root.as_path().display().to_string());
                }
                for (key, value) in manifest.environment() {
                    println!("{} {}", &key, value);
                    let result = parse_consuming_all_paths_with_provider(Rc::clone(&provider), value)?;
                    println!("{:?}", result);
                    {
                        if let Some(val) = env_vars.get_mut(key) {
                            *val += result;
                        } else {    
                            env_vars.insert(key.clone(), result);
                        }
                    }
                }
            }

            println!("OUTPUT VARS");
            for (k,v ) in env_vars {
                println!("{}", k);
                for val in v.inner() {
                    println!("\t{:?}", val);
                }
            }
            // construct environment vec<CString> for execve call
            // identify shell
            // call execve with environment vec
        },
        _ => panic!("SubCmd expected to be SubCmds::Shell variant"),
        
    };
    Ok(())
}

fn main() {
    match _main() {
        Ok(_) => (),
        Err(PesError::NoSolution(msg)) => {
            eprintln!("\nError\n");
            let parts = msg
                        .split("and")
                        .enumerate()
                        .map(|(c, x)| if c > 0 { 
                                format!("\tand {}", x) 
                            } else {
                                format!("{}", x)
                            }
                        )
                        .collect::<Vec<_>>();
            for part in parts {
                eprintln!("{}", part);
            }
            eprintln!("");
        },
        Err(e) => {
            eprintln!("\nError\n");
            eprintln!("{}", e);
            eprintln!("");
        },
    };
}

fn _main() -> Result<(), PesError> {
    // let clean_env = JsysCleanEnv::new();
    // for env in clean_env.base_env() {
    //     println!("{:?}", env);
    // }
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:?}", opt);
    }

    let Opt{ debug, subcmd } = opt;
    match subcmd {
        SubCmds::Env { .. } => env_cmd(subcmd, debug)?,
        SubCmds::Shell { .. } => shell_cmd(subcmd, debug)?,
    };
    Ok(())
}