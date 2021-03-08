use structopt::{StructOpt, clap::ArgGroup};
use std::path::PathBuf;

const DEFAULT_LOG_LEVEL: &str = "warn";
const DEFAULT_TARGET: &str = "run";

#[derive(Debug, StructOpt)]
#[structopt(name = "pes", about = "PES - the Package Environment System command line")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    /// Set the log level (trace, debug, info, warn, error, critical)
    #[structopt(short="l", long="log-level", default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: String,

    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    pub subcmd: SubCmds
}

#[derive(Debug, StructOpt)]
pub enum SubCmds {
    #[structopt(name = "audit")]
    /// Audit the manifest, which may either be provided or found
    Audit {
        /// Provide an explicit path to a manifest. 
        #[structopt(short="m", long="manifest", parse(from_os_str))]
        manifest: Option<PathBuf>,
    },
    #[structopt(name = "env", group = ArgGroup::with_name("env_action").required(true))]
    /// Solve a dependency closure for the provided constraints, or the provided distribution and target
    Env {
        #[structopt(short = "l", long = "lock-file", parse(from_os_str))]
        /// Output solve to a pes lock-file
         output: Option<PathBuf>,

        #[structopt(group="env_action")]
        /// provide a list of constraints
         constraints: Vec<String>,

         #[structopt(short="d", long="distribution", group="env_action")]
         /// Provide a distribution to solve, coupled with an optional target name (defaults to run)
         distribution: Option<String>,

         #[structopt(short="t", long="target", default_value=DEFAULT_TARGET)]
         /// Provide a target to calculate the dependencies for. Used with -d | --distribution
         target: String
    },
    #[structopt(name = "shell", group = ArgGroup::with_name("shell_action").required(true))]
    /// Solve a dependency closure based on supplied package constraints, build an environment,
    /// and launch a subshell
    Shell {
        #[structopt(short="d", long="debug")]
        /// Debug mode
         debug: bool,

        #[structopt(short = "l", long="lock-file", group= "shell_action", parse(from_os_str))]
        /// Provide a pes lock file
         lockfile: Option<PathBuf>,

        #[structopt(group = "shell_action")]
        /// provide a list of constraints
         constraints: Vec<String>,
    }
}