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

    // Parsing

    fn parse_value(&mut self) {
        match self.peek() {
            Token::Identifier(ident) => {
                self.instr.push(ScmProcUnit::Variable(ident));
                self.next();
            }
            Token::Value(val) => {
                self.instr.push(ScmProcUnit::Val(val));
                self.next();
            }
            Token::OpenParen => {
                self.parse_expr();
            }
            _ => {
                panic!("Unexpected symbol");
            }
        }
    }

    fn gen_define(&mut self) {
        self.next(); // "define"
        

        match self.peek() {
            Token::Identifier(ident) => {
                self.instr.push(ScmProcUnit::Assign(String::from(ident)));
                self.next();

                // Get one value. If there are others, error will be raised later.
                self.parse_value();
            }

            Token::OpenParen => {
                self.next();

                if let Token::Identifier(ident) = self.peek() {
                    self.instr.push(ScmProcUnit::Assign(String::from(ident)));
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

                let start_idx = self.instr.len();
                self.parse_expr();
                let lambda_size = self.instr.len() - start_idx;

                self.instr.push(ScmProcUnit::Lambda {
                    args: params,
                    units_cnt: lambda_size,
                });
            }

            _ => {
                panic!("Unexpected symbol");
            }
        };
    }

    fn parse_expr(&mut self) {
        assert!(matches!(self.next(), Token::OpenParen));

        let mut header_idx = None;
        let mut args_cnt: usize = 666;

        // Get first element (callable)
        match self.peek() {
            Token::Identifier(var) if var == "define" => {
                self.instr.push(ScmProcUnit::Val(ScmValue::Nil));
                self.gen_define();
                assert!(matches!(self.next(), Token::ClosingParen));
                return;
            }

            Token::Identifier(var) if var == "lambda" => {
                todo!()
            }

            Token::Identifier(var) if var == "if" => {
                todo!()
            }

            Token::Identifier(var) => {
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
                Token::ClosingParen => {
                    break;
                }
                Token::Sentiel => {
                    panic!("Unexpected EOF");
                }
                _ => {
                    self.parse_value();
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

    pub fn parse(&mut self) -> Vec<ScmCallable> {
        let mut res = Vec::new();

        while let Token::OpenParen = self.peek() {
            self.parse_expr();
            res.push(ScmCallable::CustomProc(ScmProcedure {
                params: Vec::<String>::new(),
                instructions: self.instr.clone(),
            }));
            self.instr.clear();
        }

        assert!(matches!(self.peek(), Token::Sentiel));

        res
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         // let p = Parser::new("(+ 1 2)");

//         // println!("{:?}", p);
//     }
// }
