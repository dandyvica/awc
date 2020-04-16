use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate clap;

use flate2::read::GzDecoder;

mod framework;
use framework::Options;

mod read;
use read::read_file;

fn main() -> Result<(), std::io::Error> {
    // manage arguments using clap.rs
    let matches = clap_app!(ewc =>
        (version: "0.1")
        (author: "Dandyvica <dandyvica@gmail.com>")
        (about: "Counts byte, chars, lines, words in a text file")
        (@arg file_name: +required "File name to get stats from")
        (@arg bytes: -b --bytes "print the byte counts")
        (@arg chars: -c --chars "print the character counts")
        (@arg words: -w --words "print the word counts")
        (@arg lines: -l --lines "print the newline counts")
        (@arg zip: -z --zip "assume provided file is zipped")
        (@arg maxline: -L --max "print the maximum line length in chars")
        (@arg minline: -M --min "print the minimum line length in chars")
    )
    .get_matches();

    // gather all options into a single struct
    let mut opt = Options::new();

    opt.bytes = matches.is_present("bytes");
    opt.chars = matches.is_present("chars");
    opt.words = matches.is_present("words");
    opt.lines = matches.is_present("lines");
    opt.max_line = matches.is_present("maxline");
    opt.min_line = matches.is_present("minline");
    opt.zipped = matches.is_present("zip");

    // mimic wc behaviour. If no option is given, assume -c -w -l
    if matches.args.len() == 1 {
        opt.bytes = true;
        opt.chars = true;
        opt.words = true;
        opt.lines = true;
    }

    // get file names
    let file_name = matches.value_of("file_name").unwrap();

    // open file for reading line by line
    let file = File::open(file_name)?;

    // just define this variable. In case of a zipped file, create it
    let stats = if opt.zipped {
        let decoder = GzDecoder::new(file);
        let reader = BufReader::new(decoder);
        read_file(reader, &opt)?
    } else {
        let reader = BufReader::new(file);
        read_file(reader, &opt)?
    };

    stats.print_results(&opt);

    Ok(())
}
