use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct FileInputData {
    path: String,
}

pub trait InputData {
    fn lines(&self) -> impl Iterator<Item=String>;
}


impl FileInputData {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl InputData for FileInputData {
    fn lines(&self) -> impl Iterator<Item=String> {
        let reader = BufReader::new(File::open(&self.path).unwrap());
        reader.lines().map(|it| it.unwrap())
    }
}


pub struct StringInputData {
    data: String,
}

impl StringInputData {
    pub fn new(data: &str) -> Self {
        Self { data: data.to_string() }
    }
}

impl InputData for StringInputData {
    fn lines(&self) -> impl Iterator<Item=String> {
        self.data.lines().map(|it| it.to_string())
    }
}


pub trait IteratorParsing: Iterator<Item=String> {
    fn parse_yolo<T>(self) -> impl Iterator<Item=T>
        where Self: Sized, T: FromStr, <T as FromStr>::Err: Debug {
        self.map(|item| item.parse::<T>().unwrap())
    }
}

impl<T> IteratorParsing for T where T: Iterator<Item=String> {}