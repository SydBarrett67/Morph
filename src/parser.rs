use ast::Node;

enum ParserError {
    SyntaxError(String),
}

struct Parser {
    tokens: Vec<Token>,
    index:  usize,
}
impl Parser {
    // Constructor
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            index: 0,
        }
    }

    pub fn peek(&self, look_ahead: i32) -> Option<Token> {
        self.tokens[self.index + look_ahead].copied()
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        let mut root_node: Node = Node::Root(Vec::new());

        for token in tokens {
            let mut statement: Node;
            match token {
                Token::Keyword(Keyword::LET) => {
                    while let Some(t) = self.peek(0) {
                        match t {
                            
                        }
                    }
                }
            }

            if let Node::Root(nodes) = &mut root_node {
                nodes.push(
                    statement
                )
            }
        }

        root_node
    }

}