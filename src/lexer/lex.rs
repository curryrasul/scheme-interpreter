use crate::scm_core::*;
use std::vec::IntoIter;

pub enum Token {
    Identifier(String),
    Value(ScmValue),
    OpenParen,
    ClosingParen,
}

pub(super) struct Lexer {
    chars: IntoIter<char>,
    tokens: Vec<Token>,
    current: Option<char>,
    line: u32,
    column: u32,
}

impl Lexer {
    pub(super) fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect::<Vec<char>>().into_iter(),
            tokens: Vec::new(),
            current: None,
            line: 1,
            column: 1,
        }
    }

    pub(super) fn run(&mut self) -> Vec<Token> {
        self.increment();

        while let Some(c) = self.current {
            match c {
                ' ' | '\n' => {
                    self.increment();
                }
                '(' => {
                    self.tokens.push(Token::OpenParen);
                    self.increment();
                }
                ')' => {
                    self.tokens.push(Token::ClosingParen);
                    self.increment();
                }
                '#' => {
                    let val = self.parse_boolean();
                    self.tokens.push(Token::Value(ScmValue::Bool(val)));
                }
                '0'..='9' => {
                    let val = self.parse_number();
                    self.tokens.push(Token::Value(ScmValue::Integer(val)));
                }
                '\"' => {
                    let val = self.parse_string();
                    self.tokens.push(Token::Value(ScmValue::String(val)));
                }
                _ => panic!("Lexer error"),
            }
        }

        std::mem::take(&mut self.tokens)
    }

    fn increment(&mut self) {
        if let Some('\n') = self.current {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.current = self.chars.next();
    }

    fn parse_number(&mut self) -> i64 {
        let mut s = String::new();

        while let Some(c) = self.current {
            match c {
                '0'..='9' => {
                    s.push(c);
                    self.increment();
                }
                _ => break,
            }
        }

        s.parse().unwrap()
    }

    // fn parse_identifier(&mut self) {}

    fn parse_boolean(&mut self) -> bool {
        true
    }

    fn parse_string(&mut self) -> String {
        "Hello".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut l = Lexer::new("(123 456 89)");

        let v = l.run();

        println!("{:?}", v);
    }
}
