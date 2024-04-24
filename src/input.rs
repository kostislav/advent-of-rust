use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct InputData {
    path: String
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

