#[cfg(test)]
mod tests {
    use crate::asm_lexer::{AsmLexer, Token};

    #[test]
    fn simple_lexing() {
        let mut lexer = AsmLexer::new(&String::from(r"
            ; Example program
            start:
                INSTR_A (#$ABC), F
                INSTR_B
                INSTR_C %001
                INSTR_D 123456
                INSTR_E %0
        "));
        let res = lexer.tokenize();
        let tokens = vec![
            Token::COMMENT(" Example program".to_string()), Token::NEWLINE,
            Token::LITERAL("start".to_string()), Token::COLON, Token::NEWLINE,
            
            Token::LITERAL("INSTR_A".to_string()), Token::PARENTOPEN, 
            Token::HASH, Token::HEX("ABC".to_string()), Token::PARENTCLOSE, Token::COMMA, 
            Token::LITERAL("F".to_string()), Token::NEWLINE,

            Token::LITERAL("INSTR_B".to_string()), Token::NEWLINE,
            Token::LITERAL("INSTR_C".to_string()), Token::BIN("001".to_string()), Token::NEWLINE,

            Token::LITERAL("INSTR_D".to_string()), Token::DEC("123456".to_string()), Token::NEWLINE,
            Token::LITERAL("INSTR_E".to_string()), Token::BIN("0".to_string()),
            Token::EOF
        ];
        assert_eq!(res.unwrap(), tokens);
    }

    #[test]
    fn basic_lexing() {
        let mut lexer = AsmLexer::new(&String::from(r"
            .proc .procend   .main some._literal ; then comments
            $ff01 %00010 ,   :
        "));
        let res = lexer.tokenize();

        let tokens = vec![
            Token::DIRECTIVE("proc".to_string()),
            Token::DIRECTIVE("procend".to_string()), Token::DIRECTIVE("main".to_string()), 
            Token::LITERAL("some._literal".to_string()), Token::COMMENT(" then comments".to_string()), Token::NEWLINE,
            Token::HEX("ff01".to_string()), Token::BIN("00010".to_string()), Token::COMMA, Token::COLON,
            Token::EOF
        ];

        assert_eq!(res.unwrap(), tokens);
    }
}