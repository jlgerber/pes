#![recursion_limit = "256"]

use structopt::StructOpt;
use users::{get_current_uid, get_user_by_uid};

use peslib::prelude::*;

mod cli_opts;
mod utils;

use cli_opts::*;
use utils::{
    audit_manifest_file, audit_manifest_for_current_location, init_log, launch_shell,
    perform_solve, present_solve_results, solve_for_distribution_and_target,
};

fn env_cmd(subcmd: SubCmds) -> Result<(), PesError> {
    let plugin_mgr = PluginMgr::new()?;
    match subcmd {
        // Here the user has specified a specific distribution (eg foo-1.0.1) and a target
        SubCmds::Env {
            distribution: Some(dist),
            target,
            output,
            ..
        } => {
            let results =
                solve_for_distribution_and_target(&plugin_mgr, dist.as_str(), target.as_str())?;
            let results = results
                .iter()
                .filter(|x| x.0 != "ROOT_REQUEST")
                .collect::<Vec<_>>();
            if let Some(output) = output {
                // get the user from the current process
                let user = get_user_by_uid(get_current_uid()).unwrap();
                let user = user.name();
                // reconstruct the request string from args
                let request = std::env::args().collect::<Vec<_>>().join(" ");
                // create a new lockfile
                let mut lockfile = LockFile::new(request, user.to_string_lossy());
                // add the target distribution to the lockfile (first?)
                lockfile.add_dist(target.as_str(), dist.as_str())?;
                // add remaining distributions to lockfile
                for result in results {
                    let dist = format!("{}-{}", result.0, result.1);
                    lockfile.add_dist(target.as_str(), dist.as_str())?;
                }
                lockfile.to_file(output, true)?;
            } else {
                println!("{}", dist.as_str());
                for result in results {
                    println!("{}-{}", result.0, result.1);
                }
            }
        }
        // here the user has specified a set of constraints instead of a specific distribution. This is
        // used to generate a solve for runtime
        SubCmds::Env {
            constraints,
            output: None,
            ..
        } => {
            // perform the solve given the constraints, and filter out the ROOT_REQUEST from the
            // results, as we dont want to present that to the end user
            let (distmap, _results) = perform_solve(&plugin_mgr, constraints)?;
            //let results = results.iter().filter(|x| x.0 != "ROOT_REQUEST").collect::<Vec<_>>();
            // print the results
            present_solve_results(distmap);
            // for result in results {
            //     println!("{}-{}", result.0, result.1);
            // }
        }
        // here the user has specified a set of constraints as well as an output lockfile. Rather
        // than display the results, we write them to a file.
        SubCmds::Env {
            constraints,
            output: Some(output),
            ..
        } => {
            // perform the solve given the constraints
            let (distmap, results) = perform_solve(&plugin_mgr, constraints)?;

            // filter out the fake request we build to pass the solver, which only takes a
            // single distribution as input
            // we should not need to do this now, as we are deleting this in perform_solve
            //let results = results.iter().filter(|x| x.0 != "ROOT_REQUEST").collect::<Vec<_>>();

            // calculate the request string
            let request = std::env::args().collect::<Vec<_>>().join(" ");
            // extract the user's login from the current process
            let user = get_user_by_uid(get_current_uid()).unwrap();
            let user = user.name();
            // create a new lockfile
            let mut lockfile = LockFile::new(request, user.to_string_lossy());
            for result in results {
                let dist = format!("{}-{}", result.0, result.1);
                lockfile.add_dist("run", dist.as_str())?;
            }
            present_solve_results(distmap);
            lockfile.to_file(output, true)?;
        }
        _ => println!("Unsupported argument combination for pes env"),
    };
    Ok(())
}

fn shell_cmd(subcmd: SubCmds) -> Result<(), PesError> {
    let plugin_mgr = PluginMgr::new()?;
    match subcmd {
        SubCmds::Shell {
            lockfile: Some(lockfile),
            ..
        } => {
            let lockfile = LockFile::from_file(lockfile)?;
            let solution = lockfile.selected_dependencies_for("run")?;
            launch_shell(&plugin_mgr, solution)
        }
        SubCmds::Shell {
            constraints,
            lockfile: None,
            ..
        } => {
            let (distmap, solution) = perform_solve(&plugin_mgr, constraints)?;
            present_solve_results(distmap);
            launch_shell(&plugin_mgr, solution)
        }
        _ => panic!("SubCmd expected to be SubCmds::Shell variant"),
    }?;
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
                .map(|(c, x)| {
                    if c > 0 {
                        format!("\tand {}", x)
                    } else {
                        format!("{}", x)
                    }
                })
                .collect::<Vec<_>>();
            for part in parts {
                eprintln!("{}", part);
            }
            eprintln!("");
        }
        Err(e) => {
            eprintln!("\nError\n");
            eprintln!("{}", e);
            eprintln!("");
        }
    };
}

fn _main() -> Result<(), PesError> {
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:?}", opt);
    }

    let Opt {
        log_level, subcmd, ..
    } = opt;
    init_log(&log_level);
    match subcmd {
        SubCmds::Audit {
            manifest: Some(manifest),
        } => {
            audit_manifest_file(manifest)?;
        }
        SubCmds::Audit { manifest: None } => {
            audit_manifest_for_current_location()?;
        }
        SubCmds::Env { .. } => env_cmd(subcmd)?,
        SubCmds::Shell { .. } => shell_cmd(subcmd)?,
    };
    Ok(())
}
