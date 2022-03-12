use crate::engine::scm_core::ScmValue;
use crate::engine::typed_num::TypedNum;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Value(ScmValue),
    OpenParen,
    ClosingParen,
    Sentiel,
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
                    self.increment();
                    match self.current {
                        Some('\'') => self.parse_char(),
                        _ => self.parse_boolean(),
                    }
                }
                '0'..='9' => self.parse_number(),
                '\"' => self.parse_string(),
                '[' | ']' | '{' | '}' | '|' | '\\' => {
                    panic!("Unexpected {} {}", self.line, self.column);
                }
                _ => self.parse_identifier(),
            }
        }

        self.tokens.push(Token::Sentiel);
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

    fn parse_number(&mut self) {
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

        let s = s.parse().unwrap();
        self.tokens
            .push(Token::Value(ScmValue::Number(TypedNum::Integer(s))));

        self.parse_delimiter();
    }

    fn parse_identifier(&mut self) {
        let mut s = String::new();

        while let Some(c) = self.current {
            match c {
                ' ' | '(' | ')' | '\n' => break,
                _ => {
                    s.push(c);
                    self.increment();
                }
            }
        }

        self.tokens.push(Token::Identifier(s));

        self.parse_delimiter();
    }

    fn parse_boolean(&mut self) {
        let val;

        match self.current {
            Some('t') => val = true,
            Some('f') => val = false,
            _ => panic!("Lexer error bool type on {} {}", self.line, self.column),
        }

        self.increment();

        self.tokens.push(Token::Value(ScmValue::Bool(val)));

        self.parse_delimiter();
    }

    fn parse_string(&mut self) {
        self.increment();

        let mut s = String::new();

        while let Some(c) = self.current {
            match c {
                '\"' => {
                    self.increment();
                    break;
                }
                _ => {
                    s.push(c);
                    self.increment();
                }
            }
        }

        self.tokens.push(Token::Value(ScmValue::String(s)));

        self.parse_delimiter();
    }

    fn parse_char(&mut self) {
        self.increment();

        match self.current {
            Some(c @ 'a'..='z') => self.tokens.push(Token::Value(ScmValue::Char(c))),
            _ => panic!("Lexer error on {} {}", self.line, self.column),
        }

        self.increment();

        self.parse_delimiter();
    }

    fn parse_delimiter(&mut self) {
        match self.current {
            Some(')') => {
                self.tokens.push(Token::ClosingParen);
                self.increment();
            }
            Some(' ') | Some('\n') | None => (),
            Some(_) => panic!("Lexer error on {} {}", self.line, self.column),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let contents = fs::read_to_string("test.scm").expect("No such file");
        let mut l = Lexer::new(&contents);

        let v = l.run();

        println!("{:#?}", v);
    }
}
