use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::hash::Hash;
use std::iter::{Peekable, successors};
use std::ops::{Index, IndexMut};

use ahash::AHashMap;
use bstr::ByteSlice;
use itertools::Itertools;
use num::ToPrimitive;
use unindent::unindent;

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

    pub fn lines_as<T: ParseYolo>(&self) -> impl Iterator<Item=T> + '_ {
        let mut stream = self.stream();
        std::iter::from_fn(move || {
            if stream.has_next() {
                let line = stream.parse_yolo();
                stream.try_consume("\n");
                Some(line)
            } else {
                None
            }
        })
    }

    pub fn stream(&self) -> ParseStream<'_> {
        self.data.as_slice().stream()
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

    pub fn parse_yolo<T: ParseYolo>(&mut self) -> T {
        T::parse_from_stream(self)
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

    pub fn expect(&mut self, pattern: &str) {
        if !self.try_consume(pattern) {
            panic!("Pattern not found")
        }
    }

    pub fn fold_while<T, P: Fn(u8) -> bool, F: Fn(T, u8) -> T>(&mut self, initial: T, predicate: P, f: F) -> T {
        let mut acc: T = initial;
        while self.has_next() {
            let c = self.bytes[self.position];
            if predicate(c) {
                self.position += 1;
                acc = f(acc, c);
            } else {
                return acc;
            }
        }
        return acc;
    }

    pub fn parse_array<T: Default + Copy + ParseYolo, const N: usize>(&mut self, delimiter: &str) -> [T; N] {
        let mut result = [T::default(); N];
        for i in 0..(N - 1) {
            result[i] = self.parse_yolo();
            self.expect(delimiter);
        }
        result[N - 1] = self.parse_yolo();
        result
    }

    pub fn parse_iter<'b: 'a, T: ParseYolo + 'b>(&'b mut self, separator: &'b str) -> impl Iterator<Item=T> + 'a {
        successors(
            Some(self.parse_yolo()),
            |_| if self.try_consume(separator) {
                Some(self.parse_yolo())
            } else {
                None
            },
        )
    }

    pub fn parse_iter_right_aligned<'b: 'a, T: ParseYolo + 'b>(&'b mut self) -> impl Iterator<Item=T> + 'a {
        while self.try_consume(" ") {}
        successors(
            Some(self.parse_yolo()),
            |_| if self.try_consume(" ") {
                while self.try_consume(" ") {}
                Some(self.parse_yolo())
            } else {
                None
            },
        )
    }

    pub fn has_next(&self) -> bool {
        self.position < self.bytes.len()
    }
}

pub trait ParseYolo {
    fn parse_from_stream(stream: &mut ParseStream) -> Self;
}

impl ParseYolo for u64 {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        stream.fold_while(
            0,
            |c| c >= b'0' && c <= b'9',
            |acc, c| acc * 10 + (c - b'0' as u8) as u64,
        )
    }
}

impl ParseYolo for usize {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        stream.parse_yolo::<u64>() as usize
    }
}


impl ParseYolo for i64 {
    fn parse_from_stream(stream: &mut ParseStream) -> Self {
        let negative = stream.try_consume("-");
        let value = stream.parse_yolo::<u64>() as i64;
        if negative { -value } else { value }
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
}

impl<I, T: Ord> OrdIteratorExtras<T> for I where I: Iterator<Item=T> {}