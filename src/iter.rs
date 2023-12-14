use crate::LoadlessIteratorExt;
use colored::Color;
use std::io::{stdout, Write};

const CLEAR_LINE: &str = "\r";

pub struct LoadlessIterator<'a, Iterator> {
    /// Underlying iterator behind the loader abstraction.
    iter: Iterator,
    /// Current index of the progress bar.
    idx: usize,
    /// Total indexes to traverse (likely total number of iterations of the iterator).
    size: usize,
    /// Length of the progress bar (number of total characters).
    prog_len: usize,
    /// Character that gets displayed to increment the progress of the loader.
    prog_ch: char,
    /// Color of the progress character.
    prog_color: Option<Color>,
    /// Array representing characters that wrap a loader. [0] is the starting wrapper, [1] is the ending wrapper.
    wrap_ch: [char; 2],
    /// Color of the progress bar wrapper.
    wrap_color: Option<Color>,
    /// Write target, by default is the Stdout.
    target: WriteTarget<'a>,
}

enum WriteTarget<'a> {
    Stdout,
    Custom(&'a mut dyn Write),
}

impl<'a, Iter: Iterator> LoadlessIteratorExt<'a> for Iter {
    fn loadless(self) -> LoadlessIterator<'a, Self> {
        return LoadlessIterator::default(self);
    }
}

impl<'a, Iter: Iterator> LoadlessIterator<'a, Iter> {
    pub fn default(iter: Iter) -> Self {
        let size;
        // Matches on the iterators estimated upper or lower bounds for the iterator.
        match iter.size_hint().1 {
            Some(s) => size = s,
            None => size = iter.size_hint().0,
        }
        let mut prog_len: usize = size;
        // By default, the progress bar
        if size > 10 {
            prog_len = 10;
        }
        return LoadlessIterator {
            iter,
            idx: 0,
            size,
            prog_len,
            prog_ch: '▓',
            prog_color: None,
            wrap_ch: ['[', ']'],
            wrap_color: None,
            target: WriteTarget::Stdout,
        };
    }

    pub fn write_target(mut self, target: &'a mut dyn Write) -> Self {
        self.target = WriteTarget::Custom(target);
        return self;
    }

    fn write_ln(&mut self) -> Result<(), std::io::Error> {
        let wrap_color: String;
        let prog_color;
        match self.wrap_color {
            Some(color) => wrap_color = color.to_fg_str().into(),
            None => wrap_color = "".to_string(),
        }
        match self.prog_color {
            Some(color) => prog_color = color.to_fg_str().into(),
            None => prog_color = "".to_string(),
        }
        let space_count = self
            .prog_len
            .checked_sub(self.idx / (self.size / self.prog_len));
        let output = format!(
            "{CLEAR_LINE}{}{}{}{}{}{}{}{}",
            &wrap_color,
            self.wrap_ch[0],
            &prog_color,
            self.prog_ch
                .to_string()
                .repeat(self.idx / (self.size / self.prog_len)),
            " ".to_string()
                .repeat(if let Some(space_count) = space_count {
                    space_count as usize
                } else {
                    0 as usize
                }),
            &wrap_color,
            self.wrap_ch[1],
            if self.idx == self.size { "\n" } else { "" }
        );
        match &mut self.target {
            WriteTarget::Stdout => {
                write!(stdout(), "{output}")?;
            }
            WriteTarget::Custom(wt) => {
                write!(wt, "{output}")?;
            }
        }
        self.idx += 1;
        return Ok(());
    }
}

impl<'a, Iter: Iterator> Iterator for LoadlessIterator<'a, Iter> {
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.write_ln();
        return self.iter.next();
    }
}
