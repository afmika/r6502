use std::cmp::min;

use crate::asm_lexer::Token;
use crate::opcodes::{
    Instr,
    AdrMode, INSTR
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
    ASSIGN(String, Operand), // lit, value
    LABEL(String),
    INSTR(Instr, AdrMode, Operand)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    NONE,               // implied
    EXPR(MathExpr)      // label, variable, 1 or 2 bytes hex/dec/bin
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathExpr {
    BIN(Token, Box<MathExpr>, Box<MathExpr>),
    PLACEHOLDER(String), NUM(NumericValue)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    EXPORT, INCLUDE(String),
    ENDPROC, PROC(String),                 // .proc main 
    SEGMENT(String),                       // .segment "NAME"
    BYTE(Vec<String>),                     // .byte 1, 2, 3, ... (8 bits dec, bin or hex) or even strings
    ENDMACRO, MACRO(String, Vec<String>)    // .macro NAME arg1 arg2 ... argN (.*)\n endmacro
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumericValue {
    value: u32,
    size: u32
}

fn canonicalize_number(n: &Token) -> Result<NumericValue, String> {
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


fn get_instr(s: &String) -> Result<Instr, String> {
    match INSTR.get(s) {
        Some(i) => Ok(i.to_owned()),
        None => Err(format!("{:?} is not a valid instruction", s))
    }
}

fn is_branching(i: &Instr) -> bool {
    let list = [
        Instr::BPL, Instr::BMI, Instr::BVC,
        Instr::BVS, Instr::BCC, Instr::BCS,
        Instr::BNE, Instr::BEQ
    ];
    for item in list {
        if item.to_string().eq(&i.to_string()) {
            return true;
        }
    }
    false
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
            if *self.curr() == Token::NEWLINE {
                self.next();
                continue;
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

    fn consume_literal(&mut self, s: &str) -> Result<Token, String> {
        let curr = self.curr().clone();
        match &curr {
            Token::LITERAL(lit) => {
                if lit.ne(s) {
                    return Err(format!("literal {:?} was expected, got {:?} instead", s, curr))
                }
                let curr = self.curr().clone();
                self.next();
                Ok(curr)
            },
            token => Err(format!("literal {:?} was expected, got {:?} instead", s, token))
        }
    }

    // expr      ::= term (+| -) expr | term
    fn consume_math_expr(&mut self) -> Result<MathExpr, String> {
        let expr = self.consume_math_term()?;
        let bin = vec![Token::PLUS, Token::MINUS];
        for op in bin {
            if *self.curr() == op {
                let op_token = self.consume(op)?;
                let right = self.consume_math_expr()?;
                return Ok(MathExpr::BIN(op_token, Box::new(expr), Box::new(right)));
            }
        }
        Ok(expr)
    }

    // term      ::= factor (* | /) term | factor
    fn consume_math_term(&mut self) -> Result<MathExpr, String> {
        let expr = self.consume_math_factor()?;
        let bin = vec![Token::MULT, Token::DIV];
        for op in bin {
            if *self.curr() == op {
                let op_token = self.consume(op)?;
                let right = self.consume_math_term()?;
                return Ok(MathExpr::BIN(op_token, Box::new(expr), Box::new(right)));
            }
        }
        Ok(expr)
    }

    // factor    ::= (expr) | unary
    fn consume_math_factor(&mut self) -> Result<MathExpr, String> {
        if *self.curr() == Token::PARENTOPEN {
            self.consume(Token::PARENTOPEN)?;
            let expr = self.consume_math_expr()?;
            self.consume(Token::PARENTCLOSE)?;
            return Ok(expr);
        }
        self.consume_math_unary()
    }

    // unary     ::= <literal> | hex | dec | bin
    fn consume_math_unary(&mut self) -> Result<MathExpr, String> {
        match canonicalize_number(&self.curr()) {
            Ok(number) => {
                self.next();
                Ok(MathExpr::NUM(number))
            },
            Err(e) => {
                match &self.curr() {
                    Token::LITERAL(s) => {
                        let out = MathExpr::PLACEHOLDER(s.to_string());
                        self.next();
                        Ok(out)
                    },
                    _ => {
                        Err(e)
                    }
                }
            }
        }
    }

    fn state_instr(&mut self) -> Result<Expr, String> {
        let instr = match self.curr().clone() {
            Token::LITERAL(i) => Ok(get_instr(&i)?),
            token => Err(format!("{:?} is not a literal", token))
        }?;
        self.next();
        // [none ::= implied, accumulator]
        // operand ::= none | imm | abs | ind | rel | zp
        // imm     ::= #$BB
        // ind     ::= '(' $LLHH ')' | '(' $BB ',' 'x' ')' | '(' $BB  ')' ',' 'y'
        // rel     ::= $BB                                  (context bound: only for jumps BXX)
        // zp      ::= $BB | $BB ',' ('x'|'y')
        // abs     ::= $LLHH | $LLHH ',' ('x'|'y')

        // none
        if *self.curr() == Token::NEWLINE || *self.curr() == Token::EOF {
            return Ok(Expr::INSTR(instr, AdrMode::IMPL, Operand::NONE));
        }

        // branching BXX
        if is_branching(&instr) {
            match canonicalize_number(self.curr()) {
                Ok(number) => {
                    let op = Operand::EXPR(MathExpr::NUM(number));
                    return Ok(Expr::INSTR(instr, AdrMode::REL, op));
                },
                Err(e) => {
                    match self.curr() {
                        Token::LITERAL(s) => {
                            let op = Operand::EXPR(MathExpr::PLACEHOLDER(s.clone()));
                            return Ok(Expr::INSTR(instr, AdrMode::REL, op));
                        },
                        _ => { return Err(e) }
                    }
                }
            }
        }

        // immidiate
        if *self.curr() == Token::HASH {
            self.consume(Token::HASH)?;
            let number = self.consume_math_expr()?;
            let op = Operand::EXPR(number);
            return Ok(Expr::INSTR(instr, AdrMode::IMM, op));
        }

        // ind, indx, indy
        if *self.curr() == Token::PARENTOPEN {
            // indirect
            self.consume(Token::PARENTOPEN)?;

            let number = canonicalize_number(self.curr())?;
            if number.size > 8 {
                // indirect
                let op = Operand::EXPR(MathExpr::NUM(number));
                self.next();
                self.consume(Token::PARENTCLOSE)?;
                return Ok(Expr::INSTR(instr, AdrMode::IND, op));
            } else {
                let op = Operand::EXPR(MathExpr::NUM(number));
                self.next();
                if *self.curr() == Token::COMMA {
                    // indirect x
                    self.consume(Token::COMMA)?;
                    self.consume_literal("x")?;
                    self.consume(Token::PARENTCLOSE)?;
                    return Ok(Expr::INSTR(instr, AdrMode::INDX, op));
                } else {
                    // indirect y
                    self.consume(Token::PARENTCLOSE)?;
                    self.consume(Token::COMMA)?;
                    self.consume_literal("y")?;
                    return Ok(Expr::INSTR(instr, AdrMode::INDY, op));
                }
            }
        }

        // abs and zp
        let number = canonicalize_number(self.curr())?;
        if number.size > 8 {
            // abs
            let op = Operand::EXPR(MathExpr::NUM(number));
            let mut mode = AdrMode::ABS;
            self.next();
            if *self.curr() == Token::COMMA {
                self.consume(Token::COMMA)?;
                match self.consume_literal("x") {
                    Ok(_) =>  { mode = AdrMode::ABSX },
                    Err(_) => { 
                        self.consume_literal("y")?;
                        mode = AdrMode::ABSY;
                    }
                };
            }
            return Ok(Expr::INSTR(instr, mode, op));
        } else {
            // zp
            let op = Operand::EXPR(MathExpr::NUM(number));
            let mut mode = AdrMode::ZP;
            self.next();
            if *self.curr() == Token::COMMA {
                self.consume(Token::COMMA)?;
                match self.consume_literal("x") {
                    Ok(_) =>  { mode = AdrMode::ZPX },
                    Err(_) => { 
                        self.consume_literal("y")?;
                        mode = AdrMode::ZPY;
                    }
                };
            }
            return Ok(Expr::INSTR(instr, mode, op));
        }
    }
}