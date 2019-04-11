use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate clap;

mod util;
use util::framework::{Option, Stat};

// useful macro for debugging at run time depending on the first argument
macro_rules! debug {
    ($debug:expr, $($arg:tt)*) => ({
        if $debug {
            eprintln!($($arg)*);
        }
    })
}

fn main() {
    // manage arguments
    let matches = clap_app!(ewc =>
        (version: "0.1")
        (author: "Dandyvica <dandyvica@gmail.com>")
        (about: "Counts byte, chars, lines, words in a text file")
        (@arg FILES: +required "File name to get stats from")
        (@arg bytes: -c --bytes "Print the byte counts")
        (@arg chars: -m --chars "Print the character counts")
        (@arg words: -w --words "Print the newline counts")
        (@arg lines: -l --lines "Print the newline counts")
        (@arg maxline: -L --max "Print the maximum line length in chars")
        (@arg debug: -d --debug "Print debug information")
    )
    .get_matches();

    // gather all options into a single struct
    let mut opt = Option::new();

    let debug = matches.is_present("debug");
    opt.bytes = matches.is_present("bytes");
    opt.chars = matches.is_present("chars");
    opt.words = matches.is_present("words");
    opt.lines = matches.is_present("lines");
    opt.max_line = matches.is_present("maxline");

    debug!(debug, "{:?}", opt);

    // new stat struct
    let mut stats = Stat::new();

    // get file names
    let files = matches.value_of("FILES").unwrap();
    debug!(debug, "files: {:?}", files);

    // open file for reading line by line
    let f = File::open(files).unwrap();
    let mut file = BufReader::new(&f);

    // string buffer
    let mut line = String::new();

    loop {
        // read next line
        let nb_read = match file.read_line(&mut line) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error <e> reading line, {}", e);
                0
            }
        };

        // did we meet EOF?
        if line.len() == 0 {
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
        }

        // calculate max_line if any
        if opt.max_line {
            let tmp = line.chars().count() as u64 - 1;
            if tmp > stats.max_line {
                stats.max_line = tmp;
            }
        }        

        // count chars if any
        if opt.words {
            stats.words += line.split_whitespace().count() as u64;
        }

        // clear buffer to not accumulate data
        line.clear();
    }

    println!("{:?}", stats);
}
