use crate::grammar::Token;
use crate::grammar::Keyword;
use crate::grammar::Operand;

pub struct Tokenizer {
    source: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }

    pub fn peek(&self, look_ahead: usize) -> Option<char> {
        self.source.get(self.position + look_ahead).copied()
    }

    pub fn consume(&mut self, look_ahead: usize) -> String {
        let word: String = self.source[self.position..self.position + look_ahead]
                .iter()
                .collect();

        self.position += look_ahead;

        word
    }

    pub fn token_from_str(&self, word: &str) -> Token {
        match word {
            "let"   => Token::Keyword(Keyword::LET),
            "+"     => Token::Operand(Operand::PLUS),
            "="     => Token::Operand(Operand::EQUALS),
            ";"     => Token::Semicolon,
            // Number or Identifier
            _       => match word.parse::<i32>() {
                Ok(number)  => Token::Number(number),
                Err(_)      => Token::Identifier(word.to_string()),
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() {

            let current: char = self.source[self.position];
            let mut chars_to_consume: usize = 0;

            match current {
                // Whitespace => Ignore
                c if c.is_whitespace() => { self.position += 1; continue }

                // Number => parse
                c if c.is_ascii_digit() => {
                    while let Some(c) = self.peek(chars_to_consume) {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        chars_to_consume += 1;
                    }
                }

                // Text => parse
                c if c.is_ascii_alphabetic() => {
                    while let Some(c) = self.peek(chars_to_consume) {
                        if !c.is_ascii_alphabetic() && !c.is_ascii_digit() {
                            break;
                        }
                        chars_to_consume += 1;
                    }
                }

                // Operators
                ';' => { chars_to_consume += 1 }
                '+' => { chars_to_consume += 1 }
                '=' => { chars_to_consume += 1 }

                _ => { continue; }
            }

            let word = self.consume(chars_to_consume);

            tokens.push(
                self.token_from_str(&word)
            )
        }

        tokens
    }
}