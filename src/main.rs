use r6502::{compiler::Compiler, asm_parser::AsmParser};

fn main() {
    let mut parser = AsmParser::new(String::from(r"
        LDA #$FF    
        LDY #$09
    "));
    let res = parser.run();
    match res {
        Ok(v) => {
            for token in v.iter() {
                println!("{:?}", token);
            }
        },
        Err(e) => println!("error {}", e)
    }
}
