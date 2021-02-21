use std::env;
use std::path::PathBuf;
use std::io::{self, BufReader};

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
    let files = get_files(&args);

    // if no files this means we want to read from stdin
    if files.is_empty() {
        let reader = BufReader::new(io::stdin());
        match Counter::read_file(reader, &options) {
            Ok(stats) => stats.print_results(&options, ""),
            Err(e) => eprintln!("error '{}' when counting from stdin", e),
        };
        return Ok(());
    }

    // calculate stats for each file found
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
    args
    .iter()
    .skip(1)    // first element is the executable file name, so skip it
    //.map(|x| x as &str) // transforms to a vector of references
    .filter(|&x| !x.starts_with("-"))   // and only keep non-flags (not starting with "-")
    .map(|x| PathBuf::from(x))
    .collect()
}

#[cfg(target_family = "windows")]
fn get_files<'a>(args: &'a [String]) -> Vec<&'a str> {
    // fetch glob because on Windows, no file name expansion is made. So if we pass '*.jpg', we only
    // get this
    let glob = args
    .iter()
    .skip(1)    // first element is the executable file name, so skip it
    .map(|x| x as &str) // transforms to a vector of references
    .filter(|&x| !x.starts_with("-"))   // and only keep non-flags (not starting with "-")
    .collect();

    debug_assert!(glob.len() == 1);

    let v: Vec<String> = Vec::new();

    for entry in glob(glob[1]).unwrap() {
        match entry {
            Ok(path) => v.push(path),
    
            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => eprintln!("error {} trying to get file names using pattern {}", e, glob[1]),
        }
    }

    v
}
