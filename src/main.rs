use r6502::asm_lexer::AsmLexer;

fn main() {
    let mut lexer = AsmLexer::new(&String::from(r"
        ; Example program
        :label
        ; ADC   $61

        lda #$01
        sta $24
        lda #$02
        sta $25
        
        ldx #$10 ; 10-th term
        stx $26
        :start
            txa
            lda $26 ; load the first op
            adc $25 ; add the first op to the second
            sta #$24 ; store the result in $0024 to replace the old first op
            ldx $25
            inx ; new second op
            stx $25
            cpx $26
            bne start  ; USES LABEL
            brk
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
