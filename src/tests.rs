#[cfg(test)]
mod tests {
    use crate::asm_lexer::{AsmLexer, Operand, Token};

    #[test]
    fn simple_lexing() {
        let mut lexer = AsmLexer::new(&String::from(r"
            ; Example program
            :label
            ; ADC   $61
            lda             #$01
            sta $24
            lda #$02  ; some comment
            sta $25
            ; lda X
            ; yet another comment
        "));
        let res = lexer.tokenize();

        let tokens = vec![
            Token::LABEL("label".to_string()),

            Token::LITERAL("lda".to_string()), 
            Token::OP(Operand::IMM(Box::new(Operand::ADDR("01".to_string())))), 
            
            Token::LITERAL("sta".to_string()), 
            Token::OP(Operand::ADDR("24".to_string())), 

            Token::LITERAL("lda".to_string()), 
            Token::OP(Operand::IMM(Box::new(Operand::ADDR("02".to_string())))), 

            Token::LITERAL("sta".to_string()), Token::OP(Operand::ADDR("25".to_string())),

            // Token::LITERAL("lda".to_string()), 
            // Token::OP(Operand::X), 
            Token::EOF
        ];

        assert_eq!(res.unwrap(), tokens);
    }
}