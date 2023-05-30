#[cfg(test)]
mod tests {
    use crate::asm_lexer::{AsmLexer, Operand, Token};

    #[test]
    fn simple_lexing() {
        let mut lexer = AsmLexer::new(&String::from(r"
            ; Example program
            start:
                INSTR_A (#$ABC), F
                INSTR_B
                INSTR_C X
        "));
        let res = lexer.tokenize();
        let tokens = vec![
            Token::LABEL("start".to_string()),
            Token::LITERAL("INSTR_A".to_string()),
            Token::OP(
                Operand::PAIR(
                    Box::new(
                            Operand::ABS(
                                Box::new(Operand::IMM(
                                    Box::new(Operand::HEX("ABC".to_string()))
                                )
                            )
                        )
                    ), 
                    Box::new(Operand::LITERAL("F".to_string()))
                )
            ),
            Token::LITERAL("INSTR_B".to_string()),
            Token::LITERAL("INSTR_C".to_string()),
            Token::LITERAL("X".to_string()),
            Token::EOF
        ];
        assert_eq!(res.unwrap(), tokens);
    }

    #[test]
    fn complex_lexing() {
        let mut lexer = AsmLexer::new(&String::from(r"
            ; Example program
            label:
            ; ADC   $61
            lda             #$01
            sta $24
            lda #$02  ; some comment
            sta $25
            lda X
            sta ($foo), A 
            ; yet another comment
        "));
        let res = lexer.tokenize();

        let tokens = vec![
            Token::LABEL("label".to_string()),

            Token::LITERAL("lda".to_string()), 
            Token::OP(Operand::IMM(Box::new(Operand::HEX("01".to_string())))), 
            
            Token::LITERAL("sta".to_string()), 
            Token::OP(Operand::HEX("24".to_string())), 

            Token::LITERAL("lda".to_string()), 
            Token::OP(Operand::IMM(Box::new(Operand::HEX("02".to_string())))), 

            Token::LITERAL("sta".to_string()), Token::OP(Operand::HEX("25".to_string())),

            Token::LITERAL("lda".to_string()), 
            Token::LITERAL("X".to_string()),

            Token::LITERAL("sta".to_string()),
            Token::OP(
                Operand::PAIR(
                    Box::new(Operand::ABS(Box::new(Operand::HEX("foo".to_string())))), 
                    Box::new(Operand::LITERAL("A".to_string()))
                )
            ),

            Token::EOF
        ];

        assert_eq!(res.unwrap(), tokens);
    }
}