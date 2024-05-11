use std::fmt::Debug;
use std::fs;
use std::iter::Peekable;
use std::str::FromStr;

use unindent::unindent;

pub trait ParseYolo {
    fn parse(s: &str) -> Self;
}

pub struct InputData {
    data: String,
}

impl InputData {
    pub fn from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).unwrap();
        Self { data }
    }

    pub fn from_string(data: &str) -> Self {
        Self { data: unindent(data) }
    }

    pub fn lines(&self) -> impl Iterator<Item=&str> {
        self.data.lines()
    }
}


impl<T> ParseYolo for T where T: FromStr, <T as FromStr>::Err: Debug {
    fn parse(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

pub trait IteratorYoloParsing<'a>: Iterator<Item=&'a str> {
    fn parse_yolo<T>(self) -> impl Iterator<Item=T>
        where Self: Sized, T: ParseYolo {
        self.map(|item| T::parse(item))
    }
}

impl<'a, T> IteratorYoloParsing<'a> for T where T: Iterator<Item=&'a str> {}


pub trait IteratorExtras<'a>: Iterator<Item=&'a str> where Self: Sized {
    fn map_chunks<T, F: FnMut(ChunkLinesIterator<Peekable<Self>>) -> T>(self, chunk_transformation: F) -> impl Iterator<Item=T> {
        ProcessedChunkIterator::new(self.peekable(), chunk_transformation)
    }
}

impl<'a, I> IteratorExtras<'a> for I where I: Iterator<Item=&'a str> {}

struct ProcessedChunkIterator<F, I: Iterator> {
    lines: Peekable<I>,
    chunk_transformation: F,
}

impl<'a, T, I: Iterator<Item=&'a str>, F: FnMut(ChunkLinesIterator<Peekable<I>>) -> T> ProcessedChunkIterator<F, I> {
    pub fn new(lines: Peekable<I>, chunk_transformation: F) -> Self {
        Self { lines, chunk_transformation }
    }
}

impl<'a, T, I: Iterator<Item=&'a str>, F: FnMut(ChunkLinesIterator<Peekable<I>>) -> T> Iterator for ProcessedChunkIterator<F, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.peek().is_some() {
            Some((self.chunk_transformation)( ChunkLinesIterator { big_iterator: &mut self.lines }))
        } else {
            None
        }
    }
}

pub struct ChunkLinesIterator<'a, I> {
    big_iterator: &'a mut I,
}

impl<'a, 'b, I: Iterator<Item=&'b str>> Iterator for ChunkLinesIterator<'a, I> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        self.big_iterator.next().filter(|line| !line.is_empty())
    }
}
