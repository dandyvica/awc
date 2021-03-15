// generate UTF-8 to UFT-32 file samples
use std::char;

use clap::{App, Arg};

enum Encoding {
    UTF8,
    UTF16,
    UTF32,
}

const LAST_CHAR_INDEX: u32 = 1000;

fn main() {
    let matches = App::new("Generate UTD-8 to UTF-32 sample files, with optional BOM marker")
        .version("0.1")
        .author("Alain Viguier dandyvica@gmail.com")
        .arg(
            Arg::new("encoding")
                .long_about("Encoding of the result file")
                .short('e')
                .long("encoding")
                .required(true)
                .possible_values(&["utf8", "utf16", "utf32"])
                .takes_value(true),
        )
        .arg(
            Arg::new("bom")
                .long_about("Use BOM marker")
                .short('b')
                .long("bom")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    // get encoding & bom
    let encoding = match matches.value_of("encoding").unwrap() {
        "uft8" => Encoding::UTF8,
        "uft16" => Encoding::UTF16,
        "uft32" => Encoding::UTF32,
        _ => Encoding::UTF8,
    };

    let bom = matches.is_present("bom");

    // write 1000 chars
    match encoding {
        Encoding::UTF8 => write_utf8(),
        Encoding::UTF16 => write_utf16(),
        _ => todo!("todo"),
    }
}

// write utf-8 chars
fn write_utf8() {
    for i in 32..LAST_CHAR_INDEX + 32 {
        match char::from_u32(i) {
            Some(c) => print!("{}", c),
            None => (),
        }
    }
}

// write utf-16 chars
fn write_utf16() {
    for i in 32..LAST_CHAR_INDEX + 32 {
        match char::from_u32(i) {
            Some(c) => {
                let mut buffer = [0; 2];
                let utf16_char = c.encode_utf16(&mut buffer);
                print!("{}", utf16_char[0]);
                print!("{}", utf16_char[1]);
            }
            None => (),
        }
    }
}
