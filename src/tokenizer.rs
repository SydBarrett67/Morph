use crate::grammar::{ TokenInfo, Position, Token, Keyword, Operand };

pub struct Tokenizer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 0,
            column: 0
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
            // Keywords
            "let"   => Token::Keyword(Keyword::LET),

            // Operands
            "+"     => Token::Operand(Operand::PLUS),
            "-"     => Token::Operand(Operand::MINUS),
            "*"     => Token::Operand(Operand::STAR),
            "/"     => Token::Operand(Operand::SLASH),
            "="     => Token::Operand(Operand::EQUALS),

            // Structure
            "("     => Token::OpenParen,
            ")"     => Token::CloseParen,
            ";"     => Token::Semicolon,
            // Number or Identifier
            _       => match word.parse::<i32>() {
                Ok(number)  => Token::Number(number),
                Err(_)      => Token::Identifier(word.to_string()),
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<TokenInfo> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            let current = self.source[self.position];

            // Whitespace
            if current == ' ' || current == '\t' || current == '\r' {
                self.position += 1;
                self.column += 1;
                continue;
            }

            // Newline
            if current == '\n' {
                self.position += 1;
                self.line += 1;
                self.column = 0;
                continue;
            }

            // Salviamo la posizione INIZIALE del token
            let token_line = self.line;
            let token_column = self.column;

            let mut chars_to_consume = 0;

            match current {
                // Number
                c if c.is_ascii_digit() => {
                    while let Some(c) = self.peek(chars_to_consume) {
                        if !c.is_ascii_digit() {
                            break;
                        }

                        chars_to_consume += 1;
                    }
                }

                // Identifier / Keyword
                c if c.is_ascii_alphabetic() => {
                    while let Some(c) = self.peek(chars_to_consume) {
                        if !c.is_ascii_alphabetic() && !c.is_ascii_digit() {
                            break;
                        }

                        chars_to_consume += 1;
                    }
                }

                // Single-character tokens
                '+' | '-' | '/' | '*' | '=' | ';' | '(' | ')' => {
                    chars_to_consume = 1;
                }

                _ => {
                    self.position += 1;
                    self.column += 1;
                    continue;
                }
            }

            let word = self.consume(chars_to_consume);
            let token = self.token_from_str(&word);

            tokens.push(TokenInfo {
                token,
                pos: Position {
                    line: token_line,
                    column: token_column,
                },
            });

            self.column += chars_to_consume;
        }

        tokens
    }
}