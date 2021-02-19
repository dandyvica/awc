use std::env;
use std::io::{self, BufReader};

mod stats;
use stats::Stats;

mod counter;
use counter::Counter;

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
    let files: Vec<&str> = args
        .iter()
        .skip(1)
        .map(|x| x as &str)
        .filter(|&x| !x.starts_with("-"))
        .collect();

    // if no files this means we want to read from stdin
    if files.is_empty() {
        let reader = BufReader::new(io::stdin());
        match Counter::read_file(reader, &options) {
            Ok(stats) => stats.print_results(&options, ""),
            Err(e) => println!("error '{}' when counting from stdin", e),
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
            Err(e) => println!("error '{}' when counting into file {}", e, f),
        };
    }

    // print out total if any
    if files.len() > 1 {
        sum_stats.print_results(&options, "total");
    }

    Ok(())
}
