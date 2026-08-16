// Lexer
mod grammar;
mod tokenizer;

// Parser
mod parser;
mod ast;

// Interpreter
mod interpreter;
mod scope;
mod symbol;
mod typechecker;
mod namesolver;

// Exported structs
use tokenizer::Tokenizer;
use parser::Parser;
use interpreter::Interpreter;
use typechecker::{ Type, TypeChecker };
use namesolver::NameSolver;


use std::fs;
fn main() {
    let source: String = fs::read_to_string("morph/test.mr").unwrap();

    println!("\n\n\n{}", source.clone());

    let mut tokenizer: Tokenizer = Tokenizer::new(source);

    let tokens = tokenizer.tokenize();

    println!("\n\nTokens: \n\n{:?}", tokens);

    let mut parser: Parser = Parser::new(tokens);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            println!("Parser error: {:?}", err);
            return;
        }
    };

    println!("\n\nCrude AST:\n\n{:?}", ast);

    let mut namesolver = NameSolver::new(ast);

    println!("\n\nScopes:\n\n{:?}", namesolver.build_scopes());

    let ast_extracted = match ast {
        Ok(_) => _,
        Err(_) => return 
    };

    let mut interpreter: Interpreter = Interpreter::new(ast_extracted);

    println!("\n\n{:?}", interpreter.interpret());
    println!("\n\n{:?}", interpreter.env);

}