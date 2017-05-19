
use super::super::Learner;
use std::collections::HashMap;
use file_operator::{ FileOperator, Parser };

pub struct Perceptron<T: Parser> {
    word_model: HashMap<String, i8>,
    file_operator: FileOperator<T>
}

impl<T: Parser> Learner for Perceptron<T> {

    fn learn(&mut self) {

        for line in self.file_operator.get_lines() {
            let (y, key_values) = self.file_operator.parse(&line);
            let yy = self.predict_one(&self.word_model);

            if yy == y {
                continue
            }

            match yy {
                1  => {
                    for (word, _) in &key_values {
                        *self.word_model.entry(word.to_string()).or_insert(0) += -1;
                    }
                },
                -1 => {
                    for (word, _) in &key_values {
                        *self.word_model.entry(word.to_string()).or_insert(0) += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

impl<T: Parser> Perceptron<T> {

    pub fn new(file_operator: FileOperator<T>) -> Self {
        Perceptron {
            word_model: HashMap::new(),
            file_operator
        }
    }

    pub fn learned(&self) {
        for (key, value) in self.word_model.iter() {
            println!("{}: {}", key, value);
        }
    }

    fn predict_one(&self, key_values: &HashMap<String, i8>) -> i8 {

        let x = key_values.iter().map( |(word, _)|
            match self.word_model.contains_key(word) {
                true => self.word_model[word] * key_values[word],
                false => 0,
            }
        ).fold(0, |sum, x| sum + x);

        self.sign((x as f64))
    }

    fn sign(&self, x: f64) -> i8 {
        match x < 0. {
            true  => -1,
            false => 1
        }
    }
}
