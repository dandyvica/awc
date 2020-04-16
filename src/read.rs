use std::io::{BufRead, Error};

use crate::framework::{Options, Stats};

// calculate and return statistics. Need to buld this function with
// T: BufRead to read either regular or compressed files
pub fn read_file<T: BufRead>(mut reader: T, opt: &Options) -> Result<Stats, Error> {
    // used to acculmate and keep statistics
    let mut stats = Stats::new();

    // initialize value for calculating minimum length
    if opt.min_line {
        stats.min_line = std::u64::MAX;
    }

    // this buffer will receive each line
    let mut line = String::new();

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

        // calculate max_line if any
        if opt.max_line {
            let tmp = line.chars().count() as u64 - 1;
            if tmp > stats.max_line {
                stats.max_line = tmp;
            }
        }

        // calculate min_line if any
        if opt.min_line {
            let tmp = line.chars().count() as u64 - 1;
            if tmp < stats.min_line {
                stats.min_line = tmp;
            }
        }

        // count chars if any
        if opt.words {
            stats.words += line.split_whitespace().count() as u64;
        }

        // clear buffer to not accumulate data
        line.clear();
    }

    Ok(stats)
}
