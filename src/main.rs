mod grammar;
mod tokenizer;
mod parser;
mod ast;

use tokenizer::Tokenizer;
use parser::Parser;


use std::fs;
fn main() {
    let source: String = fs::read_to_string("morph/test.mr").unwrap();

    let mut tokenizer: Tokenizer = Tokenizer::new(source);

    let tokens: Vec<grammar::Token> = tokenizer.tokenize();

    println!("{:?}", tokens);

    let mut parser: Parser = Parser::new(tokens);

    let AST = parser.parse();

    println!("{:?}", AST);

}