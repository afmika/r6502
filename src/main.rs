use r6502::asm_lexer::AsmLexer;

fn main() {
    let mut lexer = AsmLexer::new(&String::from(r"
    ; Example program
    :label
    ; ADC   $61
    lda             #$01
    sta $24
    lda #$02  ; some comment
    sta $25
    lda A
    "));
    let res = lexer.tokenize();
    match res {
        Ok(v) => {
            for token in v.iter() {
                println!("{:?}", token);
            }
            println!("Done");
        },
        Err(e) => println!("error {}", e)
    }
}
