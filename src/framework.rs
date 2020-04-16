// As arguments are related to final statistics, use this to not redefine structs
#[derive(Debug)]
pub struct FrameWork<T> {
    pub bytes: T,
    pub chars: T,
    pub words: T,
    pub lines: T,
    pub max_line: T,
    pub min_line: T,
    pub zipped: T,
}

impl<T: Default> FrameWork<T> {
    pub fn new() -> FrameWork<T> {
        FrameWork {
            bytes: T::default(),
            chars: T::default(),
            words: T::default(),
            lines: T::default(),
            max_line: T::default(),
            min_line: T::default(),
            zipped: T::default(),
        }
    }
}

pub type Options = FrameWork<bool>;
pub type Stats = FrameWork<u64>;

// This is used for displaying the final result
impl Stats {
    pub fn print_results(&self, opt: &Options) {
        if opt.bytes {
            print!("{} ", self.bytes);
        }

        if opt.chars {
            print!("{} ", self.chars);
        }

        if opt.words {
            print!("{} ", self.words);
        }

        if opt.lines {
            print!("{} ", self.lines);
        }

        if opt.min_line {
            print!("{} ", self.min_line);
        }

        if opt.max_line {
            print!("{} ", self.max_line);
        }
        println!("");
    }
}
