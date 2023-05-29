use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Token {
    INSTR,
    COMMENT,
    LABEL,
    VALUE,
    OP(Operand),
}

#[derive(Debug, Clone)]
pub enum Operand {
    NONE,
    DEC(String), // #XXX
    HEX(String), // $XX
    BIN(String), // %XXXXYYYY
    ABS(Box<Operand>), // ( (\$|#|$)[0-9]{2,8} )
}

pub struct AsmParser {
    pub source: String,
    cursor: i32,
}

impl AsmParser {
    pub fn new(source: String) -> Self {
        Self {
            source,
            cursor: 0
        }
    }

    pub fn run(&mut self) -> Result<Vec<Token>, String> {
        self.cursor = 0;
        let mut prog = Vec::new();
        prog.push(Token::COMMENT);
        prog.push(Token::OP(Operand::BIN(String::from("1234"))));
        Ok(prog)
    }
}