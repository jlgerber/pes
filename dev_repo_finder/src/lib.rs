use pes_core::RepoFinderService;
use std::path::PathBuf;
use std::env;

const REPO_VARNAME: &str = "PES_REPO_PATH";
#[no_mangle]
pub fn new_finder_service() -> Box<dyn RepoFinderService> {
    Box::new(DevRepoFinder::new())
}

pub struct DevRepoFinder;

impl DevRepoFinder {
    fn new() -> DevRepoFinder {
        DevRepoFinder
    }
}

impl RepoFinderService for DevRepoFinder {
    // TODO: change this to return a Result. This will mean moving PesError into pes_interace 
    // TODO: and perhaps renaming pes_core to pes_base or pes_core
    fn find_repo(&self) -> Vec<PathBuf> {

        let repo_path = 
        // first we try and read from the environment variable 
        env::var(REPO_VARNAME)
            // next we map over the results 
            .map(|x| 
                // we take the inner value and split it by the colon
                x.split(":")
                // then we convert each substring into a PathBuf
                 .map(|p| PathBuf::from(p))
                 // and collect the whole thing in a Vec<PathBuf>
                 .collect::<Vec<_>>()
            )
            // of course, the latter course of action is only along the happy path. What if the repo is not found in 
            // the environment?
            .unwrap_or_else(|_| {
                // first lets look up the homedir for the current user
                let mut path = dirs::home_dir()
                    // if looking up the homedir fails, we will need to build something ourselves.
                    .ok_or_else(|| {
                        // lets get the current user and outright panic if the current user cannot be determined
                        let username = users::get_current_username().expect("unable to get current username from environment");
                        let username = username.to_string_lossy();
                        // Now lets build a resonable path for osx and linux
                        // TODO: determine windows path
                        #[cfg(target_os = "macos")]
                        let  path = PathBuf::from(format!("/Users/{}", &username));
                        #[cfg(target_os = "linux")]
                        let  path = PathBuf::from(format!("/home/{}", &username));
                        path
                    }).expect("unable to build path to homedir");
                // ok lets add a reasonably brittle path to the repo test fixtures, assuming that they are in 
                // the user's home directory under a specific path. This is convenient for my testing but
                // I should change all of this before releasing to the wider world.
                path.push("src/rust/pes/test_fixtures/repo_test");
                vec![path]
                
            } );
            repo_path
    }
}