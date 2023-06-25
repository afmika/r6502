#[cfg(test)]
mod tests {
    use crate::asm_lexer::{AsmLexer, Token};
    use crate::asm_parser::{AsmParser, Expr, Directive, Operand, MathExpr, NumericValue};
    use crate::compiler::Compiler;
    use crate::opcodes::{Instr, AdrMode};

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
            Token::COMMENT(" Example program".to_owned()), Token::NEWLINE,
            Token::LITERAL("start".to_owned()), Token::COLON, Token::NEWLINE,
            
            Token::LITERAL("INSTR_A".to_owned()), Token::PARENTOPEN, 
            Token::HASH, Token::HEX("ABC".to_owned()), Token::PARENTCLOSE, Token::COMMA, 
            Token::LITERAL("F".to_owned()), Token::NEWLINE,

            Token::LITERAL("INSTR_B".to_owned()), Token::NEWLINE,
            Token::LITERAL("INSTR_C".to_owned()), Token::BIN("001".to_owned()), Token::NEWLINE,

            Token::LITERAL("INSTR_D".to_owned()), Token::DEC("123456".to_owned()), Token::NEWLINE,
            Token::LITERAL("INSTR_E".to_owned()), Token::BIN("0".to_owned()),
            Token::EOF
        ];
        assert_eq!(res.unwrap(), tokens);
    }

    #[test]
    fn basic_lexing() {
        let mut lexer = AsmLexer::new(&String::from(r##"
            .proc .procend   .main some._literal ; then comments
            $ff01 %00010 , "a \"string\"" 'a' .lit : 'b' '\''
        "##));
        let res = lexer.tokenize();

        let tokens = vec![
            Token::DIRECTIVE("proc".to_owned()),
            Token::DIRECTIVE("procend".to_owned()), Token::DIRECTIVE("main".to_owned()), 
            Token::LITERAL("some._literal".to_owned()), Token::COMMENT(" then comments".to_owned()), Token::NEWLINE,
            Token::HEX("ff01".to_owned()), Token::BIN("00010".to_owned()), Token::COMMA, 
            Token::STR("a \"string\"".to_owned()),
            Token::CHAR("a".to_owned()), Token::DIRECTIVE("lit".to_owned()), Token::COLON, 
            Token::CHAR("b".to_owned()), Token::CHAR("\'".to_owned()),
            Token::EOF
        ];

        assert_eq!(res.unwrap(), tokens);
    }

    #[test]
    fn simple_parsing() {
        let mut lexer = AsmLexer::new(&String::from(r##"
            ; Example program
            .segment "SOME SEGMENT"
            x = 1 + 2 * 3
            .byte "ABC", 'C' + 1, %001 + 2*x
            .dw "CD"
            y = 2*(x-$a)+%1
            start:
            BNE start
        "##));
        let res = lexer.tokenize();
        let tokens = res.unwrap();
        let mut parser = AsmParser::new(&tokens);
        let prog = parser.parse();
        let lines = vec![
            Expr::DIRECTIVE(Directive::SEGMENT("SOME SEGMENT".to_owned())), 
            Expr::ASSIGN(
                "x".to_owned(), 
                MathExpr::BIN(
                    Token::PLUS, 
                    Box::new(MathExpr::NUM(NumericValue { value: 1, size: 8 })), 
                    Box::new(MathExpr::BIN(
                        Token::MULT, 
                        Box::new(MathExpr::NUM(NumericValue { value: 2, size: 8 })), 
                        Box::new(MathExpr::NUM(NumericValue { value: 3, size: 8 }))
                    ))
                )
            ), 
            Expr::DIRECTIVE(
                Directive::BYTE(vec![
                    NumericValue { value: 65, size: 8 }, 
                    NumericValue { value: 66, size: 8 }, 
                    NumericValue { value: 67, size: 8 }, 
                    NumericValue { value: 68, size: 8 }, 
                    NumericValue { value: 15, size: 8 }
                ])
            ), 
            Expr::DIRECTIVE(
                Directive::DWORD(vec![
                    NumericValue { value: 17220, size: 16 }
                ])
            ), 
            Expr::ASSIGN(
                "y".to_owned(), 
                MathExpr::BIN(
                    Token::PLUS, 
                    Box::new(
                        MathExpr::BIN(
                            Token::MULT, 
                            Box::new(MathExpr::NUM(NumericValue { value: 2, size: 8 })), 
                            Box::new(MathExpr::BIN(
                                Token::MINUS, 
                                Box::new(MathExpr::PLACEHOLDER("x".to_owned())), 
                                Box::new(MathExpr::NUM(NumericValue { value: 10, size: 8 })))
                            ))), 
                            Box::new(MathExpr::NUM(NumericValue { value: 1, size: 8 })
                        )
                    )), 
            Expr::LABEL("start".to_owned()), 
            Expr::INSTR(Instr::BNE, AdrMode::REL, Operand::LABEL("start".to_owned()))
        ];
        assert_eq!(prog.unwrap(), lines);
    }

    #[test]
    fn simple_compilation() {
        let source =String::from(r##"
            ; should be ignored
            also_ignored:
            ignored = 1 + %101 * ($ff - 3)  
            .byte "HELLO WORLD" ; also ignored
            .dword "LLHH", $00ff
            LDA #$a
        "##);
        let mut compiler = Compiler::new(None);
        compiler.init_source(&source).unwrap();
        let hex_string = compiler.to_hex_string().unwrap();
        let bytes = compiler.to_byte_code().unwrap();
        assert_eq!(hex_string, "48 45 4c 4c 4f 20 57 4f 52 4c 44 4c 4c 48 48 ff 00 a9 0a");
        assert_eq!(bytes, vec![
            72, 69, 76, 76, 79, 32, 87, 79, 82, 76, 68, 76, 76, 72, 72, 255, 0, 169, 10
        ]);
    }
}