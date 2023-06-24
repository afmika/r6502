use std::cmp::min;

// https://famicom.party/book/05-6502assembly/
#[derive(Debug, Clone, Eq)]
pub enum Token {
    DIRECTIVE(String),  // .LITERAL (.segment)
    LITERAL(String),    // [\w_]+ 
    COMMENT(String),    // ;(.*)\n
    COMMA,              // ,
    COLON,              // :
    PARENTOPEN,         // (
    PARENTCLOSE,        // )
    NEWLINE,            // \n | \r\n
    HASH,               // #
    PLUS,               // -
    MINUS,              // +
    MULT,               // *
    DIV,                // /
    EQUAL,              // =
    DEC(String),        // [0-9]+
    HEX(String),        // \$[0-9abdef]+
    BIN(String),        // %[01]+
    STR(String),        // "(.*)"
    CHAR(String),        // '.{1}'
    EOF
}

impl PartialEq for Token {
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

            if self.is_literal() && !self.is_dec() && *self.curr() != '.' {
                let lit = self.consume_literal()?;
                match lit {
                    Token::LITERAL(s) => {
                        self.back(s.len());
                        prog.push(self.consume(&s, None)?);
                    },
                    _ => {}
                }
                continue;
            }

            let c = *self.curr();
            if c == ' ' || c == '\t' {
                self.consume_whitespaces()?;
                continue;
            }

