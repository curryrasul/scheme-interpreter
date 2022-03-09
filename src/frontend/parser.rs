use crate::frontend::lex::{Lexer, Token};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(s: &str) -> Self {
        Self {
            tokens: Lexer::new(s).run(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = Parser::new("(+ 1 2)");

        println!("{:?}", p);
    }
}
