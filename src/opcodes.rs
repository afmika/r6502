use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]

pub enum AdrMode {
    IMPL, IMM, ABS,
    ABSX, ABSY,
    ZP, ZPX, ZPY,
    IND, INDX, INDY,
    REL
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    LDA, LDX, LDY, 
    STA, STX, STY,
    TAX, TAY, TSX, TXA, TXS, TYA,
    PHA, PHP, PLA, PLP, 
    DEC, DEX, DEY, 
    INC, INX, INY, 
    ADC, SBC, 
    AND, EOR, ORA, 
    ASL, LSR, 
    ROL, ROR, 
    CLC, CLD, CLI, CLV, 
    SEC, SED, SEI, 
    CMP, CPX, CPY,
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS, 
    JMP, JSR, RTS, 
    BRK, RTI, 
    BIT, NOP
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Opcode {
    instr: Instr,
    adr_mode: AdrMode,
}

impl Opcode {
    // Absolute Y: AND $4400,Y consumes $44 and $00, Y is for notation
    // Indirect X: AND ($44,X) consumes $44 only, X is for notation
    // Zero Page, Immediate : AND $44 consumes $44 only
    pub fn canonical_op_len(&self) -> u8 {
        match self.adr_mode {
            AdrMode::IMPL => 0,
            AdrMode::IMM | AdrMode::ZP | AdrMode::ZPX | AdrMode::ZPY 
            | AdrMode::INDX | AdrMode::INDY | AdrMode::REL => 1,
            AdrMode::ABS | AdrMode::ABSX | AdrMode::ABSY | AdrMode::IND => 2,
        }
    }

    fn new(instr: Instr, adr_mode: AdrMode) -> Self {
        Self {
            instr,
            adr_mode,
        }
    }
}

lazy_static! {
    #[rustfmt::skip]
    pub static ref OPCODES: HashMap<Opcode, u8> = HashMap::from([
        // todo
        (Opcode::new(Instr::BIT, AdrMode::ZP), 0x24),
    ]);
}