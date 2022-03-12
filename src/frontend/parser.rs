use std::vec;

use crate::{engine::*, frontend::lex::*};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
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

    // Parsing

    fn parse_value(&mut self, instr: &mut Vec<ScmProcUnit>) {
        match self.peek() {
            Token::Identifier(ident) => {
                instr.push(ScmProcUnit::Variable(ident));
                self.next();
            }
            Token::Value(val) => {
                instr.push(ScmProcUnit::Val(val));
                self.next();
            }
            Token::OpenParen => {
                self.parse_expr(instr);
            }
            _ => {
                panic!("Unexpected symbol");
            }
        }
    }

    fn gen_define(&mut self, instr: &mut Vec<ScmProcUnit>) {
        match self.peek() {
            Token::Identifier(ident) => {
                instr.push(ScmProcUnit::Assign(String::from(ident)));
                self.next();

                // Get one value. If there are others, error will be raised later.
                self.parse_value(instr);
            }

            Token::OpenParen => {
                self.next();

                if let Token::Identifier(ident) = self.peek() {
                    instr.push(ScmProcUnit::Assign(String::from(ident)));
                } else {
                    panic!("Expected procedure name");
                }
                self.next();

                let mut params = Vec::new();
                while let Token::Identifier(ident) = self.peek() {
                    params.push(ident);
                    self.next();
                }
                assert!(matches!(self.next(), Token::ClosingParen));

                let start_idx = instr.len();
                self.parse_expr(instr);
                let lambda_size = instr.len() - start_idx;

                instr.push(ScmProcUnit::Lambda {
                    args: params,
                    units_cnt: lambda_size,
                });
            }

            _ => {
                panic!("Unexpected symbol");
            }
        };
    }

    fn gen_lambda(&mut self, instr: &mut Vec<ScmProcUnit>) {
        let params = if let Token::Identifier(ident) = self.peek() {
            vec![ident]
        } else if let Token::OpenParen = self.peek() {
            self.next();

            let mut res = Vec::new();
            while let Token::Identifier(param) = self.peek() {
                res.push(param);
                self.next();
            }

            assert!(matches!(self.next(), Token::ClosingParen));

            res
        } else {
            panic!("Formals expected");
        };

        let start_idx = instr.len();
        self.parse_value(instr);
        let lambda_size = instr.len() - start_idx;

        instr.push(ScmProcUnit::Lambda {
            args: params,
            units_cnt: lambda_size,
        });
    }

    fn gen_condif(&mut self, instr: &mut Vec<ScmProcUnit>) {
        let mut cond_instr = Vec::new();
        self.parse_value(&mut cond_instr);

        let mut true_instr = Vec::new();
        self.parse_value(&mut true_instr);

        let has_else = !matches!(self.peek(), Token::ClosingParen);
        if has_else {
            let start_idx = instr.len();
            self.parse_value(instr);
            let fbr_size = instr.len() - start_idx;
            instr.push(ScmProcUnit::FalseBranch(fbr_size));
        } else {
            instr.push(ScmProcUnit::Val(ScmValue::Nil));
            instr.push(ScmProcUnit::FalseBranch(1));
        }

        let tbr_size = true_instr.len() + 1;
        instr.append(&mut true_instr);
        instr.push(ScmProcUnit::TrueBranch(tbr_size));

        instr.append(&mut cond_instr);
    }

    fn parse_expr(&mut self, instr: &mut Vec<ScmProcUnit>) {
        assert!(matches!(self.next(), Token::OpenParen));

        let mut header_idx = None;
        let mut args_cnt: usize = 666;

        // Get first element (callable)
        match self.peek() {
            Token::Identifier(var) if var == "define" => {
                self.next();
                instr.push(ScmProcUnit::Val(ScmValue::Nil));
                self.gen_define(instr);
                assert!(matches!(self.next(), Token::ClosingParen));
                return;
            }

            Token::Identifier(var) if var == "lambda" => {
                self.next();
                self.gen_lambda(instr);
                assert!(matches!(self.next(), Token::ClosingParen));
                return;
            }

            Token::Identifier(var) if var == "if" => {
                self.next();
                self.gen_condif(instr);
                assert!(matches!(self.next(), Token::ClosingParen));
                return;
            }

            Token::Identifier(var) => {
                header_idx = Some(instr.len());
                args_cnt = 0;
                instr.push(ScmProcUnit::ProcCall(var, 666));
                self.next();
            }

            Token::Value(val) => {
                instr.push(ScmProcUnit::ProcCall(String::from("apply"), 2));
                instr.push(ScmProcUnit::Val(val));

                header_idx = Some(instr.len());
                args_cnt = 0;
                instr.push(ScmProcUnit::ProcCall(String::from("list"), 666));
                self.next();
            }

            Token::OpenParen => {
                instr.push(ScmProcUnit::ProcCall(String::from("apply"), 2));
                self.parse_expr(instr); // Consumed here

                header_idx = Some(instr.len());
                args_cnt = 0;
                instr.push(ScmProcUnit::ProcCall(String::from("list"), 666));
            }

            Token::ClosingParen => {
                instr.push(ScmProcUnit::Val(ScmValue::Nil));
            }

            Token::Sentiel => {
                panic!("");
            }
        };

        // Arguments
        loop {
            match self.peek() {
                Token::ClosingParen => {
                    break;
                }
                Token::Sentiel => {
                    panic!("Unexpected EOF");
                }
                _ => {
                    self.parse_value(instr);
                }
            };
            args_cnt += 1;
        }

        if let Some(idx) = header_idx {
            if let ScmProcUnit::ProcCall(_, cnt) = &mut instr[idx] {
                *cnt = args_cnt;
            } else {
                panic!("WTF");
            }
        }

        assert!(matches!(self.next(), Token::ClosingParen));
    }

    pub fn parse(&mut self) -> Vec<ScmCallable> {
        let mut res = Vec::new();

        while let Token::OpenParen = self.peek() {
            let mut instr = Vec::new();

            self.parse_expr(&mut instr);
            res.push(ScmCallable::CustomProc(ScmProcedure {
                params: Vec::<String>::new(),
                instructions: instr,
            }));
        }

        assert!(matches!(self.peek(), Token::Sentiel));

        res
    }

    pub fn new(s: &str) -> Self {
        let tokens = Lexer::new(s).run();
        Self { tokens, idx: 0 }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         // let p = Parser::new("(+ 1 2)");

//         // println!("{:?}", p);
//     }
// }
