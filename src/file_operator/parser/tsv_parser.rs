
use super::parser::Parser;
use std::collections::HashMap;

pub struct TSVParser;

impl Parser for TSVParser {
    fn parse(&self, string: &str) -> (i8, HashMap<String, i8>) {

        let mut word_count = HashMap::new();
        let splits: Vec<_> = string.trim_right().split('\t').collect();
        let y: i8 = splits[0].to_string().parse().unwrap();
        let words: Vec<&str> = splits[1].split(' ').collect();
        for word in words {
            if !word_count.contains_key(word) {
                word_count.insert(word.to_string(), 1);
            } else {
                *word_count.get_mut(word).unwrap() += 1;
            }
        }

        (y, word_count)
    }
}
