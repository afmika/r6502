use std::cell::RefCell;

use crate::compiler::{Compiler, CompilerConfig};

#[test]
fn simple_compilation() {
    let source =String::from(r##"
        ; should be ignored
        also_ignored:
        ignored = 1 + %101 * ($ff - 3)  
        .byte "HELLO WORLD"                 ; also ignored
        .dword "LLHH", $00ff
        LDA ($ff), y                        ;  official
        NOP
    "##);
    let mut compiler = Compiler::new(None);
    compiler.init_source(&source).unwrap();
    let hex_string = compiler.to_hex_string().unwrap();
    let bytes = compiler.to_byte_code().unwrap();
    assert_eq!(hex_string, "48 45 4c 4c 4f 20 57 4f 52 4c 44 4c 4c 48 48 ff 00 b1 ff ea");
    assert_eq!(bytes, vec![
        72, 69, 76, 76, 79, 32, 87, 79, 82, 76, 68, 76, 76, 72, 72, 255, 0, 177, 255, 234
    ]);
}


#[test]
fn compile_illegal() {
    let source =String::from(r##"
        x = %010
        LAX #$a                        ; non official
        LDA ($f0 + (x * 8 - $1)), y    ; official
        NOP                            ; official, but let's emit non-official opcode
    "##);
    let mut compiler = Compiler::new(Some(CompilerConfig {
        allow_illegal: true,
        allow_list: RefCell::new(vec![
            0xDA // non official op
        ])
    }));
    compiler.init_source(&source).unwrap();
    let hex_string = compiler.to_hex_string().unwrap();
    let bytes = compiler.to_byte_code().unwrap();
    assert_eq!(hex_string, "ab 0a b1 ff da");
    assert_eq!(bytes, vec![
        171, 10, 177, 255, 218
    ]);
}