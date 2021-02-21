use std::env;
use std::io::{self, BufRead};
use std::path::PathBuf;

mod stats;
use stats::Stats;

mod counter;
use counter::Counter;

#[cfg(target_family = "windows")]
use glob::glob;

mod options;
use options::CliOptions;

fn main() -> Result<(), std::io::Error> {
    // get args from the command line
    let args: Vec<String> = env::args().collect();
    //println!("args={:?}", args);

    // used to add stats to print out total
    let mut sum_stats = Stats::default();

    // convert arguments into flags
    let options = CliOptions::check_args(&args);

    // get files from arguments
    let mut files = get_files(&args);

    // waiting for stdin
    if files.is_empty() {
        files = io::stdin()
            .lock()
            .lines()
            .filter(|x| x.is_ok())
            .map(|x| PathBuf::from(x.unwrap()))
            .collect();
    }

    // now just coutn for each file found
    for f in &files {
        match Counter::count(f, &options) {
            Ok(stats) => {
                stats.print_results(&options, f);
                sum_stats += stats;
            }
            Err(e) => eprintln!("error '{}' when counting into file {}", e, f.display()),
        };
    }

    // print out total if any
    if files.len() > 1 {
        sum_stats.print_results(&options, "total");
    }

    Ok(())
}

// returns the list of files from command line
#[cfg(target_family = "unix")]
fn get_files<'a>(args: &'a [String]) -> Vec<PathBuf> {
    args.iter()
        .skip(1) // first element is the executable file name, so skip it
        //.map(|x| x as &str) // transforms to a vector of references
        .filter(|&x| !x.starts_with("-")) // and only keep non-flags (not starting with "-")
        .map(|x| PathBuf::from(x))
        .collect()
}

#[cfg(target_family = "windows")]
fn get_files<'a>(args: &'a [String]) -> Vec<PathBuf> {
    // fetch glob because on Windows, no file name expansion is made. So if we pass '*.jpg', we only
    // get this
    let pattern = args
        .iter()
        .skip(1) // first element is the executable file name, so skip it
        .map(|x| x as &str) // transforms to a vector of references
        .find(|&x| !x.starts_with("-")); // and only keep non-flags (not starting with "-")

    // this vector will hold the list of found files
    let v: Vec<PathBuf> = Vec::new();
    if pattern.is_none() {
        return v;
    }

    debug_assert!(pattern.is_some());

    let mut v: Vec<PathBuf> = Vec::new();
    let pattern = pattern.unwrap();

    for entry in glob(pattern).unwrap() {
        match entry {
            Ok(path) => v.push(path),

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => eprintln!(
                "error {} trying to get file names using pattern {}",
                e, pattern
            ),
        }
    }

    // contrary to UNIX where the file name expansion is made before calling awc,
    // on Windows, it's not. In this case, we need to check whether the glob returned
    // some files
    if v.is_empty() {
        eprintln!("no files found with pattern {}", pattern);
        std::process::exit(0);
    }

    v
}
