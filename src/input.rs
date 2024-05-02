use std::cell::RefCell;
use std::fmt::Debug;
use std::fs;
use std::iter::Peekable;
use std::rc::Rc;
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

pub struct SplittingIterator<I, F> {
    iterator: Rc<RefCell<I>>,
    predicate: Rc<F>,
}

pub struct SplittingSubIterator<I, F> {
    iterator: Rc<RefCell<I>>,
    predicate: Rc<F>,
}

pub trait IteratorExtras: Iterator {
    fn split_on<F: Fn(&Self::Item) -> bool>(self, predicate: F) -> SplittingIterator<Self, F>
        where Self: Sized {
        SplittingIterator { iterator: Rc::from(RefCell::from(self)), predicate: Rc::from(predicate) }
    }
}

impl<I, F, T> Iterator for SplittingSubIterator<I, F>
    where I: Iterator<Item=T>, F: Fn(&T) -> bool {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterator.borrow_mut().next();
        if let Some(next) = next {
            if (self.predicate)(&next) {
                None
            } else {
                Some(next)
            }
        } else {
            None
        }
    }
}

impl<I, F, T> Iterator for SplittingIterator<Peekable<I>, F>
    where I: Iterator<Item=T>, F: Fn(&T) -> bool {
    type Item = SplittingSubIterator<Peekable<I>, F>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator.borrow_mut().peek().is_some() {
            Some(SplittingSubIterator { iterator: self.iterator.clone(), predicate: self.predicate.clone() })
        } else {
            None
        }
    }
}

impl<I> IteratorExtras for I where I: Iterator {}