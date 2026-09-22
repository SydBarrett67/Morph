// Lexer
mod grammar;
mod tokenizer;

// Parser
mod parser;
mod ast;

// Interpreter
mod interpreter;
mod scope;
mod semanticanalyzer;

// Exported structs
use tokenizer::Tokenizer;
use parser::Parser;
use semanticanalyzer::SemanticAnalyzer;
use interpreter::Interpreter;


use std::fs;
fn main() {
    let source = fs::read_to_string("morph/test.mr").unwrap();

    println!("\n\n\n{}", source);

    let mut tokenizer = Tokenizer::new(source);

    let tokens = tokenizer.tokenize();

    // println!("\n\nTokens:\n\n{:?}", tokens);

    // Create AST
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            println!("Parser error: {:?}", err);
            return;
        }
    };

    // Add types
    let semanticanalyzer = SemanticAnalyzer::new(ast.clone());
    let typed_ast = semanticanalyzer.analyze();

    println!("\n\nAST:\n\n{:?}", typed_ast);

    let mut interpreter = Interpreter::new(ast);

    println!("\n\n{:?}", interpreter.interpret());
    //println!("\n\n{:?}", interpreter.env);
}