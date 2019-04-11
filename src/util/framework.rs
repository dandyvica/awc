// As arguments are related to final statistics, use this to not redefine structs
#[derive(Debug)]
pub struct FrameWork<T> {
    pub chars: T,
    pub bytes: T,
    pub words: T,
    pub lines: T,
    pub max_line: T,
}

impl<T: Default> FrameWork<T> {
    pub fn new() -> FrameWork<T> {
        FrameWork {
            bytes: T::default(),
            chars: T::default(),
            words: T::default(),
            lines: T::default(),
            max_line: T::default(),
        }
    }
}

pub type Option = FrameWork<bool>;
pub type Stat = FrameWork<u64>;
