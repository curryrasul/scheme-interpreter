use crate::{engine::*, frontend::lex::*};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    instr: Vec<ScmProcUnit>,
}

impl Parser {
    fn peek(&self) -> Token {
        self.tokens[self.idx].clone()
    }

    fn next(&mut self) -> Token {
        if let Token::Sentiel = self.tokens[self.idx] {
            return Token::Sentiel;
        };
        let res = self.tokens[self.idx].clone();
        self.idx += 1;
        res
    }

    fn parse_expr(&mut self) {
        assert!(matches!(self.next(), Token::OpenParen));

        let mut header_idx = None;
        let mut args_cnt: usize = 666;

        // Get first element (callable)
        match self.peek() {
            Token::Identifier(var) => {
                // TODO: spec

                header_idx = Some(self.instr.len());
                args_cnt = 0;
                self.instr.push(ScmProcUnit::ProcCall(var, 666));
                self.next();
            }

            Token::Value(val) => {
                self.instr
                    .push(ScmProcUnit::ProcCall(String::from("apply"), 2));
                self.instr.push(ScmProcUnit::Val(val));

                header_idx = Some(self.instr.len());
                args_cnt = 0;
                self.instr
                    .push(ScmProcUnit::ProcCall(String::from("list"), 666));
                self.next();
            }

            Token::OpenParen => {
                self.instr
                    .push(ScmProcUnit::ProcCall(String::from("apply"), 2));
                self.parse_expr(); // Consumed here

                header_idx = Some(self.instr.len());
                args_cnt = 0;
                self.instr
                    .push(ScmProcUnit::ProcCall(String::from("list"), 666));
            }

            Token::ClosingParen => {
                self.instr.push(ScmProcUnit::Val(ScmValue::Nil));
            }

            Token::Sentiel => {
                panic!("");
            }
        };

        // Arguments
        loop {
            match self.peek() {
                Token::Identifier(var) => {
                    self.instr.push(ScmProcUnit::Variable(var));
                    self.next();
                }

                Token::Value(val) => {
                    self.instr.push(ScmProcUnit::Val(val));
                    self.next();
                }

                Token::OpenParen => {
                    self.parse_expr();
                }

                Token::ClosingParen => {
                    break;
                }

                Token::Sentiel => {
                    panic!("");
                }
            };
            args_cnt += 1;
        }

        if let Some(idx) = header_idx {
            if let ScmProcUnit::ProcCall(_, cnt) = &mut self.instr[idx] {
                *cnt = args_cnt;
            } else {
                panic!("WTF");
            }
        }

        assert!(matches!(self.next(), Token::ClosingParen));
    }

    pub fn parse(&mut self) -> ScmCallable {
        while let Token::OpenParen = self.peek() {
            self.parse_expr();
        }

        assert!(matches!(self.peek(), Token::Sentiel));

        let proc = ScmProcedure {
            params: Vec::<String>::new(),
            instructions: self.instr.clone(),
        };
        ScmCallable::CustomProc(proc)
    }

    pub fn new(s: &str) -> Self {
        let tokens = Lexer::new(s).run();
        Self {
            tokens: tokens,
            idx: 0,
            instr: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = Parser::new("(+ 1 2)");

        // println!("{:?}", p);
    }
}
