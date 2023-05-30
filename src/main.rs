use r6502::asm_lexer::AsmLexer;

fn main() {
    let mut lexer = AsmLexer::new(&String::from(r"
    ; Example program
    start:
        INSTR_A (#$ABC), $F
        INSTR_B
        INSTR_C X
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
