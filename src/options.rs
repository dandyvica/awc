use std::char;

// Options as interpreted or inferred from the command line
#[derive(Debug, Default)]
pub struct CliOptions<'a> {
    // request to output the number of bytes
    pub bytes: bool,

    // request to output the number of chars
    pub chars: bool,

    // request to output the number of words (blank-separated)
    pub words: bool,

    // request to output the number of lines
    pub lines: bool,

    // request to output the maximum length line
    pub max_line: bool,

    // request to output the minimum length line
    pub min_line: bool,

    // whether file is zipped
    pub zipped: bool,

    // here we'll keep all other arguments considered as files
    pub files: Vec<&'a str>,
}

impl<'a> CliOptions<'a> {
    // manage arguments from the command line
    pub fn check_args(args: &'a [String]) -> CliOptions<'a> {
        let mut options = CliOptions::default();

        // check flags
        for arg in args {
            match arg.as_str() {
                // manage first single flags
                "-b" | "--bytes" => options.bytes = true,
                "-c" | "--chars" => options.chars = true,
                "-w" | "--words" => options.words = true,
                "-l" | "--lines" => options.lines = true,
                "-z" | "--zip" => options.zipped = true,
                "-L" | "--max" => options.max_line = true,
                "-M" | "--min" => options.min_line = true,
                "-a" | "--all" => {
                    options.bytes = true;
                    options.chars = true;
                    options.words = true;
                    options.lines = true;
                    options.max_line = true;
                    options.min_line = true;
                }
                // now check for combined flags. E.g: -bcw
                &_ => {
                    if arg.as_str().starts_with("-") {
                        // explode into individual chars
                        let maybe_flags: Vec<char> = arg.as_str().chars().collect();
                        for c in maybe_flags {
                            CliOptions::maybe_flags(c, &mut options);
                        }
                    }
                }
            }
        }

        options
    }

    // set individual flag
    fn maybe_flags(char_flag: char, options: &mut CliOptions) {
        match char_flag {
            'b' => options.bytes = true,
            'c' => options.chars = true,
            'w' => options.words = true,
            'l' => options.lines = true,
            'z' => options.zipped = true,
            'L' => options.max_line = true,
            'M' => options.min_line = true,
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_args() {
        let args = vec!["-b".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.bytes);

        let args = vec!["-c".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.chars);

        let args = vec!["-w".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.words);

        let args = vec!["-l".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.lines);

        let args = vec!["-L".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.max_line);

        let args = vec!["-M".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.min_line);

        let args = vec!["-bcwlLM".to_string()];
        let options = CliOptions::check_args(&args);
        assert!(options.bytes);
        assert!(options.chars);
        assert!(options.words);
        assert!(options.lines);
        assert!(options.max_line);
        assert!(options.min_line);
    }
}
