mod grammar;
mod tokenizer;
mod parser;
mod ast;
mod interpreter;

use tokenizer::Tokenizer;
use parser::Parser;
use interpreter::Interpreter;


use std::fs;
fn main() {
    let source: String = fs::read_to_string("morph/test.mr").unwrap();

    let mut tokenizer: Tokenizer = Tokenizer::new(source);

    let tokens: Vec<grammar::Token> = tokenizer.tokenize();

    println!("\n\n{:?}", tokens);

    let mut parser: Parser = Parser::new(tokens);

    let ast: Result<ast::Statement, parser::ParserError> = parser.parse();

    println!("\n\n{:?}", ast);

    let mut interpreter: Interpreter = Interpreter::new(Result::unwrap(ast));

    println!("\n\n{:?}", interpreter.interpret());
    println!("\n\n{:?}", interpreter.variables);

}