use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;
use std::hash::Hash;
use std::iter::{Peekable, successors};
use std::ops::{Index, IndexMut};

use ahash::AHashMap;
use bstr::ByteSlice;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use num::ToPrimitive;

pub struct InputData {
    data: Vec<u8>,
}

impl InputData {
    pub fn from_file(path: &str) -> Self {
        let data = fs::read(path).unwrap();
        Self { data }
    }

    pub fn from_string(data: &str) -> Self {
        Self { data: unindent(data).into_bytes() }
    }

    pub fn lines(&self) -> impl Iterator<Item=&[u8]> {
        self.data.lines()
    }

    pub fn lines_as<'a, 'b: 'a, T: ParseYolo<'a>>(&'b self) -> impl Iterator<Item=T> + 'a {
        let mut stream = self.stream();
        std::iter::from_fn(move || {
            if stream.has_next() {
                let line = stream.parse_yolo().unwrap();
                stream.try_consume("\n");
                Some(line)
            } else {
                None
            }
        })
    }

    pub fn stream(&self) -> ParseStream {
        self.data.as_slice().stream()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn raw(&self) -> &[u8] {
        &self.data
    }
}

pub fn unindent(input: &str) -> String {
    let lines = input.lines().collect_vec();
    if lines.len() == 1 {
        lines[0].to_owned()
    } else {
        let num_spaces_to_remove = lines[1].chars().take_while(|&c| c == ' ').count();

        lines[1..lines.len()].iter()
            .map(|line| if line.len() <= num_spaces_to_remove {
                ""
            } else {
                &line[num_spaces_to_remove..]
            })
            .join("\n")
    }
}


pub struct ParseStream<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ParseStream<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub fn parse_yolo<T: ParseYolo<'a>>(&mut self) -> Result<T, ()> {
        self.try_parse(T::parse_from_stream)
    }

    pub fn parse_separated<T: ParseSeparated<'a>>(&mut self, separator: &str) -> Result<T, ()> {
        self.try_parse(|stream| T::parse_from_stream(stream, separator))
    }

    pub fn parse_yololo<T: ParseYolo<'a>>(&mut self) -> T {
        self.parse_yolo().unwrap()
    }

    pub fn try_consume(&mut self, what: &str) -> bool {
        let what_bytes = what.as_bytes();
        if self.bytes[self.position..].starts_with(what_bytes) {
            self.position += what_bytes.len();
            true
        } else {
            false
        }
    }

    pub fn try_parse<T, F: Fn(&mut ParseStream<'a>) -> Result<T, ()>>(&mut self, parser: F) -> Result<T, ()> {
        let snapshot = self.position;
        let result = parser(self);
        if result.is_err() {
            self.position = snapshot;
        }
        result
    }

    pub fn expect(&mut self, pattern: &str) -> Result<(), ()> {
        if self.try_consume(pattern) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn fold_while<T, P: Fn(u8) -> bool, F: Fn(T, u8) -> T>(&mut self, initial: T, predicate: P, f: F) -> Result<T, ()> {
        let mut acc: T = initial;
        let orig_position = self.position;
        while self.has_next() {
            let c = self.bytes[self.position];
            if predicate(c) {
                self.position += 1;
                acc = f(acc, c);
            } else {
                break;
            }
        }
        if self.position == orig_position {
            Err(())
        } else {
            Ok(acc)
        }
    }

    pub fn slice_while<P: Fn(u8) -> bool>(&mut self, predicate: P) -> &'a [u8] {
        let start = self.position;
        while self.has_next() {
            let c = self.bytes[self.position];
            if predicate(c) {
                self.position += 1;
            } else {
                break;
            }
        }
        &self.bytes[start..self.position]
    }

    pub fn parse_iter<'b: 'a, T: ParseYolo<'a> + 'a>(mut self, separator: &'b str) -> impl Iterator<Item=T> + 'a {
        successors(
            Some(self.parse_yolo().unwrap()),
            move |_| if self.try_consume(separator) && self.has_next() {
                Some(self.parse_yolo().unwrap())
            } else {
                None
            },
        )
    }

    pub fn parse_iter_right_aligned<'b: 'a, T: ParseYolo<'a> + 'b>(&'b mut self) -> impl Iterator<Item=T> + 'a {
        while self.try_consume(" ") {}
        successors(
            Some(self.parse_yolo().unwrap()),
            |_| if self.try_consume(" ") {
                while self.try_consume(" ") {}
                Some(self.parse_yolo().unwrap())
            } else {
                None
            },
        )
    }

    pub fn drop_until(&mut self, separator: &str) {
        let separator_bytes = separator.as_bytes();
        self.position = self.bytes[self.position..].find(separator_bytes).unwrap() + separator_bytes.len()
    }

    pub fn parse_header<T: ParseYolo<'a>>(&mut self) -> T {
        let header = self.parse_yolo().unwrap();
        self.expect("\n\n").unwrap();
        header
    }

    pub fn has_next(&self) -> bool {
        self.position < self.bytes.len()
    }

    pub fn next(&mut self) -> Result<u8, ()> {
        if let Some(&result) = self.bytes.get(self.position) {
            self.position += 1;
            Ok(result)
        } else {
            Err(())
        }
    }

    pub fn peek(&self) -> Result<u8, ()> {
        self.bytes.get(self.position).copied().ok_or(())
    }
}

