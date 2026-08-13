mod grammar;
mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let source: String = String::from("let x1 = 10 + 5;");
    let mut tokenizer: Tokenizer = Tokenizer::new(source);

    let tokens = tokenizer.tokenize();

    println!("{:?}", tokens);

}