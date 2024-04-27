use std::fmt::Debug;
use std::fs;
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


pub trait IteratorParsingUsingFromStr<'a>: Iterator<Item=&'a str> {
    fn parse_yolo<T>(self) -> impl Iterator<Item=T>
        where Self: Sized, T: FromStr, <T as FromStr>::Err: Debug {
        self.map(|item| item.parse::<T>().unwrap())
    }
}

impl<'a, T> IteratorParsingUsingFromStr<'a> for T where T: Iterator<Item=&'a str> {}


pub trait IteratorYoloParsing<'a>: Iterator<Item=&'a str> {
    fn parse_yolo<T>(self) -> impl Iterator<Item=T>
        where Self: Sized, T: ParseYolo {
        self.map(|item| T::parse(item))
    }
}

impl<'a, T> IteratorYoloParsing<'a> for T where T: Iterator<Item=&'a str> {}