pub trait ParseYolo<'a> {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Result<Self, ()> where Self: Sized;
}

impl ParseYolo<'_> for u64 {
    fn parse_from_stream(stream: &mut ParseStream) -> Result<Self, ()> {
        Ok(
            stream.fold_while(
                0,
                |c| c.is_ascii_digit(),
                |acc, c| acc * 10 + (c - b'0') as u64,
            )?
        )
    }
}

impl ParseYolo<'_> for usize {
    fn parse_from_stream(stream: &mut ParseStream) -> Result<Self, ()> {
        Ok(stream.parse_yolo::<u64>()? as usize)
    }
}

impl ParseYolo<'_> for u32 {
    fn parse_from_stream(stream: &mut ParseStream) -> Result<Self, ()> {
        Ok(stream.parse_yolo::<u64>()? as u32)
    }
}

impl ParseYolo<'_> for i64 {
    fn parse_from_stream(stream: &mut ParseStream) -> Result<Self, ()> {
        let negative = stream.try_consume("-");
        let value = stream.parse_yolo::<u64>()? as i64;
        Ok(if negative { -value } else { value })
    }
}

impl ParseYolo<'_> for isize {
    fn parse_from_stream(stream: &mut ParseStream) -> Result<Self, ()> {
        Ok(stream.parse_yolo::<i64>()? as isize)
    }
}

impl ParseYolo<'_> for i32 {
    fn parse_from_stream(stream: &mut ParseStream) -> Result<Self, ()> {
        Ok(stream.parse_yolo::<i64>()? as i32)
    }
}


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Word<'a>(&'a [u8]);

impl<'a> Word<'a> {
    pub fn from_str(value: &'a str) -> Self {
        Self(value.as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }
}

impl<'a> ParseYolo<'a> for Word<'a> {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Result<Self, ()> {
        Ok(Self(stream.slice_while(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase())))
    }
}

impl ParseYolo<'_> for char {
    fn parse_from_stream(stream: &mut ParseStream<'_>) -> Result<Self, ()> {
        Ok(stream.next()? as char)
    }
}

impl<'a, T: ParseYolo<'a>> ParseYolo<'a> for Box<T> {
    fn parse_from_stream(stream: &mut ParseStream<'a>) -> Result<Self, ()> where Self: Sized {
        Ok(Box::new(stream.parse_yolo()?))
    }
}


pub trait ParseSeparated<'a> {
    fn parse_from_stream(stream: &mut ParseStream<'a>, separator: &str) -> Result<Self, ()> where Self: Sized;
}


impl<'a, T: Default + Copy + ParseYolo<'a>, const N: usize> ParseSeparated<'a> for [T; N] {
    fn parse_from_stream(stream: &mut ParseStream<'a>, separator: &str) -> Result<Self, ()> where Self: Sized {
        let mut result = [T::default(); N];
        for i in 0..(N - 1) {
            result[i] = stream.parse_yolo()?;
            stream.expect(separator)?;
        }
        result[N - 1] = stream.parse_yolo()?;
        Ok(result)
    }
}


pub trait U8IteratorExtras<'a>: Iterator<Item=&'a [u8]> where Self: Sized {
    fn map_chunks<T, F: FnMut(ChunkLinesIterator<Peekable<Self>>) -> T>(self, chunk_transformation: F) -> impl Iterator<Item=T> {
        ProcessedChunkIterator::new(self.peekable(), chunk_transformation)
    }
}

impl<'a, I> U8IteratorExtras<'a> for I where I: Iterator<Item=&'a [u8]> {}

struct ProcessedChunkIterator<F, I: Iterator> {
    lines: Peekable<I>,
    chunk_transformation: F,
}

impl<'a, T, I: Iterator<Item=&'a [u8]>, F: FnMut(ChunkLinesIterator<Peekable<I>>) -> T> ProcessedChunkIterator<F, I> {
    pub fn new(lines: Peekable<I>, chunk_transformation: F) -> Self {
        Self { lines, chunk_transformation }
    }
}

