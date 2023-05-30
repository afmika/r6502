use std::cmp::min;

#[derive(Debug, Clone, Eq)]
pub enum Token {
    LITERAL(String),    // [\w]+ 
    WHITESPACE,         // \s | \t | \n
    COMM(String),       // ;(.*)
    LABEL(String),      // :[\w]+
    OP(Operand),
    EOF
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

#[derive(Debug, Clone, Eq)]
pub enum Operand {
    LITERAL(String),    // (.*)
    HEX(String),        // $XXXX
    IMM(Box<Operand>),  // #value, #$HEX
    ABS(Box<Operand>),  // ( (\$|#|$)[0-9]{2,8} | label )
    PAIR(Box<Operand>, Box<Operand>)
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
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
            if self.is_eof() {
                break;
            }

            let c = *self.curr();

            if c.is_whitespace() {
                self.consume_whitespace()?;
                continue;
            }

            if c == ';' {
                self.consume_comment()?;
                continue;
            }

            // label, instruction
            if c.is_alphanumeric() {
                let tk = self.consume_literal()?;
                if *self.curr() == ':' {
                    self.consume(":")?;
                    match tk {
                        Token::LITERAL(s) => {
                            prog.push(Token::LABEL(s))
                        },
                        _ => {}
                    }
                } else {
                    prog.push(tk);
                }
                continue;
            }

            // operands
            prog.push(self.consume_right_side()?);
        }
        prog.push(Token::EOF);
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
        self.cursor = min(self.source.len(), self.cursor + 1);
        return self.curr();
    }

    fn prev(&mut self) -> &char {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        return self.curr();
    }

    fn consume(&mut self, s: &str) -> Result<Token, String> {
        for c in s.chars() {
            if *self.curr() != c {
                return Err(format!("'{}' was expected, got '{}'", c, *self.curr()));
            }
            self.next();
        }
        Ok(Token::LITERAL(s.to_string()))
    }

    fn consume_literal(&mut self) -> Result<Token, String> {
        self.consume_alphanum()
            .map(|tk| Token::LITERAL(tk))
    }

    fn consume_right_side(&mut self) -> Result<Token, String> {
        match self.consume_operand() {
            Ok(left) => {
                self.consume_whitespace().ok();
                if *self.curr() == ',' {
                    self.consume(",")?;
                    self.consume_whitespace().ok();
                    let right = self.consume_operand()?;
                    let pairs = Operand::PAIR(
                        Box::new(left), 
                        Box::new(right)
                    );
                    self.consume_whitespace().ok();
                    self.consume_comment().ok();
                    return Ok(Token::OP(pairs));
                }
                return Ok(Token::OP(left));
            },
            Err(e) => Err(e)
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
        
        while self.curr().is_alphanumeric() || "_.".contains(*self.curr()) {
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

        Err(format!("whitespace or newline was expected, got '{}'", self.curr()))
    }

    fn consume_operand(&mut self) -> Result<Operand, String> {
        let prefix = *self.curr();
        self.next();

        if prefix == '#' {
            let op = self.consume_operand()?;
            return Ok(Operand::IMM(Box::new(op)));
        }

        if prefix == '(' {
            self.prev();
            return Ok(self.consume_wrapped_mode()?);
        }

        if prefix == '$' {
            return Ok(Operand::HEX(self.consume_alphanum()?));
        }
        
        // unconsume prefix
        self.prev();

        if prefix.is_alphanumeric() {
            return Ok(Operand::LITERAL(self.consume_alphanum()?))
        }
        Err(format!("HEX or alphanumeric expected, got '{}'", prefix))
    }

    fn consume_wrapped_mode(&mut self) -> Result<Operand, String> {
        self.consume("(")?;
        let op = self.consume_operand();
        self.consume(")")?;

        op.map(|o| Operand::ABS(Box::new(o)))
    }
}