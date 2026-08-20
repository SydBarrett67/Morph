// Lexer
mod grammar;
mod tokenizer;

// Parser
mod parser;
mod ast;

// Interpreter
// mod interpreter;
// mod scope;
// mod scope;
// mod symbol;
mod semanticanalyzer;

// Exported structs
use tokenizer::Tokenizer;
use parser::Parser;
//use interpreter::Interpreter;
/*
use namesolver::NameSolver;
use typechecker::TypeChecker;
*/

use std::fs;
fn main() {
    let source = fs::read_to_string("morph/test.mr").unwrap();

    println!("\n\n\n{}", source);

    let mut tokenizer = Tokenizer::new(source);

    let tokens = tokenizer.tokenize();

    // println!("\n\nTokens:\n\n{:?}", tokens);

    let mut parser = Parser::new(tokens);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            println!("Parser error: {:?}", err);
            return;
        }
    };

    println!("\n\nCrude AST:\n\n{:?}", ast);


    println!("\n\nTyped AST:\n\n{:?}", ast);

    /*

    let mut interpreter = Interpreter::new(
        ast,
        scopes
    );

    println!("\n\n{:?}", interpreter.interpret());
    println!("\n\n{:?}", interpreter.values);
     */
}