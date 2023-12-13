use colored::Color;
use std::io::{stdout, Write};

pub const CLEAR_LINE: &str = "\r";
pub trait LoadlessIteratorExt<'a>: Sized {
    fn loadless(self) -> LoadlessIterator<'a, Self>;
}

pub struct LoadlessIterator<'a, Iterator> {
    iter: Iterator,
    idx: usize,
    size: usize,
    prog_ch: char,
    prog_color: Option<Color>,
    /// Array representing characters that wrap a loader. [0] is the starting wrapper, [1] is the ending wrapper.
    wrap_ch: [char; 2],
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
        return LoadlessIterator {
            iter,
            idx: 0,
            size,
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
        let output = format!(
            "{CLEAR_LINE}{}{}{}{}{}{}{}{}",
            &wrap_color,
            self.wrap_ch[0],
            &prog_color,
            self.prog_ch.to_string().repeat(self.idx),
            " ".to_string().repeat(self.size - self.idx),
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
