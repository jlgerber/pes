use peslib::prelude::*;
use peslib::jsys::*;

// must bring the StructOpt trait into scope
use structopt::StructOpt;

mod cli_opts;
use cli_opts::*;

fn env_cmd(subcmd: SubCmds, global_debug: bool) {
    
    match subcmd {
        SubCmds::Env{ output: Some(output), debug, ..} => println!("User supplied output: {:?}. debug mode? {}", output, debug || global_debug),
        SubCmds::Env {constraints,output: None, debug, ..} => println!("user supplied constraints: {:?}. Debug mode? {}", constraints, debug || global_debug),
        _ => panic!("SubCmd expected to be SubCmds::Env variant"),
        
    };
}


fn shell_cmd(subcmd: SubCmds, global_debug: bool) {
    
    match subcmd {
        SubCmds::Shell{ lockfile: Some(lockfile),debug, ..} => println!("User supplied lockfile: {:?}. debug mode? {}", lockfile, debug || global_debug),
        SubCmds::Shell {constraints, lockfile: None, debug, ..} => println!("user supplied constraints: {:?}. Debug Mode? {}", constraints, debug || global_debug),
        _ => panic!("SubCmd expected to be SubCmds::Shell variant"),
        
    };
}

fn main() {
    // let clean_env = JsysCleanEnv::new();
    // for env in clean_env.base_env() {
    //     println!("{:?}", env);
    // }
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let Opt{ debug, subcmd } = opt;
    match subcmd {
        SubCmds::Env { .. } => env_cmd(subcmd, debug),
        SubCmds::Shell { .. } => shell_cmd(subcmd, debug),
    };
}