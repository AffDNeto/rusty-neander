pub mod core;
pub mod bindgen;
pub mod decoder;
mod operations;

#[derive(Debug)]
pub enum BranchType {
    Br,
    Bne,
    Beq,
    Bpl,
    Bmi,
    Bvc,
    Bvs,
    Bcc,
    Bcs,
    Bge,
    Blt,
    Bgt,
    Ble,
    Bhi,
    Bls,
    Nop
}

#[derive(Debug)]
pub enum OneOperandType {
    Clr,
    Not,
    Inc,
    Dec,
    Neg,
    Tst,
    Ror,
    Rol,
    Asr,
    Asl,
    Adc,
    Sbc,
    Nop
}

#[derive(Debug)]
pub enum TwoOperandType{
    Mov,
    Add,
    Sub,
    Cmp,
    And,
    Or,
    Nop
}

#[derive(Debug)]
pub enum AddressMode {
    Register,
    PosInc,
    PreDec,
    Index,
    Indirect,
    IndirectPosInc,
    IndirectPreDec,
    IndirectIndex
}

#[derive(Debug)]
pub struct ConditionFlags{
    pub n:bool,
    pub z:bool,
    pub v:bool,
    pub c:bool
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    SetCondition(ConditionFlags),
    ClearCondition(ConditionFlags),
    Jump{ rx:u8, mode:AddressMode},
    Branch{ displacement:u8, kind:BranchType},
    Loop{ rx:u8, displacement:u8},
    BranchSubroutine{ r1:u8, r2:u8, mode:AddressMode},
    ReturnSubroutine{ rx:u8 },
    OneOperand{ rx:u8, mode:AddressMode, kind:OneOperandType},
    TwoOperand{ r1:u8, r2:u8, mode1:AddressMode, mode2:AddressMode, kind:TwoOperandType},
    Halt
}

