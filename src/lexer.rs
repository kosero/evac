#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Equal,
    Unknown(char),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.next_char() {
            let token = match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '=' => Token::Equal,
                ';' => Token::Semicolon,
                '0'..='9' => {
                    let mut num = ch.to_string();
                    while let Some(next) = self.peek_char() {
                        if next.is_ascii_digit() {
                            num.push(self.next_char().unwrap());
                        } else {
                            break;
                        }
                    }
                    Token::Number(num.parse().unwrap())
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = ch.to_string();
                    while let Some(next) = self.peek_char() {
                        if next.is_alphanumeric() || next == '_' {
                            ident.push(self.next_char().unwrap());
                        } else {
                            break;
                        }
                    }
                    Token::Identifier(ident)
                }
                _ if ch.is_whitespace() => continue,
                _ => Token::Unknown(ch),
            };
            tokens.push(token);
        }

        tokens
    }
}
