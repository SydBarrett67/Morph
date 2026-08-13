mod grammar;
mod tokenizer;

use tokenizer::Tokenizer;


use std::fs;
fn main() {
    let source: String = fs::read_to_string("morph/test.mr").unwrap();

    let mut tokenizer: Tokenizer = Tokenizer::new(source);

    let tokens: Vec<grammar::Token> = tokenizer.tokenize();

    println!("{:?}", tokens);

}