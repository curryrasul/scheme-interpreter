mod lex;
use lex::Lexer;

pub use lex::Token;

pub fn lex(s: &str) -> Vec<Token> {
    let mut l = Lexer::new(s);

    l.run()
}
