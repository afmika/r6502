use std::{
    path::Path, 
    collections::HashMap, 
    io::Write, 
    cell::RefCell
};

use lazy_static::__Deref;

use crate::{
    asm_parser::{
        Expr, 
        Operand, 
        Directive, 
        AsmParser
    }, 
    opcodes::{
        OPCODES, 
        AdrMode, 
        Instr, 
        Opcode
    }, 
    asm_lexer::AsmLexer
};

use std::fs;

// Examples:
// Absolute Y: AND $4400,Y consumes $44 and $00, Y is for notation
// Indirect X: AND ($44,X) consumes $44 only, X is for notation
// Zero Page, Immediate : AND $44 consumes $44 only
pub fn canonical_op_len(adr_mode: &AdrMode) -> i8 {
    match adr_mode {
        AdrMode::IMPL => 0,
        AdrMode::IMM | AdrMode::ZP | AdrMode::ZPX | AdrMode::ZPY 
        | AdrMode::INDX | AdrMode::INDY | AdrMode::REL => 1,
        AdrMode::ABS | AdrMode::ABSX | AdrMode::ABSY | AdrMode::IND => 2,
    }
}

pub fn get_opcode(
    instr: Instr, 
    mode: AdrMode, 
    config: Option<CompilerConfig>
) -> Result<Opcode, String> {
    let opcodes = OPCODES.get(&(instr.clone(), mode.clone()));
    if let Some(opcodes) = opcodes {
        if let Some(ref config) = config {
            if config.allow_illegal {
                let mut official: Option<Opcode> = None;
                for opcode in opcodes {
                    for hex in config.allow_list.borrow().deref() {
                        if opcode.hex == *hex {
                            return Ok(opcode.to_owned());
                        }
                        if opcode.official {
                            official = Some(opcode.to_owned());
                        }
                    }
                }
                // fallback to official if no hit
                if let Some(opcode) = official {
                    return Ok(opcode);
                }
            }
        } else {
            // only allow official
            for opcode in opcodes {
                if opcode.official {
                    return Ok(opcode.to_owned());
                }
            }
        }
    }
    Err(format!("instruction ({}, {:?}) does not exist", instr, mode))
}

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    /// Allow illegal opcode
    pub allow_illegal: bool,
    /// Illegal opcodes will be picked using this list as hint
    pub allow_list: RefCell<Vec<u8>>
}

pub struct Compiler {
    lines: Vec<Expr>,
    prog_counter: usize,
    label_pos: HashMap<String, isize>,
    config: Option<CompilerConfig>
}

impl Compiler {
    pub fn new(
        config: Option<CompilerConfig>
    ) -> Self {
        Self {
            lines: vec![],
            prog_counter: 0,
            label_pos: HashMap::new(),
            config
        }
    }

    pub fn init<P: AsRef<Path>>(&mut self, source_path: P) -> Result<(), String>{
        let contents = fs::read_to_string(source_path)
            .expect("unable to read source file");
        self.init_source(&contents)?;
        Ok(())
    }

    pub fn init_source(&mut self, source: &String) -> Result<(), String> {
        let mut lexer = AsmLexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = AsmParser::new(&tokens);
        self.lines = parser.parse()?;
        self.prog_counter = 0;
        Ok(())
    }

    /// Compile source code
    pub fn run<P: AsRef<Path>>(&mut self, dest: P) -> Result<(), String> {
        let bytes = self.to_byte_code()?;
        let mut file = fs::File::create(dest)
            .map_err(|e| e.to_string())?;
        file.write_all(&bytes)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Compile source code to hex string
    pub fn to_hex_string(&mut self) -> Result<String, String>{
        let buffer = self.to_byte_code()?;
        let mut out = String::new();
        for byte in buffer {
            let hex = format!("{:#04x} ", byte);
            out.push_str(hex.strip_prefix("0x").unwrap());
        }
        Ok(out.trim().to_owned())
    }

    /// Compile source code to contiguous bytes
    pub fn to_byte_code(&mut self) -> Result<Vec<u8>, String> {
        let mut program: Vec<u8> = vec![];
        self.prog_counter = 0;
        for line in &self.lines {
            match line {
                Expr::LABEL(label) => {
                    self.label_pos.insert(label.to_owned(), self.prog_counter as isize);
                },
                Expr::DIRECTIVE(directive) => {
                    match directive {
                        Directive::BYTE(seq) => {
                            for item in seq {
                                assert!(item.size == 8);
                                program.push(item.value as u8);
                            }
                        },
                        Directive::DWORD(seq) => {
                            for item in seq {
                                assert!(item.size == 16);
                                let hi = ((item.value & 0xff00) >> 8) as u8;
                                let lo = (item.value & 0x00ff) as u8;
                                // little-endian
                                program.push(lo);
                                program.push(hi);
                            }
                        },
                        Directive::ENDPROC => todo!("nes rom"),
                        Directive::PROC(_) => todo!("nes rom"),
                        Directive::SEGMENT(_) => todo!("nes rom"),
                    }
                },
                Expr::INSTR(name, mode, op) => {
                    let opcode = get_opcode(name.to_owned(), mode.to_owned(), self.config.to_owned())?;
                    program.push(opcode.hex);
                    self.prog_counter += 1; // instruction
                    match op {
                        Operand::LABEL(name) => {
                            let pos = self.label_pos.get(name);
                            if let Some(pos) = pos {
                                // [pos] .......... [pc]
                                // delta = pc - pos
                                // pc <- pc - delta = pc - (pc - pos) = pos
                                let delta = self.prog_counter as isize - pos;
                                if delta > 127 {
                                    return Err(format!("label jump too large {}({}) > 127", delta, name));
                                }
                                let hi = ((pos & 0xff00) >> 8) as u8;
                                let lo = (pos & 0x00ff) as u8;
                                // little-endian
                                program.push(lo);
                                program.push(hi);
                            } else {
                                todo!("jump ahead 128 bytes");
                            }
                        },
                        Operand::VALUE(num) => {
                            assert!(num.size == 8 || num.size == 16);
                            if num.size == 8 {
                                program.push(num.value as u8);
                            } else {
                                let hi = ((num.value & 0xff00) >> 8) as u8;
                                let lo = (num.value & 0x00ff) as u8;
                                // little-endian
                                program.push(lo);
                                program.push(hi);
                            }
                        },
                        Operand::NONE => {},
                    }
                    self.prog_counter += canonical_op_len(&mode) as usize; // operand
                },
                Expr::ASSIGN(..) => {}, // evaluated at parse time
            }
        }
        Ok(program)
    }

    pub fn get_parse_string(&self) -> String {
        self.lines
            .iter()
            .map(|v| format!("{:?}", v))
            .collect::<Vec<String>>()
            .join("\n")
    }
}