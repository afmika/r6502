#[derive(Debug, Clone)]
pub enum Token {
    LITERAL(String),    // [\w]+ 
    WHITESPACE,         // \s | \t | \n
    COMM(String),       // ;(.*)
    LABEL(String),      // :[\w]+
    OP(Operand),
    EOF
}

#[derive(Debug, Clone)]
pub enum Operand {
    X, Y, A,            // registers, accumulator
    LABEL(String),      // (.*)
    ADDR(String),       // $XXXX
    IMM(Box<Operand>),  // #value, #$ADDR
    ABS(Box<Operand>),  // ( (\$|#|$)[0-9]{2,8} | label )
}

pub struct AsmLexer {
    source: Vec<char>,
    cursor: usize,
}

impl AsmLexer {
    pub fn new(source: &String) -> Self {
        Self {
            source: source
                .trim()
                .chars()
                .collect(),
            cursor: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut prog = Vec::new();
        self.cursor = 0;
        loop {
            // cleanup
            self.consume_whitespace().ok();
            self.consume_comment().ok();

            if self.is_eof() {
                break;
            }

            let c = *self.curr();

            // label
            if c == ':' {
                prog.push(self.consume_label()?);
                continue;
            }

            // instruction
            if c.is_alphanumeric() {
                let ins = self.consume_instr()?;
                prog.push(ins);
                continue;
            }

            // op1, op2 | ; ... | \n
            if self.is_endline() {
                self.next();
                continue;
            }

            // operands
            prog.push(Token::OP(self.consume_operand()?));
            self.consume_whitespace().ok();
            if c == ',' {
                self.next();
                self.consume_whitespace().ok();
                prog.push(Token::OP(self.consume_operand()?));
            }
        }

        Ok(prog)
    }

    fn curr(&mut self) -> &char {
        self
            .source
            .get(self.cursor)
            .unwrap_or(&'\0')
    }

    fn is_eof(&mut self) -> bool {
        *self.curr() == '\0'
    }

    fn is_endline(&mut self) -> bool {
        let c = *self.curr();
        if c == '\r' {
            self.next(); // \n
            return self.is_endline();
        }
        self.is_eof() || c == '\n'
    }

    fn next(&mut self) -> &char {
        self.cursor += 1;
        return self.curr();
    }

    fn consume_instr(&mut self) -> Result<Token, String> {
        self.consume_alphanum()
            .map(|tk| Token::LITERAL(tk))
    }

    fn consume_label(&mut self) -> Result<Token, String> {
        let c = *self.curr();
        self.next();

        // label
        if c == ':' {
            let value = self.consume_alphanum()?;
            Ok(Token::LABEL(value))
        } else {
            Err("':' was expected".to_string())
        }
    }

    fn consume_comment(&mut self) -> Result<Token, String> {
        if *self.curr() != ';' {
            return Err("';' was expected".to_string());
        }
        self.next();

        let mut tk = String::from("");
        while !self.is_endline() && !self.is_eof() {
            tk.push(*self.next());
        }
        Ok(Token::COMM(tk))
    }

    fn consume_alphanum(&mut self) -> Result<String, String> {
        let mut tk = String::from("");
        while self.curr().is_alphanumeric() {
            tk.push(*self.curr());
            self.next();
        }

        if tk.len() == 0 {
            Err(format!("alphanum was expected, got '{}'", self.curr()))
        } else {
            Ok(tk)
        }
    }

    fn consume_whitespace(&mut self) -> Result<Token, String> {
        let mut s = String::from("");
        while self.curr().is_whitespace() {
            s.push(*self.curr());
            self.next();
        }

        if s.len() > 0 {
            return Ok(Token::WHITESPACE);
        }

        Err(format!("whitespace was expected, got '{}'", self.curr()))
    }

    fn consume_operand(&mut self) -> Result<Operand, String> {
        let prefix = *self.curr();
        self.next();

        if prefix == '#' {
            let op = self.consume_operand()?;
            return Ok(Operand::IMM(Box::new(op)));
        }

        let s = self.consume_alphanum()?;
        match prefix {
            '$' => Ok(Operand::ADDR(s)),
            'X' => Ok(Operand::X),
            'Y' => Ok(Operand::Y),
            'A' => Ok(Operand::A),
            '(' => Ok(self.consume_wrapped_mode()?),
            _ => {
                let res = format!("{}{}", prefix, s);
                if prefix.is_alphabetic() {
                    return Ok(Operand::LABEL(res))
                }
                Err(format!("address or alphanumeric expected, got {}", res))
            }
        }
    }

    fn consume_wrapped_mode(&mut self) -> Result<Operand, String> {
        if *self.curr() != '(' {
            return Err("'(' was expected".to_string());
        }
        self.next();

        let op = self.consume_operand();
        if *self.curr() != ')' {
            return Err("')' was expected".to_string());
        }
        self.next();

        op.map(|o| Operand::ABS(Box::new(o)))
    }
}