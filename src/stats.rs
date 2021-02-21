use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;

use crate::options::CliOptions;

// As arguments are related to final statistics, use this to not redefine structs
#[derive(Debug, Default)]
pub struct Stats {
    pub bytes: u64,
    pub chars: u64,
    pub words: u64,
    pub lines: u64,
    pub max_line: u64,
    pub min_line: u64,
}

// This is used for displaying the final result
impl Stats {
    pub fn print_results(&self, opt: &CliOptions, text: &str) {
        if opt.bytes {
            print!("{:8} ", self.bytes);
        }

        if opt.chars {
            print!("{:8} ", self.chars);
        }

        if opt.words {
            print!("{:8} ", self.words);
        }

        if opt.lines {
            print!("{:8} ", self.lines);
        }

        if opt.min_line {
            print!("{:8} ", self.min_line);
        }

        if opt.max_line {
            print!("{:8} ", self.max_line);
        }
        println!("{}", text);
    }
}

/// Sum is used to sum all stats
impl<'a> Sum<&'a Self> for Stats {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::default(), |a, b| Self {
            bytes: a.bytes + b.bytes,
            chars: a.chars + b.chars,
            words: a.words + b.words,
            lines: a.lines + b.lines,
            min_line: a.min_line + b.min_line,
            max_line: a.max_line + b.max_line,
        })
    }
}

/// Used to add 2 stats
impl AddAssign for Stats {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            bytes: self.bytes + other.bytes,
            chars: self.chars + other.chars,
            words: self.words + other.words,
            lines: self.lines + other.lines,
            min_line: self.min_line + other.min_line,
            max_line: self.max_line + other.max_line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut stats1 = Stats::default();
        stats1.bytes = 1;
        stats1.chars = 1;
        stats1.words = 1;
        stats1.lines = 1;
        stats1.max_line = 1;
        stats1.min_line = 1;

        let mut stats2 = Stats::default();
        stats1.bytes = 2;
        stats1.chars = 2;
        stats1.words = 2;
        stats1.lines = 2;
        stats1.max_line = 2;
        stats1.min_line = 2;

        stats2 += stats1;

        assert_eq!(stats2.bytes, 2);
        assert_eq!(stats2.chars, 2);
        assert_eq!(stats2.words, 2);
        assert_eq!(stats2.lines, 2);
        assert_eq!(stats2.max_line, 2);
        assert_eq!(stats2.min_line, 2);
    }
}
