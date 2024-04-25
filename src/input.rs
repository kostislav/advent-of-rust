use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct InputData {
    path: String,
}

impl InputData {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn lines(&self) -> impl Iterator<Item=String> {
        let reader = BufReader::new(File::open(&self.path).unwrap());
        reader.lines().map(|it| it.unwrap())
    }
}

pub trait IteratorParsing: Iterator<Item=String> {
    fn parse_yolo<T>(self) -> impl Iterator<Item=T>
        where Self: Sized, T: FromStr, <T as FromStr>::Err: Debug {
        self.map(|item| item.parse::<T>().unwrap())
    }
}

impl<T> IteratorParsing for T where T: Iterator<Item=String> {}