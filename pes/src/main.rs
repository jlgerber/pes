use peslib::prelude::*;
use peslib::jsys::*;

// must bring the StructOpt trait into scope
use structopt::StructOpt;
use main_error::MainError;

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

            // iterate through package manifests, building environment
            
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