use std::cmp::min;

use crate::asm_lexer::Token;
use crate::opcodes::{
    Instr,
    AdrMode
};

/**
 * * decimal
 * @TODOS
 * 0. start at an offset
 * ORG $0080
 * AND other compiler directive
 * 
 * 1. store variables, and strings
 * SRC     .WORD $0400     ;source string pointer
 * MY_STR  .
 * 
 * 2. in place operations and support of chars
 * CMP #'A'
 * CMP #'Z'+1
 */

// https://famicom.party/book/05-6502assembly/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    DIRECTIVE(Directive),
    ASSIGN(Token, Operand), // lit, value
    LABEL(Token),
    INSTR(Instr, AdrMode, Operand)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    NONE,               // implied
    VALUE(Token)       // label, variable, 1 or 2 bytes hex/dec/bin
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    ORG, PROC, MAIN, END,
    EXPORT, SEGMENT,
    WORD, DB
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
                Token::COMMENT(_) => {},
                _ => {
                    // prog.push(self.state_instr()?);
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

    // fn state_instr(&mut self) -> Result<Expr, String> {
    //     // label
    //     if *self.peek_next() == Token::COLON {
    //         match self.curr() {
    //             Token::LITERAL(s) => {
    //                 return Ok(Expr::LABEL(s.to_string()));
    //             },
    //             _ => {}
    //         }
    //     }

    //     // instr
    //     match self.curr() {
    //         Token::LITERAL(s) => {
    //             let op = self.state_operand()?;
    //             Ok(Expr::INSTR(s.to_string(), op))
    //         },
    //         any => Err(format!("{:?} unexpected", any))
    //     }
    // }

    fn state_operand(&self) -> Result<Operand, String> {
        Ok(Operand::VALUE(Token::BIN("1234".to_string())))
    }

}