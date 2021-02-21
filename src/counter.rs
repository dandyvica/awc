use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::path::Path;

use flate2::read::GzDecoder;

use crate::options::CliOptions;
use crate::stats::Stats;

pub struct Counter;

impl Counter {
    pub fn count<P: AsRef<Path>>(file_name: P, options: &CliOptions) -> Result<Stats, Error> {
        let file = File::open(file_name)?;

        let stats = if options.zipped {
            let decoder = GzDecoder::new(file);
            let reader = BufReader::new(decoder);
            Counter::read_file(reader, &options)?
        } else {
            let reader = BufReader::new(file);
            Counter::read_file(reader, &options)?
        };

        Ok(stats)
    }

    // calculate and return statistics. Need to buld this function with
    // T: BufRead to read either regular or compressed files
    pub fn read_file<T: BufRead>(mut reader: T, opt: &CliOptions) -> Result<Stats, Error> {
        // used to acculmate and keep statistics
        let mut stats = Stats::default();

        // initialize value for calculating minimum length
        if opt.min_line {
            stats.min_line = std::u64::MAX;
        }

        // this buffer will receive each line
        let mut line = String::with_capacity(512);

        loop {
            // read next line
            let nb_read = match reader.read_line(&mut line) {
                Ok(n) => n,
                Err(e) => return Err(e),
            };

            // did we meet EOF?
            if nb_read == 0 {
                break;
            }

            // one more line
            if opt.lines {
                stats.lines += 1;
            }

            // count bytes if any
            if opt.bytes {
                stats.bytes += nb_read as u64;
            }

            // count chars if any
            if opt.chars {
                stats.chars += line.chars().count() as u64;
                //eprint!("{};{};{};{}", nb_read, line.chars().count(), nb_read-line.chars().count(), line);
            }

            // count chars if any
            if opt.words {
                stats.words += line.split_whitespace().count() as u64;
            }

            // calculate max_line if any
            if opt.max_line {
                #[cfg(target_family = "unix")]
                let tmp = line.chars().count() as u64 - 1;
                #[cfg(target_family = "windows")]
                let tmp = line.chars().count() as u64 - 2;

                if tmp > stats.max_line {
                    stats.max_line = tmp;
                }
            }

            // calculate min_line if any
            if opt.min_line {
                #[cfg(target_family = "unix")]
                let tmp = line.chars().count() as u64 - 1;
                #[cfg(target_family = "windows")]
                let tmp = line.chars().count() as u64 - 2;
                1;
                if tmp < stats.min_line {
                    stats.min_line = tmp;
                }
            }

            // clear buffer to not accumulate data
            line.clear();
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_family = "unix")]
    fn count() {
        // set options
        let mut options = CliOptions::default();
        options.bytes = true;
        options.chars = true;
        options.words = true;
        options.lines = true;
        options.max_line = true;
        options.min_line = true;

        // sample text
        let stats = Counter::count("tests/poe.unix", &options);
        assert!(stats.is_ok());

        let stats = stats.unwrap();

        assert_eq!(stats.bytes, 25260);
        assert_eq!(stats.chars, 25258);
        assert_eq!(stats.words, 3969);
        assert_eq!(stats.lines, 887);
        assert_eq!(stats.max_line, 73);
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn count_gzipped() {
        // set options
        let mut options = CliOptions::default();
        options.bytes = true;
        options.chars = true;
        options.words = true;
        options.lines = true;
        options.max_line = true;
        options.min_line = true;
        options.zipped = true;

        // sample text
        let stats = Counter::count("tests/poe.unix.gz", &options);
        assert!(stats.is_ok());

        let stats = stats.unwrap();

        assert_eq!(stats.bytes, 25260);
        assert_eq!(stats.chars, 25258);
        assert_eq!(stats.words, 3969);
        assert_eq!(stats.lines, 887);
        assert_eq!(stats.max_line, 73);
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn count() {
        // set options
        let mut options = CliOptions::default();
        options.bytes = true;
        options.chars = true;
        options.words = true;
        options.lines = true;
        options.max_line = true;
        options.min_line = true;

        // sample text
        let stats = Counter::count("tests/poe.windows", &options);
        assert!(stats.is_ok());

        let stats = stats.unwrap();

        assert_eq!(stats.bytes, 26150);
        assert_eq!(stats.chars, 26146);
        assert_eq!(stats.words, 3969);
        assert_eq!(stats.lines, 887);
        assert_eq!(stats.max_line, 73);
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn count() {
        // set options
        let mut options = CliOptions::default();
        options.bytes = true;
        options.chars = true;
        options.words = true;
        options.lines = true;
        options.max_line = true;
        options.min_line = true;

        // sample text
        let stats = Counter::count("tests/poe.windows.gz", &options);
        assert!(stats.is_ok());

        let stats = stats.unwrap();

        assert_eq!(stats.bytes, 26150);
        assert_eq!(stats.chars, 26146);
        assert_eq!(stats.words, 3969);
        assert_eq!(stats.lines, 887);
        assert_eq!(stats.max_line, 73);
    }
}
