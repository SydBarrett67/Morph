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
            "if"    => Token::Keyword(Keyword::IF), 
            "else"  => Token::Keyword(Keyword::ELSE),
            "while" => Token::Keyword(Keyword::WHILE),
            "func"  => Token::Keyword(Keyword::FUNC),

            // Classical operands
            "+"     => Token::Operand(Operand::PLUS),
            "-"     => Token::Operand(Operand::MINUS),
            "*"     => Token::Operand(Operand::STAR),
            "/"     => Token::Operand(Operand::SLASH),
            "="     => Token::Operand(Operand::EQUALS),

            // Boolean operands
            "and"   => Token::Operand(Operand::AND),
            "&&"    => Token::Operand(Operand::AND),
            "or"    => Token::Operand(Operand::OR),
            "||"    => Token::Operand(Operand::OR),
            "=="    => Token::Operand(Operand::EQCOMP),

            // Structure
            "("     => Token::OpenParen,
            ")"     => Token::CloseParen,
            "{"     => Token::OpenBrace,
            "}"     => Token::CloseBrace,
            ";"     => Token::Semicolon,
            ":"     => Token::Colon,
            ","     => Token::Comma,
            "\""    => Token::Quotation,
            ">"     => Token::Operand(Operand::MORE),
            "<"     => Token::Operand(Operand::LESS),

            // Literal or identifier
            _ => match word.parse::<i32>() {

                Ok(number) => Token::Literal(number.to_string()),
                Err(_) => match word {
                    "true" | "false" => Token::Literal(word.to_string()),

                    _ => {
                        if word.starts_with("\"") && word.ends_with("\"") {
                            Token::Literal(word.to_string())
                        } else {
                            Token::Identifier(word.to_string())
                        }
                    }
                }
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

                // String parsing
                '"' => {
                    chars_to_consume += 1;
                    while let Some(c) = self.peek(chars_to_consume) {
                        if c == '"' {
                            chars_to_consume += 1;
                            break;
                        }
                        chars_to_consume += 1;
                    }
                }

                // Single-character tokens
                '+' | '-' | '/' | '*' | '=' | 
                ';' | '(' | ')' | '{' | '}' |
                '>' | '<' | ',' | ':' => {
                    if self.peek(1) == Some('=') {
                        chars_to_consume = 2;
                    }
                    else { chars_to_consume = 1; }
                }

                // 2 char tokens
                '&' | '|' => {
                    if self.peek(1) == Some('&') {
                        chars_to_consume = 2;
                    }
                    else {
                        self.position += 1;
                        self.column += 1;
                        continue;
                    }
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