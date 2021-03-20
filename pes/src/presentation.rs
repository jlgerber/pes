use crate::{aliases::{
        DistPathMap, SolveRefResult
    },
};
use peslib::{PluginMgr, Manifest, PesError};
use prettytable::{color, format, Attr, Cell, Row, Table};
use std::{
    collections::HashSet,
    path::{
        PathBuf,
    },
};

/// Present distributions from all of the repositories identified by the repo_finder plugin in a tidy table, 
pub fn present_distributions(plugin_mgr: &PluginMgr) -> Result<(), PesError> {
    // setup the table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(Row::new(vec![
        Cell::new("DISTRIBUTION")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
        Cell::new("LOCATION")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
    ]));
    // initialize the DistPathMap
    let dist_path_map = plugin_mgr.get_distpathmap()?;//get_distpathmap(&plugin_mgr)?;
    // retrieve the distributions and paths from the map and store in a vector
    // so that we may sort it (and lets sort it)
    let mut dists = dist_path_map.iter().collect::<Vec<_>>();
    dists.sort_by(|a,b| a.0.cmp(&b.0));
    // add distributions and paths into table
    dists.iter().for_each(|(dist, path)| {
        table.add_row(Row::new(vec![
        Cell::new(dist.as_str())
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new(path),
        ]));}
    );
    // presentation time
    eprintln!("");
    table.printstd();
    Ok(())
}


/// print the provided solver results as a pretty table of distribution, paths
#[allow(dead_code)]
pub fn present_solve_results(dpmap: DistPathMap) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(Row::new(vec![
        Cell::new("DISTRIBUTION")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
        Cell::new("LOCATION")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
    ]));

    for (dist, version) in dpmap.iter() {
        table.add_row(Row::new(vec![
            Cell::new(dist.as_str())
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new(version),
        ]));
    }
    eprintln!("");
    table.printstd();
}

pub enum PresentationInput<'a> {
    Constraints(Vec<&'a str>),
    Target{distribution: &'a str, target: &'a str}
}

/// Given a starting request and a solution, present a tree view of 
/// the dependencies
pub fn present_solve_results_tree<'a> (
     // vector of one or more requested packages as fed to the solver
     requirements: PresentationInput<'a>,
     // see alises.rs
     solve: &SolveRefResult,
     // used to find the manifest given the distribution (from solve above)
     plugin_mgr: &PluginMgr,
) -> Result<(), PesError>  {
    // recursive function
    fn present<'aa> (
        // vector of one or more requested packages as fed to the solver
        packages: Vec<&'aa str>,
        // the optional target to solve for. This will default to "run". Furthermore, 
        // we are only interested in the target when the depth is 0, as we solve for 
        // transitive dependencies using the "run" target, regardless of the initial target
        // request
        target: &str,
        // see aliases.rs
        solve: &SolveRefResult, 
        // keep track of visited packages
        mut memo: HashSet<String>, 
        // the number of tabstops to present at
        depth: usize,
        plugin_mgr: &PluginMgr,
    ) -> HashSet<String> {
        let indent = String::from_utf8(vec![b' '; depth*3]).expect("unable to construct string");
        for pkg in packages {
            // extract package from distribution if provided with distribution
            let pkg = pkg.split('-').next().unwrap();
            // get the distribution from the solve results
            let version = solve.1.get(pkg).expect(format!("distribution {} not available in solve results  {:#?}",pkg, solve.1).as_str());
            let distribution = format!("{}-{}", pkg, version);
            if memo.contains(&distribution) {
                // if it is in the memo, simply present with a ":" in front 
                println!("{}:{}", &indent, &distribution)
            } else {
                // otherwise, add it to the memo, print it out, and invoke present recursively 
                //(which means opening the manifest and building constraints), capturing memo from the return value
                memo.insert(distribution.to_string());
                println!("{}{}", &indent, &distribution);
                // get manifest and list of constraints
                let distpath = solve.0.get(&distribution).expect(format!("unable to get path to distribution {} from DistPathMap", &distribution).as_str());
                let  distpath = PathBuf::from(distpath.as_str());
                // get the manifest from the distribution path via the plugin manager
                let manifest_path = plugin_mgr.manifest_path_from_distribution(distpath);
                // open the manifest and get the constraints for the target
                let manifest = Manifest::from_path(manifest_path).expect("couldnt construct manifest from path");
                // construct the package vec from the requires
                let requires = manifest.get_requires(target).unwrap_or_else(|_| Vec::new());
                let constraints = requires.iter().map(|r| { r.name.to_string() }).collect::<Vec<_>>();
                let constraints_ref = constraints.iter().map(AsRef::as_ref).collect::<Vec<_>>();

                memo = present(
                    constraints_ref,
                    target,
                    solve, 
                    memo,
                    depth+1,
                    plugin_mgr
                );
            }
        }
        memo
    }

    fn package_from_dist(dist: &str) -> &str {
        dist
            .splitn(1, "-")
            .next()
            .unwrap_or_else(|| dist)
    }
    // I didnt end up needing this map. May in the future though...
    /*
    fn new_package_dist_map(dist_path_map: &DistPathMap) -> PackageDistMap {
        // construct a PackageDistMap from a DistPathMap
        let mut package_dist_map = PackageDistMap::new();
        dist_path_map
            .keys()
            // construct a tuple of (package, distribution)
            .map(|dist| 
                {
                    let pkg = dist
                    .split("-")
                    .next()
                    .unwrap_or_else(|| dist);
                    (pkg, dist)
                }
            ).for_each(|(pkg, dist)| {package_dist_map.insert(pkg.to_string(), dist.to_string()); });

        package_dist_map 
    }
    */
    // create the memo
    let  memo = HashSet::new();
    // create the package_distribution_map used to associate the package name with the distribution
    //let pkg_dist_map = new_package_dist_map(&solve.0);

    // construct constraints from requirements
    let (constraints, target) = match requirements {
        PresentationInput::Constraints(constraints) => {
            let c = constraints.iter().map(|v| package_from_dist(v).to_string()).collect::<Vec<_>>();
            (c, "run")
        },
        PresentationInput::Target{distribution, target} => {
            // split the package from the distribution
            //let package = package_from_dist(distribution);
            // look up the distribution path from the solve.0
            let distpath = solve.0.get(distribution).expect(format!("unable to get path to distribution {} from DistPathMap", distribution).as_str());
            let  distpath = PathBuf::from(distpath.as_str());
           // let mut distpath = distpath.to_path_buf();
            // get the manifest from the distribution path via the plugin manager
            let manifest_path = plugin_mgr.manifest_path_from_distribution(distpath);
            // open the manifest and get the constraints for the target
            let manifest = Manifest::from_path(manifest_path).expect("couldnt construct manifest from path");
            // construct the package vec from the requires
            let requires = manifest.get_requires(target).expect("unable to get requires for target");
            let constraints = requires.iter().map(|r| { r.name.to_string() }).collect::<Vec<_>>();
            (constraints, target)
        }
    };
    // convert constraints from Vec<String> to Vec<&str>
    let constraints_ref = constraints.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let _ = present(
        constraints_ref,
        target,
        solve, 
        memo,
        0,
        plugin_mgr
    );


    Ok(())
}