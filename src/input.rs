use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::iter::Peekable;
use std::str::FromStr;
use ahash::AHashMap;

use unindent::unindent;

pub trait ParseYolo<'a> {
    fn parse(s: &'a str) -> Self;
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

    pub fn whole(&self) -> &str {
        self.data.as_str().trim()
    }
}


impl<'a, T> ParseYolo<'a> for T where T: FromStr, <T as FromStr>::Err: Debug {
    fn parse(s: &'a str) -> Self {
        Self::from_str(s).unwrap()
    }
}

pub trait IteratorYoloParsing<'a>: Iterator<Item=&'a str> {
    fn parse_yolo<T>(self) -> impl Iterator<Item=T>
        where Self: Sized, T: ParseYolo<'a> {
        self.map(|item| T::parse(item))
    }
}

impl<'a, T> IteratorYoloParsing<'a> for T where T: Iterator<Item=&'a str> {}


pub trait StrIteratorExtras<'a>: Iterator<Item=&'a str> where Self: Sized {
    fn map_chunks<T, F: FnMut(ChunkLinesIterator<Peekable<Self>>) -> T>(self, chunk_transformation: F) -> impl Iterator<Item=T> {
        ProcessedChunkIterator::new(self.peekable(), chunk_transformation)
    }
}

impl<'a, I> StrIteratorExtras<'a> for I where I: Iterator<Item=&'a str> {}

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

pub trait HashableIteratorExtras<T: Eq + Hash>: Iterator<Item=T> where Self: Sized {
    fn histogram(self) -> AHashMap<T, usize> {
        let mut result = AHashMap::new();

        for item in self {
            *result.entry(item).or_insert(0) += 1;
        }
        result
    }
}

impl<I, T: Eq + Hash> HashableIteratorExtras<T> for I where I: Iterator<Item=T> {}

pub trait DefaultIteratorExtras<T: Default + Copy>: Iterator<Item=T> where Self: Sized {
    fn collect_array<const N: usize>(mut self) -> [T; N] {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.next().unwrap();
        }
        if self.next().is_some() {
            panic!("Too many items in iterator")
        }
        result
    }
}

impl<I, T: Default + Copy> DefaultIteratorExtras<T> for I where I: Iterator<Item=T> {}

pub trait IteratorExtras<T>: Iterator<Item=T> where Self: Sized {
    fn only_element(mut self) -> T {
        let result = self.next().unwrap();
        if self.next().is_some() {
            panic!("Too many items in iterator")
        }
        result
    }
}

impl<I, T> IteratorExtras<T> for I where I: Iterator<Item=T> {}