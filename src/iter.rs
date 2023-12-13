use colored::Color;
use std::io::{stdout, Write};

pub const CLEAR_LINE: &str = "\r";
pub trait LoadlessIterExt<'a>: Sized {
    fn loadless(self) -> LoadlessIter<'a, Self>;
    /// Build a `loadless` iterator with a custom write target.
    /// ```
    /// let vec = vec![0, 10, 100];
    /// let write_target: Vec<u8> = Vec::new();
    /// for int in vec.iter().loadless_set_target(write_target) { /* */ }
    /// println!("{}", String::from_utf8(write_target).unwrap());
    /// ```
    /// `write_target` now recieves all
    fn loadless_with_target(self, target: &'a mut dyn std::io::Write) -> LoadlessIter<'a, Self>;
}

pub struct LoadlessIter<'a, Iter> {
    iter: Iter,
    idx: usize,
    size: usize,
    prog_ch: char,
    prog_color: Option<Color>,
    wrap_ch: Option<char>,
    wrap_color: Option<Color>,
    target: WriteTarget<'a>,
}

enum WriteTarget<'a> {
    Stdout,
    Custom(&'a mut dyn Write),
}

impl<'a, Iter: Iterator> LoadlessIterExt<'a> for Iter {
    fn loadless(self) -> LoadlessIter<'a, Self> {
        return LoadlessIter::new(self, WriteTarget::Stdout);
    }
    fn loadless_with_target(self, target: &'a mut dyn std::io::Write) -> LoadlessIter<'a, Self> {
        return LoadlessIter::new(self, WriteTarget::Custom(target));
    }
}

impl<'a, Iter: Iterator> LoadlessIter<'a, Iter> {
    pub fn new(iter: Iter, target: WriteTarget<'a>) -> Self {
        // TODO! Figure out a better way to get the estimated size of an iterator.
        let size_hint = iter.size_hint();
        let size: usize;
        if let Some(val) = size_hint.1 {
            size = val;
        } else {
            size = size_hint.0;
        }
        // TODO! ^
        return LoadlessIter {
            iter,
            idx: 0,
            remain: 10,
            total: 10,
            target: target,
        };
    }
    fn write_ln(&self) -> Result<(), std::io::Error> {
        match self.target {
            WriteTarget::Stdout => write!(stdout(), "{CLEAR_LINE}"),
            WriteTarget::Custom(wt) => {
                write!(wt, "{CLEAR_LINE}")
            }
        }
    }
}

impl<'a, Iter: Iterator> Iterator for LoadlessIter<'a, Iter> {
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        write!(
            "{}[{}{}] {}%",
            CLEAR_LINE,
            "▓".repeat(self.idx),
            " ".repeat(self.remain),
            ((self.idx as f64 / self.total as f64) * 100 as f64) as usize,
        )
        .expect("Write target of LoadlessIterator is invalid.");
        if self.remain > 0 {
            self.remain -= 1;
        } else {
            write!(target, "\n");
        }
        self.idx += 1;
        return self.iter.next();
    }
}