            let res = match c {
                '.' => self.consume_directive(),
                ')' => self.consume(")", Some(Token::PARENTCLOSE)),
                '(' => self.consume("(", Some(Token::PARENTOPEN)),
                '#' => self.consume("#", Some(Token::HASH)),
                ',' => self.consume(",", Some(Token::COMMA)),
                ':' => self.consume(":", Some(Token::COLON)),
                '-' => self.consume("-", Some(Token::MINUS)),
                '+' => self.consume("+", Some(Token::PLUS)),
                '*' => self.consume("*", Some(Token::MULT)),
                '/' => self.consume("/", Some(Token::DIV)),
                '=' => self.consume("=", Some(Token::EQUAL)),
                ';' => self.consume_comment(),
                '\n' => self.consume_endlines(),
                '\r' => self.consume_endlines(),
                '$' => self.consume_hex(),
                '%' => self.consume_bin(),
                '"' => self.consume_string(),
                '\'' => self.consume_char(),
                '0' ..= '9' => self.consume_dec(),
                _ => {
                    return Err(format!("{:?} is not a supported character", c));
                }
            };
            prog.push(res?);
        }
        prog.push(Token::EOF);
        Ok(prog)
    }

    fn curr(&self) -> &char {
        self
            .source
            .get(self.cursor)
            .unwrap_or(&'\0')
    }

    fn is_bin(&self) -> bool {
        *self.curr() == '0' || *self.curr() == '1'
    }

    fn is_dec(&self) -> bool {
        *self.curr() >= '0' && *self.curr() <= '9'
    }

    fn is_hex(&mut self) -> bool {
        let c = *self.curr();
        c >= '0' && c <= '9' 
        || c >= 'a' && c <= 'f'
        || c >= 'A' && c <= 'F'
    }

    fn is_eof(&self) -> bool {
        *self.curr() == '\0'
    }

    fn is_endline(&self) -> bool {
        let c = *self.curr();
        self.is_eof() || c == '\n' || c == '\r'
    }

    fn is_literal(&self) -> bool {
        self.curr().is_alphanumeric() || "_.".contains(*self.curr())
    }

    fn next(&mut self) -> &char {
        self.cursor = min(self.source.len(), self.cursor + 1);
        return self.curr();
    }

    // fn prev(&mut self) -> &char {
    //     return self.back(1);
    // }

    fn back(&mut self, count: usize) -> &char {
        if self.cursor > (count - 1) {
            self.cursor -= count;
        }
        return self.curr();
    }

    fn consume(&mut self, s: &str, ret: Option<Token>) -> Result<Token, String> {
        for c in s.chars() {
            if *self.curr() != c {
                return Err(format!("{:?} was expected, got {:?}", c, *self.curr()));
            }
            self.next();
        }

        if ret.is_some() {
            return Ok(ret.unwrap());
        }
        Ok(Token::LITERAL(s.to_string()))
    }

    fn consume_directive(&mut self) -> Result<Token, String> {
        self.consume(".", None)?;
        match self.consume_literal() {
            Ok(lit) => {
                match lit {
                    Token::LITERAL(s) => Ok(Token::DIRECTIVE(s)),
                    tk =>  panic!("literal was expected after '.', got {:?}", tk)
                }
            },
            Err(e) => Err(e)
        }
    }

    fn consume_hex(&mut self) -> Result<Token, String> {
        self.consume("$", None)?;
        let mut s = String::from("");
        while self.is_hex() {
            s.push(*self.curr());
            self.next();
        }
        if s.len() == 0 || s.len() > 4 {
            return Err(format!("8 bits hex was expected, got '${}'", self.curr()));
        }
        Ok(Token::HEX(s))
    }

    fn consume_bin(&mut self) -> Result<Token, String> {
        self.consume("%", None)?;
        let mut s = String::from("");
        while self.is_bin() {
            s.push(*self.curr());
            self.next();
        }
        if s.len() == 0 || s.len() > 8 {
            return Err(format!("8 bits binary was expected, got '%{}'", self.curr()));
        }
        Ok(Token::BIN(s))
    }

    fn consume_dec(&mut self) -> Result<Token, String> {
        let mut s = String::from("");
        while self.is_dec() {
            s.push(*self.curr());
            self.next();
        }
        Ok(Token::DEC(s))
    }

    fn consume_whitespaces(&mut self) -> Result<(), String> {
        let mut ok = false;
        while *self.curr() == ' ' || *self.curr() == '\t' {
            ok = true;
            self.next();
        }
        if *self.curr() == ' ' {
            panic!("no");
        }
        if ok {
            return Ok(());
        }
        Err(format!("whitespace or newline was expected, got {:?}", self.curr()))
    }

    fn consume_endlines(&mut self) -> Result<Token, String> {
        let mut ok = false;
        while self.is_endline() {
            ok = true;
            self.next();
        }
        if ok {
            return Ok(Token::NEWLINE);
        }
        Err(format!("newline was expected, got {:?}", self.curr()))
    }

    fn consume_literal(&mut self) -> Result<Token, String> {
        let mut tk = String::from("");
        
        while self.is_literal() {
            tk.push(*self.curr());
            self.next();
        }

        if tk.len() == 0 {
            Err(format!("alphanum, _ or . was expected, got {:?}", self.curr()))
        } else {
            Ok(Token::LITERAL(tk))
        }
    }

    fn consume_string(&mut self) -> Result<Token, String> {
        self.consume("\"", None)?;
        let mut s = String::from("");
        while *self.curr() != '"' && !self.is_endline() && !self.is_eof()  {
            if *self.curr() == '\\' {
                self.next();
            }
            s.push(*self.curr());
            self.next();
        }
        self.consume("\"", None)?;
        Ok(Token::STR(s))
    }

    fn consume_char(&mut self) -> Result<Token, String> {
        self.consume("\'", None)?;
        if *self.curr() == '\\' {
            // escape
            self.next();
        }
        let value = String::from(*self.curr());
        self.next();
        self.consume("\'", None)?;
        Ok(Token::CHAR(value))
    }

    fn consume_comment(&mut self) -> Result<Token, String> {
        self.consume(";", None)?;

        let mut tk = String::from("");

        while !self.is_endline() && !self.is_eof() {
            tk.push(*self.curr());
            self.next();
        }

        Ok(Token::COMMENT(tk))
    }

}