impl<'a, T, I: Iterator<Item=&'a [u8]>, F: FnMut(ChunkLinesIterator<Peekable<I>>) -> T> Iterator for ProcessedChunkIterator<F, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.peek().is_some() {
            Some((self.chunk_transformation)(ChunkLinesIterator { big_iterator: &mut self.lines }))
        } else {
            None
        }
    }
}

pub struct ChunkLinesIterator<'a, I> {
    big_iterator: &'a mut I,
}

impl<'a, 'b, I: Iterator<Item=&'b [u8]>> Iterator for ChunkLinesIterator<'a, I> {
    type Item = &'b [u8];

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
        #[allow(clippy::needless_range_loop)]
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

    fn enumerate_as_second(self) -> impl Iterator<Item=(T, usize)> {
        self.enumerate().map(|(i, value)| (value, i))
    }
}

impl<I, T> IteratorExtras<T> for I where I: Iterator<Item=T> {}

pub trait U8SliceExtras<'a> {
    fn stream(&self) -> ParseStream<'a>;
}

impl<'a> U8SliceExtras<'a> for &'a [u8] {
    fn stream(&self) -> ParseStream<'a> {
        ParseStream::new(self)
    }
}


pub struct WrappingArray<T, const N: usize> {
    values: [T; N],
    base: usize,
}

impl<T, const N: usize> WrappingArray<T, N> {
    pub fn new<F: Fn() -> T>(element_initializer: F) -> Self {
        Self {
            values: std::array::from_fn(|_| element_initializer()),
            base: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values[self.base..N].iter()
            .chain(self.values[0..self.base].iter())
    }

    pub fn rotate_left(&mut self) {
        self.base = (self.base + 1) % N;
    }
}

impl<T: Default + Copy, const N: usize> Default for WrappingArray<T, N> {
    fn default() -> Self {
        WrappingArray {
            values: [T::default(); N],
            base: 0,
        }
    }
}

impl<T, I: ToPrimitive, const N: usize> Index<I> for WrappingArray<T, N> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.values[(self.base as isize + index.to_isize().unwrap()).rem_euclid(N as isize) as usize]
    }
}

impl<T, I: ToPrimitive, const N: usize> IndexMut<I> for WrappingArray<T, N> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.values[(self.base as isize + index.to_isize().unwrap()).rem_euclid(N as isize) as usize]
    }
}


pub trait CopyableIteratorExtras<T: Copy>: Iterator<Item=T> where Self: Sized {
    fn peek_around_window(mut self) -> impl Iterator<Item=(Option<T>, T, Option<T>)> {
        let mut values = WrappingArray::<Option<T>, 3>::default();
        values[0] = self.next();
        values[1] = self.next();
        std::iter::from_fn(move ||
            if let Some(current) = values[0] {
                let result = (values[-1], current, values[1]);
                values.rotate_left();
                values[1] = self.next();
                Some(result)
            } else {
                None
            }
        )
    }
}


impl<I, T: Copy> CopyableIteratorExtras<T> for I where I: Iterator<Item=T> {}


pub trait OrdIteratorExtras<T: Ord>: Iterator<Item=T> where Self: Sized {
    fn largest_n(self, n: usize) -> impl Iterator<Item=T> {
        let mut largest = BinaryHeap::with_capacity(n + 1);
        for item in self {
            largest.push(Reverse(item));
            if largest.len() > n {
                largest.pop();
            }
        }
        largest.into_iter().map(|it| it.0)
    }

    fn median(self) -> T {
        crate::collections::median(self.collect_vec())
    }

    fn min_yolo(self) -> T {
        self.min().unwrap()
    }

    fn min_max_yolo(self) -> (T, T) {
        if let MinMax(min, max) = self.minmax() {
            (min, max)
        } else {
            panic!("Not enough elements")
        }
    }
}

impl<I, T: Ord> OrdIteratorExtras<T> for I where I: Iterator<Item=T> {}


pub trait VecDequeExtras<T> {
    fn pop_front_while<F: Fn(&T) -> bool>(&mut self, predicate: F);
}

impl<T> VecDequeExtras<T> for VecDeque<T> {
    fn pop_front_while<F: Fn(&T) -> bool>(&mut self, predicate: F) {
        while let Some(front) = self.front() {
            if predicate(front) {
                self.pop_front();
            } else {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unindent_works() {
        let unindented = unindent("
            aa
            bb

            cc
        ");

        assert_eq!(unindented, "aa\nbb\n\ncc\n")
    }
}