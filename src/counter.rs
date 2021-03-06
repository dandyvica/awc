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
                let tmp = line_length(&line);

                if tmp > stats.max_line {
                    stats.max_line = tmp;
                }
            }

            // calculate min_line if any
            if opt.min_line {
                let tmp = line_length(&line);

                if tmp < stats.min_line {
                    stats.min_line = tmp;
                }
            }

            // clear buffer to not accumulate data
            line.clear();
        }

        // sanity check for empty files
        if stats.min_line == std::u64::MAX {
            stats.min_line = 0;
        }        

        Ok(stats)
    }
}

// calculate line length
#[cfg(target_family = "unix")]
fn line_length(line: &str) -> u64 {
    // check last char
    if line.is_empty() {
        return 0;
    }

    // other calculate length
    let l = line.chars().count() as u64;
    let last_char = line.chars().last().unwrap();

    if last_char == '\n' {
        return l - 1;
    } else {
        return l;
    }
}

#[cfg(target_family = "windows")]
fn line_length(line: &str) -> u64 {
    // check last char
    if line.is_empty() {
        return 0;
    }

    // other calculate length
    let l = line.chars().count();
    if l == 1 {
        return 1;
    }

    let last_char = line.chars().last().unwrap();
    let before_last_char = line.chars().skip(l - 2).next();

    if last_char == '\n' {
        if before_last_char.unwrap() == '\r' {
            return (l - 2) as u64;
        } else {
            return (l - 1) as u64;
        }
    } else {
        return l as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_utf8() {
        // set options
        let mut options = CliOptions::default();
        options.bytes = true;
        options.chars = true;
        options.words = true;
        options.lines = true;
        options.max_line = true;
        options.min_line = true;

        // sample text
        let stats = Counter::count("tests/utf8.txt", &options);
        assert!(stats.is_ok());

        let stats = stats.unwrap();

        assert_eq!(stats.bytes, 1904);
        assert_eq!(stats.chars, 1000);
        assert_eq!(stats.words, 3);
        assert_eq!(stats.lines, 1);
        assert_eq!(stats.min_line, 1000);
        assert_eq!(stats.max_line, 1000);
    }

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

        // greek text with greek chars
        let stats = Counter::count("tests/odysseus.unix", &options);
        assert!(stats.is_ok());

        let stats = stats.unwrap();

        assert_eq!(stats.bytes, 3776);
        assert_eq!(stats.chars, 1935);
        assert_eq!(stats.words, 304);
        assert_eq!(stats.lines, 44);
        assert_eq!(stats.max_line, 50);
        assert_eq!(stats.min_line, 1);
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
