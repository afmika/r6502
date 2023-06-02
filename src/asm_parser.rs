use std::cmp::min;

use crate::asm_lexer::Token;

// https://famicom.party/book/05-6502assembly/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    MAIN,
    PROC,
    END,
    LABEL(String),
    INSTR(String, Operand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    NONE,               // implied
    IMM(Box<Operand>),       // immediate
    X, Y, A,            // registers
    LABEL,              // jumps
    PAIR(Box<Operand>, Box<Operand>),
    HEX(String),
    BIN(String)
}

pub struct AsmParser<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
}

impl<'a> AsmParser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut prog = Vec::new();
        self.cursor = 0;
        loop {
            // cleanup
            if self.is_eof() {
                break;
            }
            let token = self.curr();

            match token {
                Token::MAIN => prog.push(Expr::MAIN),
                Token::PROC => prog.push(Expr::PROC),
                Token::END => prog.push(Expr::END),
                Token::COMMENT(_) => {},
                _ => {
                    prog.push(self.state_instr()?);
                    continue;
                }
            };

            self.next();
        }
        Ok(prog)
    }

    fn curr(&self) -> &Token {
        self
            .tokens
            .get(self.cursor)
            .unwrap_or(&Token::EOF)
    }

    fn peek_next(&self) -> &Token {
        self
            .tokens
            .get(self.cursor + 1)
            .unwrap_or(&Token::EOF)
    }

    fn is_eof(&self) -> bool {
        *self.curr() == Token::EOF 
    }

    fn next(&mut self) -> &Token {
        self.cursor = min(self.tokens.len(), self.cursor + 1);
        return self.curr();
    }

    fn prev(&mut self) -> &Token {
        return self.back(1);
    }

    fn back(&mut self, count: usize) -> &Token {
        if self.cursor > (count - 1) {
            self.cursor -= count;
        }
        return self.curr();
    }

    fn consume(&mut self, token: Token) -> Result<Token, String> {
        if *self.curr() == token {
            self.next();
            return Ok(token);
        }
        Err(format!("{:?} was expected", token))
    }

    fn state_instr(&mut self) -> Result<Expr, String> {
        // label
        if *self.peek_next() == Token::COLON {
            match self.curr() {
                Token::LITERAL(s) => {
                    return Ok(Expr::LABEL(s.to_string()));
                },
                _ => {}
            }
        }

        // instr
        match self.curr() {
            Token::LITERAL(s) => {
                let op = self.state_operand()?;
                Ok(Expr::INSTR(s.to_string(), op))
            },
            any => Err(format!("{:?} unexpected", any))
        }
    }

    fn state_operand(&self) -> Result<Operand, String> {
        Ok(Operand::A)
    }

}