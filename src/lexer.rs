use crate::scm_core::*;

pub struct Position {
    pub line: u32,
    pub row: u32,
}

pub enum TokenType {
    Identifier(String),
    Value(ScmValue),
    OpenBracket,
    ClosingBracket,
}

pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

pub fn lex(code: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let mut line = 1;
    let mut row = 1;

    let chars: Vec<char> = code.chars().collect();

    for mut i in 0..chars.len() {
        match chars[i] {
            ' ' => row += 1,
            '\n' => {
                line += 1;
                row = 1;
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::OpenBracket,
                    position: Position { line, row },
                });
                row += 1;
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::ClosingBracket,
                    position: Position { line, row },
                });
                row += 1;
            }
            '1'..='9' => {
                let mut number = String::from(chars[i]);

                while let Some(cur) = chars.get(i + 1) {
                    if cur.is_digit(10) {
                        number.push(*cur);
                    }
                    i += 1;
                }
            }
            _ => panic!("Wrong character!"),
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let c = 'a';

        match c {
            '1'..='9' => println!("Hello world"),
            _ => panic!(),
        }
    }
}
