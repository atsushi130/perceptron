
mod file_operator;
mod learner;

use file_operator::{ FileOperator, TSVParser };
use learner::{ Learner, Perceptron };

fn main() {
    let mut perceptron = Perceptron::new(FileOperator::new("../data/titles-en-train.small.labeled".to_string(), TSVParser));
    perceptron.learn();
    perceptron.learned();
}
