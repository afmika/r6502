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
    let mut lexer = AsmLexer::new(&String::from(r##"
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

        hello:
        LDX #(1 + 2 / (3 - 1))

        BNE hello
        two = 3
        one = two + $0002

        LDX #(1 + 2 * one)

        bmi $BB

        y = 12645768
        .byte "AB", 67, y
        .db "DE"
        .dword "AB", $ffff, 'A', 'A'+3, y
"##
        )
    );
    let res = lexer.tokenize();
    let tokens = res.unwrap();
    let mut parser = AsmParser::new(&tokens);
    let prog = parser.parse().unwrap();
    for i in prog {
        println!("{:?}", i);
    }
}

fn main() {
    // sample_test();
    parser_test();
}
