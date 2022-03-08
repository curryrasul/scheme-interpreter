use crate::scm_core::*;
use std::vec::IntoIter;

pub enum Token {
    Identifier(String),
    Value(ScmValue),
    OpenBracket,
    ClosingBracket,
}

struct Lexer {
    chars: IntoIter<char>,
    tokens: Vec<Token>,
    current: Option<char>,
    line: u32,
    column: u32,
}

pub fn lex(s: &str) -> Vec<Token> {
    let mut l = Lexer::new(s);

    l.run();

    l.tokens
}

impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect::<Vec<char>>().into_iter(),
            tokens: Vec::new(),
            current: None,
            line: 1,
            column: 1,
        }
    }

    fn run(&mut self) {}

    fn increment(&mut self) {
        if let Some('\n') = self.current {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.current = self.chars.next();
    }

    fn parse_number(&mut self) {}

    fn parse_identifier(&mut self) {}

    fn parse_boolean(&mut self) {}

    fn parse_string(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
