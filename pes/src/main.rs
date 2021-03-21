#![recursion_limit = "256"]

use structopt::StructOpt;
use users::{get_current_uid, get_user_by_uid};
use log::info;
use peslib::prelude::*;

mod cli_opts;
pub mod utils;
mod presentation;
pub mod aliases;

use cli_opts::*;
pub use utils::{
    audit_manifest_file, audit_manifest_for_current_location, init_log, launch_shell,
    check_distribution
};

use presentation::{
    PresentationInput,
    Presenter,
    DistributionFilter,
};

// handle the dist subcommand
fn dist_cmd(subcmd: SubCmds) -> Result<(), PesError> {
    let plugin_mgr = PluginMgr::new()?;
    let presenter = Presenter::new(&plugin_mgr);

    match subcmd {
        SubCmds::Dist{ check, dist, list_dists } => {
            if list_dists {
                presenter.distributions(DistributionFilter::All)?;
            } else if check {
                match dist {
                    Some(ref dist) => {
                        if check_distribution(&plugin_mgr, dist)? {
                            
                            println!("\n\tDistribution: {} is valid\n", dist);
                        } else {
                            
                            println!("\nWARNING:\n\n\tDistribution: {} does not exist\n", dist);
                        }
                },
                    None => return Err(PesError::CliArgError("Must supply a distribution when using --check".into()))
                } 
            } else {
                match dist {
                    Some(ref dist) =>  presenter.distributions(DistributionFilter::Package(dist))?,
                    None => return Err(PesError::CliArgError("Must supply a distribution".into()))
                }
               
            }
            
            Ok(())
        }
        _ => panic!("dist_cmd received unexpected input")
    }
}

fn env_cmd(subcmd: SubCmds) -> Result<(), PesError> {
    let plugin_mgr = PluginMgr::new()?;
    let presenter = Presenter::new(&plugin_mgr);

    match subcmd {
        // Here the user has specified a specific distribution (eg foo-1.0.1) and a target
        SubCmds::Env {
            distribution: Some(dist),
            target,
            output,
            include_pre,
            ..
        } => {
            let (distmap, results) =
                perform_solve_for_distribution_and_target(&plugin_mgr, dist.as_str(), target.as_str(), include_pre)?;
            
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

                presenter.solve_results_tree(
                    PresentationInput::Target{distribution: dist.as_str(), target: target.as_str()},
                    &(&distmap, &results),
                ).expect("present_solve_resutls_tree failed");
                
            }
        }
        // here the user has specified a set of constraints instead of a specific distribution. This is
        // used to generate a solve for runtime
        SubCmds::Env {
            constraints,
            include_pre,
            output: None,
            ..
        } => {
            // perform the solve given the constraints, and filter out the ROOT_REQUEST from the
            // results, as we dont want to present that to the end user
            let constraints: Vec<&str> = constraints.iter().map(AsRef::as_ref).collect();
            info!("perfoming solve with constraints: {:?}", &constraints);
           
            let (distmap, results) = perform_solve(&plugin_mgr, &constraints, include_pre)?;
            info!("solve returned: {:#?}", &results);

            presenter.solve_results_tree(
                PresentationInput::Constraints(constraints),
                &(&distmap, &results),
            ).expect("present_solve_resutls_tree failed");

        }
        // here the user has specified a set of constraints as well as an output lockfile. Rather
        // than display the results, we write them to a file.
        SubCmds::Env {
            constraints,
            include_pre,
            output: Some(output),
            ..
        } => {
            let constraints: Vec<&str> = constraints.iter().map(AsRef::as_ref).collect();
            // perform the solve given the constraints
           
            let (distmap, results) = perform_solve(&plugin_mgr, &constraints, include_pre)?;

            // calculate the request string
            let request = std::env::args().collect::<Vec<_>>().join(" ");
            // extract the user's login from the current process
            let user = get_user_by_uid(get_current_uid()).unwrap();
            let user = user.name();
            // create a new lockfile
            let mut lockfile = LockFile::new(request, user.to_string_lossy());
            for result in &results {
                let dist = format!("{}-{}", result.0, result.1);
                lockfile.add_dist("run", dist.as_str())?;
            }

            presenter.solve_results_tree(
                PresentationInput::Constraints(constraints),
                &(&distmap, &results),
            ).expect("present_solve_resutls_tree failed");

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
            include_pre,
            lockfile: None,
            ..
        } => {
            let presenter = Presenter::new(&plugin_mgr);

            let constraints: Vec<&str> = constraints.iter().map(AsRef::as_ref).collect();
            
            let (distmap, solution) = perform_solve(&plugin_mgr, &constraints, include_pre)?;

            presenter.solve_results_tree(
                PresentationInput::Constraints(constraints),
                &(&distmap, &solution),
            ).expect("present_solve_resutls_tree failed");

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
            eprintln!("\t{}", e);
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
        SubCmds::Dist { .. } => dist_cmd(subcmd)?,
        SubCmds::Audit { manifest: None } => {
            audit_manifest_for_current_location()?;
        }
        SubCmds::Env { .. } => env_cmd(subcmd)?,
        SubCmds::Shell { .. } => shell_cmd(subcmd)?,
    };
    Ok(())
}
