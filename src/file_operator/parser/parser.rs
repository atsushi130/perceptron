
use std::collections::HashMap;

pub trait Parser {
    fn parse(&self, &str) -> (i8, HashMap<String, i8>);
}