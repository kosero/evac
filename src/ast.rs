use crate::lexer::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ASTNode {
    Identifier(String),
    Number(i64),
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>,
    },
    Return(Box<ASTNode>),
    Assignment {
        left: String,
        right: Box<ASTNode>,
    },
    Block(Vec<ASTNode>),
}

pub struct AST {
    tokens: Vec<Token>,
    position: usize,
}

impl AST {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn parse_block(&mut self) -> Option<ASTNode> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek_token() {
            match token {
                Token::RBrace => {
                    self.next_token();
                    break;
                }
                _ => {
                    if let Some(statement) = self.parse_expression() {
                        statements.push(statement);
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(ASTNode::Block(statements))
    }

    pub fn parse_expression(&mut self) -> Option<ASTNode> {
        let token = self.next_token()?;
        println!("Parsing token: {:?}", token);

        if token == Token::LBrace {
            return self.parse_block();
        }

        if let Token::Identifier(ref name) = token {
            if name == "fn" {
                let func_name = match self.next_token()? {
                    Token::Identifier(name) => name,
                    _ => return None,
                };
                println!("Function name: {:?}", func_name);

                if self.next_token()? != Token::LParen {
                    return None;
                }

                let mut params = Vec::new();
                while let Some(Token::Identifier(param)) = self.next_token() {
                    params.push(param);
                    if let Some(Token::RParen) = self.peek_token() {
                        self.next_token();
                        break;
                    }
                }

                println!("Parameters: {:?}", params);

                if self.next_token()? != Token::LBrace {
                    return None;
                }

                let body = self.parse_block()?;

                return Some(ASTNode::Function {
                    name: func_name,
                    params,
                    body: Box::new(body),
                });
            } else if name == "return" {
                let expr = self.parse_expression()?;
                return Some(ASTNode::Return(Box::new(expr)));
            }
        }

        if let Token::Identifier(var_name) = token {
            if let Some(Token::Equal) = self.peek_token() {
                self.next_token();
                let expr = self.parse_expression()?;
                return Some(ASTNode::Assignment {
                    left: var_name,
                    right: Box::new(expr),
                });
            }
            return Some(ASTNode::Identifier(var_name));
        }

        let left = match token {
            Token::Number(n) => ASTNode::Number(n),
            _ => return None,
        };

        if let Some(op) = self.next_token() {
            match op {
                Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Equal => {
                    let right = self.parse_expression()?;
                    return Some(ASTNode::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    });
                }
                _ => {}
            }
        }

        Some(left)
    }
}
