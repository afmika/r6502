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
    EXPR(MathExpr)      // label, variable, 1 or 2 bytes hex/dec/bin
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathExpr {
    PLUS, MINUS, MULT, DIV,
    PLACEHOLDER(String), NUM(NumericValue)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    EXPORT, INCLUDE(Token),
    ENDPROC, PROC(Token),                 // .proc main 
    SEGMENT(Token),                       // .segment "NAME"
    BYTE(Vec<Token>),                     // .byte 1, 2, 3, ... (8 bits dec, bin or hex) or even strings
    ENDMACRO, MACRO(Token, Vec<Token>)    // .macro NAME arg1 arg2 ... argN (.*)\n endmacro
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericValue {
    value: u32,
    size: u32
}

pub fn canonicalize_number(n: &Token) -> Result<NumericValue, String> {
    match n {
        Token::BIN(bin) => {
            let value: u32 = u32::from_str_radix(bin, 2).unwrap();
            if bin.len() > 8 {
                return Ok(NumericValue { value, size: 16 })
            }
            Ok(NumericValue { value, size: 8 })
        },
        Token::DEC(dec) => {
            let value: u32 = u32::from_str_radix(dec, 10).unwrap();
            // ex: 256 or 00001 shall be considered as 16 bits
            if value > 255 || dec.len() > 3 {
                return Ok(NumericValue { value, size: 16 })
            }
            Ok(NumericValue { value, size: 8 })
        },
        Token::HEX(hex) => {
            let value: u32 = u32::from_str_radix(hex, 16).unwrap();
            if hex.len() > 2 {
                return Ok(NumericValue { value, size: 16 })
            }
            Ok(NumericValue { value, size: 8 })
        },
        token => {
            Err(format!("token {:?} is not a number", token))
        }
    }
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
            let res = self.state_instr()?;
            println!("{:?}", res);
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
        Err(format!("{:?} was expected, got {:?} instead", token, *self.curr()))
    }

    fn state_instr(&mut self) -> Result<Expr, String> {
        let instr = self.curr();
        self.next();
        // [none ::= implied, accumulator]
        // operand ::= none | imm | abs | ind | rel | zp
        // imm     ::= #$BB
        // abs     ::= $LLHH | $LLHH ',' ('x'|'y')
        // ind     ::= '(' $LLHH ')' | '(' $BB ',' 'x' ')' | '(' $BB  ')' ',' 'y'
        // rel     ::= $BB                                  (context bound: only for jumps BXX)
        // zp      ::= $BB | $BB ',' ('x'|'y')

        if *self.curr() == Token::PARENTOPEN {
            // indirect
            self.consume(Token::PARENTOPEN)?;

            let number = canonicalize_number(self.curr())?;
            if number.size > 8 {
                // indirect
                let op = Operand::EXPR(MathExpr::NUM(number));
                self.next();
                self.consume(Token::PARENTCLOSE)?;
                return Ok(Expr::INSTR(Instr::NOP, AdrMode::IND, op));
            } else {
                // indirect x
                panic!("8 bits non implemented")
            }
        }
        panic!("non implemented")
    }

    fn state_operand(&self) -> Result<Operand, String> {
        Ok(Operand::NONE)
    }

    fn state_math_expr(&mut self) {

    }

}