
use std::io::BufReader;
use std::io::Error as BufError;
use std::io::prelude::BufRead;
use std::fs::File;
use std::error::Error;

use super::Parser;
use std::collections::HashMap;

pub struct FileOperator<T: Parser> {
    path: String,
    parser: T
}

impl<T: Parser> FileOperator<T> {

    pub fn new(path: String, parser: T) -> Self {
        FileOperator {
            path,
            parser
        }
    }

    pub fn get_lines(&self) -> Vec<String> {

        let file   = self.open();
        let reader = BufReader::new(file);

        reader.lines().into_iter().map( |line| {
            match self.get_line(line) {
                Some(string) => string,
                None         => panic!("Value not exist.")
            }
        }).collect()
    }

    pub fn parse(&self, string: &str) -> (i8, HashMap<String, i8>) {
        self.parser.parse(string)
    }

    fn open(&self) -> BufReader<File> {
        match File::open(&self.path) {
            Ok(file)   => BufReader::new(file),
            Err(error) => panic!("Couldn't open {}: {}", &self.path, Error::description(&error))
        }
    }

    fn get_line(&self, line: Result<String, BufError>) -> Option<String> {
        match line {
            Ok(string) => Some(string),
            Err(error) => panic!("Not found line result. Reason: {}", Error::description(&error))
        }
    }
}