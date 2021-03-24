
use structopt::StructOpt;
use std::path::PathBuf;
use peslib::PesError;
use peslib::PluginMgr;
use peslib::LockFile;
use pes::utils::launch_cmd;
use std::collections::VecDeque;
use std::env;
use std::fs;
use log::{debug, trace};


use anyhow::{Result, anyhow};
#[derive(Debug, StructOpt)]
#[structopt(name = "pes-run", about = "shebang line invokation of executable in pes environment")]
struct Opt {
    /// Specify relative path to cmd from distribution root
    #[structopt(long="pes-cmd",  parse(from_os_str))]
    cmd: PathBuf,

    /// path to pes environment lockfile
    #[structopt(long="pes-lockfile", parse(from_os_str))]
    lockfile: PathBuf,

    /// Name of package which the command resides in
    #[structopt(long="pes-pkg")]
    pkg: String,
}


fn _main() -> Result<()> {
    // get args. first arg should be name of shebang wrapper
    let mut args = std::env::args().collect::<VecDeque<_>>();
    // verify that this is set up correctly
    trace!("args {:?}", args);
    let cmd = args.pop_front().ok_or_else(|| anyhow!("unable to get pes-run from front of args"))?;
    if cmd != "pes-run" {
        return Err(anyhow!(format!("extracted executable from shebang line is not res-run. it is {}", cmd)));
    }
    let wrapper = args.pop_front().ok_or_else(|| anyhow!("unable to get wrapper from front of args"))?;
    
    // read file and construct args from file, parsing non-shebang line and adding in 
    let wrapper_contents = fs::read_to_string(&wrapper)?;
    
    let pieces = wrapper_contents.split('\n').filter(|x| !x.starts_with('#')).collect::<Vec<_>>();
    let mut args: Vec<&str> = vec![&wrapper];
    pieces.into_iter().for_each(|ln| ln.split(" ").for_each(|item| args.push(item)));

    trace!("wrapper args {:?}", &args);

    // any additional args
    let opt = Opt::from_iter(args);
    let Opt{pkg, cmd, lockfile} = opt;
    debug!("executing run_cmd(lockfile: {:?}, cmd: {:?}) for package: {})",&lockfile,  &cmd,  &pkg);
    
    run_cmd(lockfile, cmd)?;
    
    Ok(())
}

fn main() {
    match _main() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error\n\n");
            eprintln!("\t{}", e);
        }
    };
}

// run a command 
fn run_cmd(lockfile: PathBuf, cmd: PathBuf) -> Result<(), PesError> {
    let plugin_mgr = PluginMgr::new()?;
   
    let lockfile = LockFile::from_file(lockfile)?;
    let solution = lockfile.selected_dependencies_for("run")?;
    let cmd_str = cmd.to_string_lossy().to_string();
    launch_cmd(&plugin_mgr, solution, cmd_str.as_str())?;
    
    Ok(())
}

