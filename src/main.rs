use r6502::{
    asm_lexer::{
        AsmLexer, 
        Token
    }, asm_parser::AsmParser
};

fn sample_test() {
    let mut lexer = AsmLexer::new(&String::from(r"
        ;some comments
        LDX #0
        .start
        .proc
        LOOP:
            LDA #$FE ; another comment
            STA $200
            STA 1234
            INX 
            INX2
            CPX %123 ; another
            BEQ LOOP

        : LDA #12
        +
        -
        =
        .endproc
        "
        )
    );
    let res = lexer.tokenize();
    match res {
        Ok(v) => {
            for token in v.iter() {
                print!(" {:?}", token);
                match token {
                    Token::NEWLINE => {
                        println!() 
                    },
                    _ => {},
                }
            }
        },
        Err(e) => println!("error {}", e)
    }
}

fn parser_test() {
    let mut lexer = AsmLexer::new(&String::from(r"
        LDA ($0FF) 
        LDA
        LDX ($0FF)
        LDX ($FF), y
        LDX ($FF, x)
        LDA
        LDX #$FF
        LDA
        LDA $00FF, y
        LDA $00FF, x
        LDA $00FF

        LDA $FF, y
        LDA $FF, x
        LDA $FF

        LDX #(1 + 2 / (3 - variable))

        BNE hello
        BMI #$BB

"
        )
    );
    let res = lexer.tokenize();
    let tokens = res.unwrap();
    let mut parser = AsmParser::new(&tokens);
    parser.parse().unwrap();
}

fn main() {
    // sample_test();
    parser_test();
}
