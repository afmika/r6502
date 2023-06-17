use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum AdrMode {
    IMPL, IMM, ABS,
    ABSX, ABSY,
    ZP, ZPX, ZPY,
    IND, INDX, INDY,
    REL, 
    UNKNOWN // fallback for math expr
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
    BIT, NOP,

    // unofficial instructions
    STP, SLO, ANC, RLA, SRE, ALR, RRA, ARR, SAX,
    XAA, AHX, TAS, SHY, SHX, LAX, LAS, DCP, AXS,
    ISC
}

// Absolute Y: AND $4400,Y consumes $44 and $00, Y is for notation
// Indirect X: AND ($44,X) consumes $44 only, X is for notation
// Zero Page, Immediate : AND $44 consumes $44 only
pub fn canonical_op_len(adr_mode: &AdrMode) -> i8 {
    match adr_mode {
        AdrMode::UNKNOWN => -1,
        AdrMode::IMPL => 0,
        AdrMode::IMM | AdrMode::ZP | AdrMode::ZPX | AdrMode::ZPY 
        | AdrMode::INDX | AdrMode::INDY | AdrMode::REL => 1,
        AdrMode::ABS | AdrMode::ABSX | AdrMode::ABSY | AdrMode::IND => 2,
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Opcode {
    hex: u8,
    examples: Vec<String>
}

impl Opcode {
    fn new(hex: u8, examples: Vec<String>) -> Self {
        Self {
            hex,
            examples
        }
    }
}


// https://github.com/afmika/opcodes-json-6502
lazy_static! {
    pub static ref INSTR: HashMap<String, Instr> = HashMap::from([
        ("LDA".to_string(), Instr::LDA),
        ("LDX".to_string(), Instr::LDX),
        ("LDY".to_string(), Instr::LDY),
        ("STA".to_string(), Instr::STA),
        ("STX".to_string(), Instr::STX),
        ("STY".to_string(), Instr::STY),
        ("TAX".to_string(), Instr::TAX),
        ("TAY".to_string(), Instr::TAY),
        ("TSX".to_string(), Instr::TSX),
        ("TXA".to_string(), Instr::TXA),
        ("TXS".to_string(), Instr::TXS),
        ("TYA".to_string(), Instr::TYA),
        ("PHA".to_string(), Instr::PHA),
        ("PHP".to_string(), Instr::PHP),
        ("PLA".to_string(), Instr::PLA),
        ("PLP".to_string(), Instr::PLP),
        ("DEC".to_string(), Instr::DEC),
        ("DEX".to_string(), Instr::DEX),
        ("DEY".to_string(), Instr::DEY),
        ("INC".to_string(), Instr::INC),
        ("INX".to_string(), Instr::INX),
        ("INY".to_string(), Instr::INY),
        ("ADC".to_string(), Instr::ADC),
        ("SBC".to_string(), Instr::SBC),
        ("AND".to_string(), Instr::AND),
        ("EOR".to_string(), Instr::EOR),
        ("ORA".to_string(), Instr::ORA),
        ("ASL".to_string(), Instr::ASL),
        ("LSR".to_string(), Instr::LSR),
        ("ROL".to_string(), Instr::ROL),
        ("ROR".to_string(), Instr::ROR),
        ("CLC".to_string(), Instr::CLC),
        ("CLD".to_string(), Instr::CLD),
        ("CLI".to_string(), Instr::CLI),
        ("CLV".to_string(), Instr::CLV),
        ("SEC".to_string(), Instr::SEC),
        ("SED".to_string(), Instr::SED),
        ("SEI".to_string(), Instr::SEI),
        ("CMP".to_string(), Instr::CMP),
        ("CPX".to_string(), Instr::CPX),
        ("CPY".to_string(), Instr::CPY),
        ("BCC".to_string(), Instr::BCC),
        ("BCS".to_string(), Instr::BCS),
        ("BEQ".to_string(), Instr::BEQ),
        ("BMI".to_string(), Instr::BMI),
        ("BNE".to_string(), Instr::BNE),
        ("BPL".to_string(), Instr::BPL),
        ("BVC".to_string(), Instr::BVC),
        ("BVS".to_string(), Instr::BVS),
        ("JMP".to_string(), Instr::JMP),
        ("JSR".to_string(), Instr::JSR),
        ("RTS".to_string(), Instr::RTS),
        ("BRK".to_string(), Instr::BRK),
        ("RTI".to_string(), Instr::RTI),
        ("BIT".to_string(), Instr::BIT),
        ("NOP".to_string(), Instr::NOP),
        ("STP".to_string(), Instr::STP),
        ("SLO".to_string(), Instr::SLO),
        ("ANC".to_string(), Instr::ANC),
        ("RLA".to_string(), Instr::RLA),
        ("SRE".to_string(), Instr::SRE),
        ("ALR".to_string(), Instr::ALR),
        ("RRA".to_string(), Instr::RRA),
        ("ARR".to_string(), Instr::ARR),
        ("SAX".to_string(), Instr::SAX),
        ("XAA".to_string(), Instr::XAA),
        ("AHX".to_string(), Instr::AHX),
        ("TAS".to_string(), Instr::TAS),
        ("SHY".to_string(), Instr::SHY),
        ("SHX".to_string(), Instr::SHX),
        ("LAX".to_string(), Instr::LAX),
        ("LAS".to_string(), Instr::LAS),
        ("DCP".to_string(), Instr::DCP),
        ("AXS".to_string(), Instr::AXS),
        ("ISC".to_string(), Instr::ISC)
    ]);

    pub static ref OPCODES: HashMap<(Instr, AdrMode), Vec<Opcode>> = HashMap::from([
        ((Instr::BRK, AdrMode::IMPL), vec![Opcode::new(0x00, vec!["BRK".to_string()])]),
        ((Instr::ORA, AdrMode::INDX), vec![Opcode::new(0x01, vec!["ORA ($44,X)".to_string()])]),
        ((Instr::ORA, AdrMode::ZP), vec![Opcode::new(0x05, vec!["ORA $44".to_string()])]),
        ((Instr::ORA, AdrMode::IMM), vec![Opcode::new(0x09, vec!["ORA #$44".to_string()])]),
        ((Instr::ORA, AdrMode::ABS), vec![Opcode::new(0x0D, vec!["ORA $4400".to_string()])]),
        ((Instr::ORA, AdrMode::INDY), vec![Opcode::new(0x11, vec!["ORA ($44),Y".to_string()])]),
        ((Instr::ORA, AdrMode::ZPX), vec![Opcode::new(0x15, vec!["ORA $44,X".to_string()])]),
        ((Instr::ORA, AdrMode::ABSY), vec![Opcode::new(0x19, vec!["ORA $4400,Y".to_string()])]),
        ((Instr::ORA, AdrMode::ABSX), vec![Opcode::new(0x1D, vec!["ORA $4400,X".to_string()])]),
        ((Instr::STP, AdrMode::IMPL), vec![
            Opcode::new(0x02, vec!["STP".to_string()]),
            Opcode::new(0x12, vec!["STP".to_string()]),
            Opcode::new(0x22, vec!["STP".to_string()]),
            Opcode::new(0x32, vec!["STP".to_string()]),
            Opcode::new(0x42, vec!["STP".to_string()]),
            Opcode::new(0x52, vec!["STP".to_string()]),
            Opcode::new(0x62, vec!["STP".to_string()]),
            Opcode::new(0x72, vec!["STP".to_string()]),
            Opcode::new(0x92, vec!["STP".to_string()]),
            Opcode::new(0xB2, vec!["STP".to_string()]),
            Opcode::new(0xD2, vec!["STP".to_string()]),
            Opcode::new(0xF2, vec!["STP".to_string()])]),
        ((Instr::SLO, AdrMode::INDX), vec![Opcode::new(0x03, vec!["SLO ($44,X)".to_string()])]),
        ((Instr::SLO, AdrMode::ZP), vec![Opcode::new(0x07, vec!["SLO $44".to_string()])]),
        ((Instr::SLO, AdrMode::ABS), vec![Opcode::new(0x0F, vec!["SLO $4400".to_string()])]),
        ((Instr::SLO, AdrMode::INDY), vec![Opcode::new(0x13, vec!["SLO ($44),Y".to_string()])]),
        ((Instr::SLO, AdrMode::ZPX), vec![Opcode::new(0x17, vec!["SLO $44,X".to_string()])]),
        ((Instr::SLO, AdrMode::ABSY), vec![Opcode::new(0x1B, vec!["SLO $4400,Y".to_string()])]),
        ((Instr::SLO, AdrMode::ABSX), vec![Opcode::new(0x1F, vec!["SLO $4400,X".to_string()])]),
        ((Instr::NOP, AdrMode::ZP), vec![Opcode::new(0x04, vec!["NOP $44".to_string()]),
            Opcode::new(0x44, vec!["NOP $44".to_string()]),
            Opcode::new(0x64, vec!["NOP $44".to_string()])]),
        ((Instr::NOP, AdrMode::ABS), vec![Opcode::new(0x0C, vec!["NOP $4400".to_string()])]),
        ((Instr::NOP, AdrMode::ZPX), vec![
            Opcode::new(0x14, vec!["NOP $44,X".to_string()]),
            Opcode::new(0x34, vec!["NOP $44,X".to_string()]),
            Opcode::new(0x54, vec!["NOP $44,X".to_string()]),
            Opcode::new(0x74, vec!["NOP $44,X".to_string()]),
            Opcode::new(0xD4, vec!["NOP $44,X".to_string()]),
            Opcode::new(0xF4, vec!["NOP $44,X".to_string()])]),
        ((Instr::NOP, AdrMode::IMPL), vec![
            Opcode::new(0x1A, vec!["NOP".to_string()]),
            Opcode::new(0x3A, vec!["NOP".to_string()]),
            Opcode::new(0x5A, vec!["NOP".to_string()]),
            Opcode::new(0x7A, vec!["NOP".to_string()]),
            Opcode::new(0xDA, vec!["NOP".to_string()]),
            Opcode::new(0xEA, vec!["NOP".to_string()]),
            Opcode::new(0xFA, vec!["NOP".to_string()])]),
        ((Instr::NOP, AdrMode::ABSX), vec![
            Opcode::new(0x1C, vec!["NOP $4400,X".to_string()]),
            Opcode::new(0x3C, vec!["NOP $4400,X".to_string()]),
            Opcode::new(0x5C, vec!["NOP $4400,X".to_string()]),
            Opcode::new(0x7C, vec!["NOP $4400,X".to_string()]),
            Opcode::new(0xDC, vec!["NOP $4400,X".to_string()]),
            Opcode::new(0xFC, vec!["NOP $4400,X".to_string()])]),
        ((Instr::NOP, AdrMode::IMM), vec![
            Opcode::new(0x80, vec!["NOP #$44".to_string()]),
            Opcode::new(0x82, vec!["NOP #$44".to_string()]),
            Opcode::new(0x89, vec!["NOP #$44".to_string()]),
            Opcode::new(0xC2, vec!["NOP #$44".to_string()]),
            Opcode::new(0xE2, vec!["NOP #$44".to_string()])]),
        ((Instr::ASL, AdrMode::ZP), vec![Opcode::new(0x06, vec!["ASL $44".to_string()])]),
        ((Instr::ASL, AdrMode::IMPL), vec![Opcode::new(0x0A, vec!["ASL".to_string()])]),
        ((Instr::ASL, AdrMode::ABS), vec![Opcode::new(0x0E, vec!["ASL $4400".to_string()])]),
        ((Instr::ASL, AdrMode::ZPX), vec![Opcode::new(0x16, vec!["ASL $44,X".to_string()])]),
        ((Instr::ASL, AdrMode::ABSX), vec![Opcode::new(0x1E, vec!["ASL $4400,X".to_string()])]),
        ((Instr::PHP, AdrMode::IMPL), vec![Opcode::new(0x08, vec!["PHP".to_string()])]),
        ((Instr::ANC, AdrMode::IMM), vec![
            Opcode::new(0x0B, vec!["ANC #$44".to_string()]),
            Opcode::new(0x2B, vec!["ANC #$44".to_string()])]),
        ((Instr::BPL, AdrMode::REL), vec![Opcode::new(0x10, vec!["BPL $10".to_string(), "BPL label".to_string()])]),
        ((Instr::CLC, AdrMode::IMPL), vec![Opcode::new(0x18, vec!["CLC".to_string()])]),
        ((Instr::JSR, AdrMode::ABS), vec![Opcode::new(0x20, vec!["JSR $4400".to_string()])]),
        ((Instr::AND, AdrMode::INDX), vec![Opcode::new(0x21, vec!["AND ($44,X)".to_string()])]),
        ((Instr::AND, AdrMode::ZP), vec![Opcode::new(0x25, vec!["AND $44".to_string()])]),
        ((Instr::AND, AdrMode::IMM), vec![Opcode::new(0x29, vec!["AND #$44".to_string()])]),
        ((Instr::AND, AdrMode::ABS), vec![Opcode::new(0x2D, vec!["AND $4400".to_string()])]),
        ((Instr::AND, AdrMode::INDY), vec![Opcode::new(0x31, vec!["AND ($44),Y".to_string()])]),
        ((Instr::AND, AdrMode::ZPX), vec![Opcode::new(0x35, vec!["AND $44,X".to_string()])]),
        ((Instr::AND, AdrMode::ABSY), vec![Opcode::new(0x39, vec!["AND $4400,Y".to_string()])]),
        ((Instr::AND, AdrMode::ABSX), vec![Opcode::new(0x3D, vec!["AND $4400,X".to_string()])]),
        ((Instr::RLA, AdrMode::INDX), vec![Opcode::new(0x23, vec!["RLA ($44,X)".to_string()])]),
        ((Instr::RLA, AdrMode::ZP), vec![Opcode::new(0x27, vec!["RLA $44".to_string()])]),
        ((Instr::RLA, AdrMode::ABS), vec![Opcode::new(0x2F, vec!["RLA $4400".to_string()])]),
        ((Instr::RLA, AdrMode::INDY), vec![Opcode::new(0x33, vec!["RLA ($44),Y".to_string()])]),
        ((Instr::RLA, AdrMode::ZPX), vec![Opcode::new(0x37, vec!["RLA $44,X".to_string()])]),
        ((Instr::RLA, AdrMode::ABSY), vec![Opcode::new(0x3B, vec!["RLA $4400,Y".to_string()])]),
        ((Instr::RLA, AdrMode::ABSX), vec![Opcode::new(0x3F, vec!["RLA $4400,X".to_string()])]),
        ((Instr::BIT, AdrMode::ZP), vec![Opcode::new(0x24, vec!["BIT $44".to_string()])]),
        ((Instr::BIT, AdrMode::ABS), vec![Opcode::new(0x2C, vec!["BIT $4400".to_string()])]),
        ((Instr::ROL, AdrMode::ZP), vec![Opcode::new(0x26, vec!["ROL $44".to_string()])]),
        ((Instr::ROL, AdrMode::IMPL), vec![Opcode::new(0x2A, vec!["ROL".to_string()])]),
        ((Instr::ROL, AdrMode::ABS), vec![Opcode::new(0x2E, vec!["ROL $4400".to_string()])]),
        ((Instr::ROL, AdrMode::ZPX), vec![Opcode::new(0x36, vec!["ROL $44,X".to_string()])]),
        ((Instr::ROL, AdrMode::ABSX), vec![Opcode::new(0x3E, vec!["ROL $4400,X".to_string()])]),
        ((Instr::PLP, AdrMode::IMPL), vec![Opcode::new(0x28, vec!["PLP".to_string()])]),
        ((Instr::BMI, AdrMode::REL), vec![Opcode::new(0x30, vec!["BMI $10".to_string(), "BMI label".to_string()])]),
        ((Instr::SEC, AdrMode::IMPL), vec![Opcode::new(0x38, vec!["SEC".to_string()])]),
        ((Instr::RTI, AdrMode::IMPL), vec![Opcode::new(0x40, vec!["RTI".to_string()])]),
        ((Instr::EOR, AdrMode::INDX), vec![Opcode::new(0x41, vec!["EOR ($44,X)".to_string()])]),
        ((Instr::EOR, AdrMode::ZP), vec![Opcode::new(0x45, vec!["EOR $44".to_string()])]),
        ((Instr::EOR, AdrMode::IMM), vec![Opcode::new(0x49, vec!["EOR #$44".to_string()])]),
        ((Instr::EOR, AdrMode::ABS), vec![Opcode::new(0x4D, vec!["EOR $4400".to_string()])]),
        ((Instr::EOR, AdrMode::INDY), vec![Opcode::new(0x51, vec!["EOR ($44),Y".to_string()])]),
        ((Instr::EOR, AdrMode::ZPX), vec![Opcode::new(0x55, vec!["EOR $44,X".to_string()])]),
        ((Instr::EOR, AdrMode::ABSY), vec![Opcode::new(0x59, vec!["EOR $4400,Y".to_string()])]),
        ((Instr::EOR, AdrMode::ABSX), vec![Opcode::new(0x5D, vec!["EOR $4400,X".to_string()])]),
        ((Instr::SRE, AdrMode::INDX), vec![Opcode::new(0x43, vec!["SRE ($44,X)".to_string()])]),
        ((Instr::SRE, AdrMode::ZP), vec![Opcode::new(0x47, vec!["SRE $44".to_string()])]),
        ((Instr::SRE, AdrMode::ABS), vec![Opcode::new(0x4F, vec!["SRE $4400".to_string()])]),
        ((Instr::SRE, AdrMode::INDY), vec![Opcode::new(0x53, vec!["SRE ($44),Y".to_string()])]),
        ((Instr::SRE, AdrMode::ZPX), vec![Opcode::new(0x57, vec!["SRE $44,X".to_string()])]),
        ((Instr::SRE, AdrMode::ABSY), vec![Opcode::new(0x5B, vec!["SRE $4400,Y".to_string()])]),
        ((Instr::SRE, AdrMode::ABSX), vec![Opcode::new(0x5F, vec!["SRE $4400,X".to_string()])]),
        ((Instr::LSR, AdrMode::ZP), vec![Opcode::new(0x46, vec!["LSR $44".to_string()])]),
        ((Instr::LSR, AdrMode::IMPL), vec![Opcode::new(0x4A, vec!["LSR".to_string()])]),
        ((Instr::LSR, AdrMode::ABS), vec![Opcode::new(0x4E, vec!["LSR $4400".to_string()])]),
        ((Instr::LSR, AdrMode::ZPX), vec![Opcode::new(0x56, vec!["LSR $44,X".to_string()])]),
        ((Instr::LSR, AdrMode::ABSX), vec![Opcode::new(0x5E, vec!["LSR $4400,X".to_string()])]),
        ((Instr::PHA, AdrMode::IMPL), vec![Opcode::new(0x48, vec!["PHA".to_string()])]),
        ((Instr::ALR, AdrMode::IMM), vec![Opcode::new(0x4B, vec!["ALR #$44".to_string()])]),
        ((Instr::JMP, AdrMode::ABS), vec![Opcode::new(0x4C, vec!["JMP $4400".to_string()])]),
        ((Instr::JMP, AdrMode::IND), vec![Opcode::new(0x6C, vec!["JMP ($5597)".to_string()])]),
        ((Instr::BVC, AdrMode::REL), vec![Opcode::new(0x50, vec!["BVC $10".to_string(), "BVC label".to_string()])]),
        ((Instr::CLI, AdrMode::IMPL), vec![Opcode::new(0x58, vec!["CLI".to_string()])]),
        ((Instr::RTS, AdrMode::IMPL), vec![Opcode::new(0x60, vec!["RTS".to_string()])]),
        ((Instr::ADC, AdrMode::INDX), vec![Opcode::new(0x61, vec!["ADC ($44,X)".to_string()])]),
        ((Instr::ADC, AdrMode::ZP), vec![Opcode::new(0x65, vec!["ADC $44".to_string()])]),
        ((Instr::ADC, AdrMode::IMM), vec![Opcode::new(0x69, vec!["ADC #$44".to_string()])]),
        ((Instr::ADC, AdrMode::ABS), vec![Opcode::new(0x6D, vec!["ADC $4400".to_string()])]),
        ((Instr::ADC, AdrMode::INDY), vec![Opcode::new(0x71, vec!["ADC ($44),Y".to_string()])]),
        ((Instr::ADC, AdrMode::ZPX), vec![Opcode::new(0x75, vec!["ADC $44,X".to_string()])]),
        ((Instr::ADC, AdrMode::ABSY), vec![Opcode::new(0x79, vec!["ADC $4400,Y".to_string()])]),
        ((Instr::ADC, AdrMode::ABSX), vec![Opcode::new(0x7D, vec!["ADC $4400,X".to_string()])]),
        ((Instr::RRA, AdrMode::INDX), vec![Opcode::new(0x63, vec!["RRA ($44,X)".to_string()])]),
        ((Instr::RRA, AdrMode::ZP), vec![Opcode::new(0x67, vec!["RRA $44".to_string()])]),
        ((Instr::RRA, AdrMode::ABS), vec![Opcode::new(0x6F, vec!["RRA $4400".to_string()])]),
        ((Instr::RRA, AdrMode::INDY), vec![Opcode::new(0x73, vec!["RRA ($44),Y".to_string()])]),
        ((Instr::RRA, AdrMode::ZPX), vec![Opcode::new(0x77, vec!["RRA $44,X".to_string()])]),
        ((Instr::RRA, AdrMode::ABSY), vec![Opcode::new(0x7B, vec!["RRA $4400,Y".to_string()])]),
        ((Instr::RRA, AdrMode::ABSX), vec![Opcode::new(0x7F, vec!["RRA $4400,X".to_string()])]),
        ((Instr::ROR, AdrMode::ZP), vec![Opcode::new(0x66, vec!["ROR $44".to_string()])]),
        ((Instr::ROR, AdrMode::IMPL), vec![Opcode::new(0x6A, vec!["ROR".to_string()])]),
        ((Instr::ROR, AdrMode::ABS), vec![Opcode::new(0x6E, vec!["ROR $4400".to_string()])]),
        ((Instr::ROR, AdrMode::ZPX), vec![Opcode::new(0x76, vec!["ROR $44,X".to_string()])]),
        ((Instr::ROR, AdrMode::ABSX), vec![Opcode::new(0x7E, vec!["ROR $4400,X".to_string()])]),
        ((Instr::PLA, AdrMode::IMPL), vec![Opcode::new(0x68, vec!["PLA".to_string()])]),
        ((Instr::ARR, AdrMode::IMM), vec![Opcode::new(0x6B, vec!["ARR #$44".to_string()])]),
        ((Instr::BVS, AdrMode::REL), vec![Opcode::new(0x70, vec!["BVS $10".to_string(), "BVS label".to_string()])]),
        ((Instr::SEI, AdrMode::IMPL), vec![Opcode::new(0x78, vec!["SEI".to_string()])]),
        ((Instr::STA, AdrMode::INDX), vec![Opcode::new(0x81, vec!["STA ($44,X)".to_string()])]),
        ((Instr::STA, AdrMode::ZP), vec![Opcode::new(0x85, vec!["STA $44".to_string()])]),
        ((Instr::STA, AdrMode::ABS), vec![Opcode::new(0x8D, vec!["STA $4400".to_string()])]),
        ((Instr::STA, AdrMode::INDY), vec![Opcode::new(0x91, vec!["STA ($44),Y".to_string()])]),
        ((Instr::STA, AdrMode::ZPX), vec![Opcode::new(0x95, vec!["STA $44,X".to_string()])]),
        ((Instr::STA, AdrMode::ABSY), vec![Opcode::new(0x99, vec!["STA $4400,Y".to_string()])]),
        ((Instr::STA, AdrMode::ABSX), vec![Opcode::new(0x9D, vec!["STA $4400,X".to_string()])]),
        ((Instr::SAX, AdrMode::INDX), vec![Opcode::new(0x83, vec!["SAX ($44,X)".to_string()])]),
        ((Instr::SAX, AdrMode::ZP), vec![Opcode::new(0x87, vec!["SAX $44".to_string()])]),
        ((Instr::SAX, AdrMode::ABS), vec![Opcode::new(0x8F, vec!["SAX $4400".to_string()])]),
        ((Instr::SAX, AdrMode::ZPY), vec![Opcode::new(0x97, vec!["SAX $44,Y".to_string()])]),
        ((Instr::STY, AdrMode::ZP), vec![Opcode::new(0x84, vec!["STY $44".to_string()])]),
        ((Instr::STY, AdrMode::ABS), vec![Opcode::new(0x8C, vec!["STY $4400".to_string()])]),
        ((Instr::STY, AdrMode::ZPX), vec![Opcode::new(0x94, vec!["STY $44,X".to_string()])]),
        ((Instr::STX, AdrMode::ZP), vec![Opcode::new(0x86, vec!["STX $44".to_string()])]),
        ((Instr::STX, AdrMode::ABS), vec![Opcode::new(0x8E, vec!["STX $4400".to_string()])]),
        ((Instr::STX, AdrMode::ZPY), vec![Opcode::new(0x96, vec!["STX $44,Y".to_string()])]),
        ((Instr::DEY, AdrMode::IMPL), vec![Opcode::new(0x88, vec!["DEY".to_string()])]),
        ((Instr::TXA, AdrMode::IMPL), vec![Opcode::new(0x8A, vec!["TXA".to_string()])]),
        ((Instr::XAA, AdrMode::IMM), vec![Opcode::new(0x8B, vec!["XAA #$44".to_string()])]),
        ((Instr::BCC, AdrMode::REL), vec![Opcode::new(0x90, vec!["BCC $10".to_string(), "BCC label".to_string()])]),
        ((Instr::AHX, AdrMode::INDY), vec![Opcode::new(0x93, vec!["AHX ($44),Y".to_string()])]),
        ((Instr::AHX, AdrMode::ABSY), vec![Opcode::new(0x9F, vec!["AHX $4400,Y".to_string()])]),
        ((Instr::TYA, AdrMode::IMPL), vec![Opcode::new(0x98, vec!["TYA".to_string()])]),
        ((Instr::TXS, AdrMode::IMPL), vec![Opcode::new(0x9A, vec!["TXS".to_string()])]),
        ((Instr::TAS, AdrMode::ABSY), vec![Opcode::new(0x9B, vec!["TAS $4400,Y".to_string()])]),
        ((Instr::SHY, AdrMode::ABSX), vec![Opcode::new(0x9C, vec!["SHY $4400,X".to_string()])]),
        ((Instr::SHX, AdrMode::ABSY), vec![Opcode::new(0x9E, vec!["SHX $4400,Y".to_string()])]),
        ((Instr::LDY, AdrMode::IMM), vec![Opcode::new(0xA0, vec!["LDY #$44".to_string()])]),
        ((Instr::LDY, AdrMode::ZP), vec![Opcode::new(0xA4, vec!["LDY $44".to_string()])]),
        ((Instr::LDY, AdrMode::ABS), vec![Opcode::new(0xAC, vec!["LDY $4400".to_string()])]),
        ((Instr::LDY, AdrMode::ZPX), vec![Opcode::new(0xB4, vec!["LDY $44,X".to_string()])]),
        ((Instr::LDY, AdrMode::ABSX), vec![Opcode::new(0xBC, vec!["LDY $4400,X".to_string()])]),
        ((Instr::LDA, AdrMode::INDX), vec![Opcode::new(0xA1, vec!["LDA ($44,X)".to_string()])]),
        ((Instr::LDA, AdrMode::ZP), vec![Opcode::new(0xA5, vec!["LDA $44".to_string()])]),
        ((Instr::LDA, AdrMode::IMM), vec![Opcode::new(0xA9, vec!["LDA #$44".to_string()])]),
        ((Instr::LDA, AdrMode::ABS), vec![Opcode::new(0xAD, vec!["LDA $4400".to_string()])]),
        ((Instr::LDA, AdrMode::INDY), vec![Opcode::new(0xB1, vec!["LDA ($44),Y".to_string()])]),
        ((Instr::LDA, AdrMode::ZPX), vec![Opcode::new(0xB5, vec!["LDA $44,X".to_string()])]),
        ((Instr::LDA, AdrMode::ABSY), vec![Opcode::new(0xB9, vec!["LDA $4400,Y".to_string()])]),
        ((Instr::LDA, AdrMode::ABSX), vec![Opcode::new(0xBD, vec!["LDA $4400,X".to_string()])]),
        ((Instr::LDX, AdrMode::IMM), vec![Opcode::new(0xA2, vec!["LDX #$44".to_string()])]),
        ((Instr::LDX, AdrMode::ZP), vec![Opcode::new(0xA6, vec!["LDX $44".to_string()])]),
        ((Instr::LDX, AdrMode::ABS), vec![Opcode::new(0xAE, vec!["LDX $4400".to_string()])]),
        ((Instr::LDX, AdrMode::ZPY), vec![Opcode::new(0xB6, vec!["LDX $44,Y".to_string()])]),
        ((Instr::LDX, AdrMode::ABSY), vec![Opcode::new(0xBE, vec!["LDX $4400,Y".to_string()])]),
        ((Instr::LAX, AdrMode::INDX), vec![Opcode::new(0xA3, vec!["LAX ($44,X)".to_string()])]),
        ((Instr::LAX, AdrMode::ZP), vec![Opcode::new(0xA7, vec!["LAX $44".to_string()])]),
        ((Instr::LAX, AdrMode::IMM), vec![Opcode::new(0xAB, vec!["LAX #$44".to_string()])]),
        ((Instr::LAX, AdrMode::ABS), vec![Opcode::new(0xAF, vec!["LAX $4400".to_string()])]),
        ((Instr::LAX, AdrMode::INDY), vec![Opcode::new(0xB3, vec!["LAX ($44),Y".to_string()])]),
        ((Instr::LAX, AdrMode::ZPY), vec![Opcode::new(0xB7, vec!["LAX $44,Y".to_string()])]),
        ((Instr::LAX, AdrMode::ABSY), vec![Opcode::new(0xBF, vec!["LAX $4400,Y".to_string()])]),
        ((Instr::TAY, AdrMode::IMPL), vec![Opcode::new(0xA8, vec!["TAY".to_string()])]),
        ((Instr::TAX, AdrMode::IMPL), vec![Opcode::new(0xAA, vec!["TAX".to_string()])]),
        ((Instr::BCS, AdrMode::REL), vec![Opcode::new(0xB0, vec!["BCS $10".to_string(), "BCS label".to_string()])]),
        ((Instr::CLV, AdrMode::IMPL), vec![Opcode::new(0xB8, vec!["CLV".to_string()])]),
        ((Instr::TSX, AdrMode::IMPL), vec![Opcode::new(0xBA, vec!["TSX".to_string()])]),
        ((Instr::LAS, AdrMode::ABSY), vec![Opcode::new(0xBB, vec!["LAS $4400,Y".to_string()])]),
        ((Instr::CPY, AdrMode::IMM), vec![Opcode::new(0xC0, vec!["CPY #$44".to_string()])]),
        ((Instr::CPY, AdrMode::ZP), vec![Opcode::new(0xC4, vec!["CPY $44".to_string()])]),
        ((Instr::CPY, AdrMode::ABS), vec![Opcode::new(0xCC, vec!["CPY $4400".to_string()])]),
        ((Instr::CMP, AdrMode::INDX), vec![Opcode::new(0xC1, vec!["CMP ($44,X)".to_string()])]),
        ((Instr::CMP, AdrMode::ZP), vec![Opcode::new(0xC5, vec!["CMP $44".to_string()])]),
        ((Instr::CMP, AdrMode::IMM), vec![Opcode::new(0xC9, vec!["CMP #$44".to_string()])]),
        ((Instr::CMP, AdrMode::ABS), vec![Opcode::new(0xCD, vec!["CMP $4400".to_string()])]),
        ((Instr::CMP, AdrMode::INDY), vec![Opcode::new(0xD1, vec!["CMP ($44),Y".to_string()])]),
        ((Instr::CMP, AdrMode::ZPX), vec![Opcode::new(0xD5, vec!["CMP $44,X".to_string()])]),
        ((Instr::CMP, AdrMode::ABSY), vec![Opcode::new(0xD9, vec!["CMP $4400,Y".to_string()])]),
        ((Instr::CMP, AdrMode::ABSX), vec![Opcode::new(0xDD, vec!["CMP $4400,X".to_string()])]),
        ((Instr::DCP, AdrMode::INDX), vec![Opcode::new(0xC3, vec!["DCP ($44,X)".to_string()])]),
        ((Instr::DCP, AdrMode::ZP), vec![Opcode::new(0xC7, vec!["DCP $44".to_string()])]),
        ((Instr::DCP, AdrMode::ABS), vec![Opcode::new(0xCF, vec!["DCP $4400".to_string()])]),
        ((Instr::DCP, AdrMode::INDY), vec![Opcode::new(0xD3, vec!["DCP ($44),Y".to_string()])]),
        ((Instr::DCP, AdrMode::ZPX), vec![Opcode::new(0xD7, vec!["DCP $44,X".to_string()])]),
        ((Instr::DCP, AdrMode::ABSY), vec![Opcode::new(0xDB, vec!["DCP $4400,Y".to_string()])]),
        ((Instr::DCP, AdrMode::ABSX), vec![Opcode::new(0xDF, vec!["DCP $4400,X".to_string()])]),
        ((Instr::DEC, AdrMode::ZP), vec![Opcode::new(0xC6, vec!["DEC $44".to_string()])]),
        ((Instr::DEC, AdrMode::ABS), vec![Opcode::new(0xCE, vec!["DEC $4400".to_string()])]),
        ((Instr::DEC, AdrMode::ZPX), vec![Opcode::new(0xD6, vec!["DEC $44,X".to_string()])]),
        ((Instr::DEC, AdrMode::ABSX), vec![Opcode::new(0xDE, vec!["DEC $4400,X".to_string()])]),
        ((Instr::INY, AdrMode::IMPL), vec![Opcode::new(0xC8, vec!["INY".to_string()])]),
        ((Instr::DEX, AdrMode::IMPL), vec![Opcode::new(0xCA, vec!["DEX".to_string()])]),
        ((Instr::AXS, AdrMode::IMM), vec![Opcode::new(0xCB, vec!["AXS #$44".to_string()])]),
        ((Instr::BNE, AdrMode::REL), vec![Opcode::new(0xD0, vec!["BNE $10".to_string(), "BNE label".to_string()])]),
        ((Instr::CLD, AdrMode::IMPL), vec![Opcode::new(0xD8, vec!["CLD".to_string()])]),
        ((Instr::CPX, AdrMode::IMM), vec![Opcode::new(0xE0, vec!["CPX #$44".to_string()])]),
        ((Instr::CPX, AdrMode::ZP), vec![Opcode::new(0xE4, vec!["CPX $44".to_string()])]),
        ((Instr::CPX, AdrMode::ABS), vec![Opcode::new(0xEC, vec!["CPX $4400".to_string()])]),
        ((Instr::SBC, AdrMode::INDX), vec![Opcode::new(0xE1, vec!["SBC ($44,X)".to_string()])]),
        ((Instr::SBC, AdrMode::ZP), vec![Opcode::new(0xE5, vec!["SBC $44".to_string()])]),
        ((Instr::SBC, AdrMode::IMM), vec![
            Opcode::new(0xE9, vec!["SBC #$44".to_string()]),
            Opcode::new(0xEB, vec!["SBC #$44".to_string()])]),
        ((Instr::SBC, AdrMode::ABS), vec![Opcode::new(0xED, vec!["SBC $4400".to_string()])]),
        ((Instr::SBC, AdrMode::INDY), vec![Opcode::new(0xF1, vec!["SBC ($44),Y".to_string()])]),
        ((Instr::SBC, AdrMode::ZPX), vec![Opcode::new(0xF5, vec!["SBC $44,X".to_string()])]),
        ((Instr::SBC, AdrMode::ABSY), vec![Opcode::new(0xF9, vec!["SBC $4400,Y".to_string()])]),
        ((Instr::SBC, AdrMode::ABSX), vec![Opcode::new(0xFD, vec!["SBC $4400,X".to_string()])]),
        ((Instr::ISC, AdrMode::INDX), vec![Opcode::new(0xE3, vec!["ISC ($44,X)".to_string()])]),
        ((Instr::ISC, AdrMode::ZP), vec![Opcode::new(0xE7, vec!["ISC $44".to_string()])]),
        ((Instr::ISC, AdrMode::ABS), vec![Opcode::new(0xEF, vec!["ISC $4400".to_string()])]),
        ((Instr::ISC, AdrMode::INDY), vec![Opcode::new(0xF3, vec!["ISC ($44),Y".to_string()])]),
        ((Instr::ISC, AdrMode::ZPX), vec![Opcode::new(0xF7, vec!["ISC $44,X".to_string()])]),
        ((Instr::ISC, AdrMode::ABSY), vec![Opcode::new(0xFB, vec!["ISC $4400,Y".to_string()])]),
        ((Instr::ISC, AdrMode::ABSX), vec![Opcode::new(0xFF, vec!["ISC $4400,X".to_string()])]),
        ((Instr::INC, AdrMode::ZP), vec![Opcode::new(0xE6, vec!["INC $44".to_string()])]),
        ((Instr::INC, AdrMode::ABS), vec![Opcode::new(0xEE, vec!["INC $4400".to_string()])]),
        ((Instr::INC, AdrMode::ZPX), vec![Opcode::new(0xF6, vec!["INC $44,X".to_string()])]),
        ((Instr::INC, AdrMode::ABSX), vec![Opcode::new(0xFE, vec!["INC $4400,X".to_string()])]),
        ((Instr::INX, AdrMode::IMPL), vec![Opcode::new(0xE8, vec!["INX".to_string()])]),
        ((Instr::BEQ, AdrMode::REL), vec![Opcode::new(0xF0, vec!["BEQ $10".to_string(), "BEQ label".to_string()])]),
        ((Instr::SED, AdrMode::IMPL), vec![Opcode::new(0xF8, vec!["SED".to_string()])]),
    ]);
}
