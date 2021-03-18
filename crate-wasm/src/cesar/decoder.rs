

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

#[derive(Debug)]
pub struct CesarDecoder {
    pub ri: [u8; 2]
}

impl CesarDecoder {
    /// Tells if the instruction ocupies one byte based on the first one
    pub fn is_single_byte_instruction(&self) -> bool {
        let instruction_code  = (self.ri[0] >> 4) & 0x0F;

        if let 0x0..=0x3 | 0x7 | 0xf = instruction_code {
            return true
        }else{
            return false
        }
    }

    /// Decodes the instruction in RI
    pub fn instruction(&self) -> Instruction {
        let instruction_code  = (self.ri[0] >> 4) & 0x0F;

        match instruction_code {
            0x0 => Instruction::Nop,
            0x1 => Instruction::ClearCondition(self.condition_flags()),
            0x2 => Instruction::SetCondition(self.condition_flags()),
            0x3 => Instruction::Branch{
                displacement:self.displacement(),
                kind: self.branch_type()
            },
            0x4 => Instruction::Jump{
                mode:self.second_mode(),
                rx: self.second_register()
            },
            0x5 => Instruction::Loop{
                displacement:self.displacement(),
                rx: self.loop_register()
            },
            0x6 => Instruction::BranchSubroutine{
                r1: self.loop_register(),
                r2: self.second_register(),
                mode: self.second_mode()
            },
            0x7 => Instruction::ReturnSubroutine{
                rx: self.loop_register()
            },
            0x8 => Instruction::OneOperand{
                rx: self.second_register(),
                mode: self.second_mode(),
                kind: self.single_op_instruction_type()
            },
            0x9 ..= 0xE => Instruction::TwoOperand{
                r1: self.first_register(),
                mode1: self.first_mode(),
                r2: self.second_register(),
                mode2: self.second_mode(),
                kind: self.dual_op_instruction_type()
            },
            0xF => Instruction::Halt,
            _ => Instruction::Nop
        }
    }

    fn dual_op_instruction_type(&self) -> TwoOperandType {
        let operation = (self.ri[0] & 0b0111_0000 ) >> 4;
        match operation {
            1 => TwoOperandType::Mov,
            2 => TwoOperandType::Add,
            3 => TwoOperandType::Sub,
            4 => TwoOperandType::Cmp,
            5 => TwoOperandType::Add,
            6 => TwoOperandType::Or,
            _ => TwoOperandType::Nop
        }
    }
    
    fn single_op_instruction_type(&self) -> OneOperandType {
        let operation = self.ri[0] & 0x0F;
        match operation {
            0 => OneOperandType::Clr,
            1 => OneOperandType::Not,
            2 => OneOperandType::Inc,
            3 => OneOperandType::Dec,
            4 => OneOperandType::Neg,
            5 => OneOperandType::Tst,
            6 => OneOperandType::Ror,
            7 => OneOperandType::Rol,
            8 => OneOperandType::Asr,
            9 => OneOperandType::Asl,
            10 => OneOperandType::Adc,
            11 => OneOperandType::Sbc,
            _ => OneOperandType::Nop
        }
    }

    /// Returns the Enum representing the address mode based on it's identifier
    fn get_address_enum(mode: u8) -> AddressMode {
        match mode{
            0 => AddressMode::Register,
            1 => AddressMode::PosInc,
            2 => AddressMode::PreDec,
            3 => AddressMode::Index,
            4 => AddressMode::Indirect,
            5 => AddressMode::IndirectPosInc,
            6 => AddressMode::IndirectPreDec,
            7 => AddressMode::IndirectIndex,
            _ => panic!("Invalid adress mode {}", mode)
        }
    }

    /// Return the address mode located on bits 00001110 
    /// of the instruction's first byte
    fn first_mode(&self) -> AddressMode {
        return Self::get_address_enum((self.ri[0] & 0b0000_1110) >> 1)
    }
    
    /// Return the register used located on bits 0000_0001 
    /// of the instruction's first byte and 1100_0000 of the
    /// second byte
    fn first_register(&self) -> u8 {
        return (self.ri[1] & 0b1100_0000) >> 6 
        | (self.ri[0] & 0b0000_0001) << 2
    }

    /// Return the address mode located on bits 00111000 
    /// of the instruction's second byte
    fn second_mode(&self) -> AddressMode {
        return Self::get_address_enum((self.ri[1] & 0b0011_1000) >> 3)
    }

    /// Return the register used located on bits 00000111 
    /// of the instruction's second byte
    fn second_register(&self) -> u8 {
        return self.ri[1] & 0b0000_0111
    }
    
    /// Return the register used located on bits 00000111 
    /// of the instruction's FIRST byte
    fn loop_register(&self) -> u8 {
        return self.ri[0] & 0b0000_0111
    }

    fn condition_flags(&self) -> ConditionFlags {
        let flags = self.ri[0] & 0x0F;

        return ConditionFlags{
            n:flags & 0b1000 != 0,
            z:flags & 0b0100 != 0,
            v:flags & 0b0010 != 0,
            c:flags & 0b0001 != 0
        }
    }

    /// Decodes the displacement for a branch or loop instructions
    fn displacement(&self) -> u8 {
        return self.ri[1]
    }

    fn branch_type(&self) -> BranchType {
        let branch_code = (self.ri[0] & 0x0F) >> 4 ;

        match branch_code {
            0 => return BranchType::Br,
            1 => return BranchType::Bne,
            2 => return BranchType::Beq,
            3 => return BranchType::Bpl,
            4 => return BranchType::Bmi,
            5 => return BranchType::Bvc,
            6 => return BranchType::Bvs,
            7 => return BranchType::Bcc,
            8 => return BranchType::Bcs,
            9 => return BranchType::Bge,
            10 => return BranchType::Blt,
            11 => return BranchType::Bgt,
            12 => return BranchType::Ble,
            13 => return BranchType::Bhi,
            14 => return BranchType::Bls,
            _ => return BranchType::Nop,
        }
    }
}


#[cfg(test)]
mod DecoderTest{
    use rstest::*;
    use super::*;

    #[rstest(instruction,
        case([0xc1, 0]),
        case([0x93, 0xc1]),
        case([0x14, 0x15]),
    )]
    fn test_decoder(instruction: [u8; 2]){
        let decoder = CesarDecoder{ri:instruction};
        println!("{:#?}", decoder.instruction());
        assert_eq!(1, 2);
    }
}