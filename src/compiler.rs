use std::{path::PathBuf, rc::Rc, cmp::min};
use crate::asm_lexer::Token;

pub struct Compiler {
    tokens: Rc<Vec<Token>>,
    cursor: usize
}

impl Compiler {
    pub fn new(tokens: Rc<Vec<Token>>) -> Self {
        Self {
            tokens,
            cursor: 0
        }
    }

    pub fn toByteCode(&mut self) -> Vec<u8> {
        loop {
            break;
        }
        vec![]
    }

    fn is_eof(&mut self) -> bool {
        *self.curr() == Token::EOF
    }

    fn curr(&mut self) -> &Token {
        self
            .tokens
            .get(self.cursor)
            .unwrap_or(&Token::EOF)
    }

    fn next(&mut self) -> &Token {
        self.cursor = min(self.tokens.len(), self.cursor + 1);
        return self.curr();
    }

    fn prev(&mut self) -> &Token {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        return self.curr();
    }

    pub fn write(path: PathBuf) {
        println!("{} bytes written to {:?}", 0, path);
    }
}