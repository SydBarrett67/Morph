mod grammar;
mod tokenizer;

mod parser;
mod ast;

mod scope;
mod types;

mod interpreter;

use tokenizer::Tokenizer;
use parser::Parser;
use interpreter::Interpreter;


use std::fs;
fn main() {
    let source: String = fs::read_to_string("morph/test.mr").unwrap();

    let mut tokenizer: Tokenizer = Tokenizer::new(source);

    let tokens = tokenizer.tokenize();

    println!("\n\n{:?}", tokens);

    let mut parser: Parser = Parser::new(tokens);

    let ast: Result<ast::Statement, parser::ParserError> = parser.parse();

    println!("\n\n{:?}", ast);

    let ast_extracted = match ast {
        Ok(ok) => ok,
        Err(err) => return 
    };

    let mut interpreter: Interpreter = Interpreter::new(ast_extracted);

    println!("\n\n{:?}", interpreter.interpret());
    println!("\n\n{:?}", interpreter.env);

}