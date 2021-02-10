use crate::common::{AhmesInstructions, BasicALU, ExecuteCycle, Memory, Memory256, NeanderOperations};

pub struct AhmesEmulator {
    pub mem: Memory256,
    pub registers: [u8; 2],
    pub negative_flag: bool,
    pub zero_flag: bool,
    pub carry_flag: bool,
    pub borrow_flag: bool,
    pub overflow_flag: bool,
    pub instruction_counter: usize
}

impl Default for AhmesEmulator{
    fn default() -> Self { 
        AhmesEmulator{
            mem: Memory256{..Default::default()},
            registers: [0; 2],
            negative_flag: false,
            zero_flag: false,
            carry_flag: false,
            borrow_flag: false,
            overflow_flag: false,
            instruction_counter: 0
        } 
    }
}

impl BasicALU for AhmesEmulator {
    fn read_register(&self, id: usize) -> u8 {
        self.registers[id]
    }

    fn write_register(&mut self, id: usize, value: u8) {
        self.registers[id] = value;
    }
}

impl ExecuteCycle<u8> for AhmesEmulator {
    fn next_instruction(&mut self) -> u8 {
        let value = self.mem.direct_read(self.read_pc());
        self.increment_pc();
        return value;
        
    }

    fn run_instruction(&mut self, op_code: u8) -> bool {
        todo!()
    }
}

impl AhmesEmulator {
    fn set_flags(&mut self, value: u8) {
        self.zero_flag = value == 0;
        self.negative_flag = value.leading_ones() > 0;
    }

    /// Retrieve destination address and jumps if condition is true
    fn jump_if(&mut self, condition: bool) -> bool {
        if condition { 
            let pos = self.next_instruction();
            self.set_pc(pos) 
        }else{
            self.increment_pc()
        }
        return true
    }
}

impl NeanderOperations for AhmesEmulator {
    fn store(&mut self) -> bool {
        let pos = self.next_instruction();
        self.mem.direct_write(
            pos, 
            self.read_register(1)
        );
        return true
    }

    fn load(&mut self) -> bool {
        let pos = self.next_instruction();
        let value = self.mem.direct_read(pos);
        self.write_register(1, value);
        self.set_flags(value);
        return true
    }

    fn add(&mut self) -> bool {
        let pos = self.next_instruction();
        let value = self.mem.direct_read(pos);
        let acc = self.read_register(1);
        let (result, carry) = acc.overflowing_add(value);
        let (_, overflow) = (acc as i8).overflowing_add(value as i8);
        
        self.write_register(1, result);
        self.set_flags(result);
        self.overflow_flag = overflow;
        self.carry_flag = carry;
        return true
    }

    fn or(&mut self) -> bool {
        let pos = self.next_instruction();
        let value = self.mem.direct_read(pos);
        let acc = self.read_register(1);
        let result = value | acc;

        self.write_register(1, result);
        self.set_flags(result);

        return true;
    }

    fn and(&mut self) -> bool {
        let pos = self.next_instruction();
        let value = self.mem.direct_read(pos);
        let acc = self.read_register(1);
        let result = value & acc;

        self.write_register(1, result);
        self.set_flags(result);

        return true;
    }

    fn not(&mut self) -> bool {
        let result = ! self.read_register(1);
        self.write_register(1, result);
        self.set_flags(result);

        return true;
    }

    fn jump(&mut self) -> bool {
        let pos = self.next_instruction();
        self.set_pc(pos);

        return true;
    }

    fn jump_negative(&mut self) -> bool {
        return self.jump_if(self.negative_flag)
    }

    fn jump_zero(&mut self) -> bool {
        return self.jump_if(self.zero_flag);
    }
}

impl AhmesInstructions for AhmesEmulator {
    fn jump_non_zero(&mut self) -> bool {
        return self.jump_if(!self.zero_flag)
    }

    fn jump_non_negative(&mut self) -> bool {
        return self.jump_if(!self.negative_flag)
    }

    fn jump_overflow(&mut self) -> bool {
        return self.jump_if(self.overflow_flag)
    }

    fn jump_non_overflow(&mut self) -> bool {
        return self.jump_if(!self.overflow_flag)
    }

    fn jump_carry(&mut self) -> bool {
        return self.jump_if(self.carry_flag)
    }

    fn jump_non_carry(&mut self) -> bool {
        return self.jump_if(!self.carry_flag)
    }

    fn jump_borrow(&mut self) -> bool {
        return self.jump_if(self.borrow_flag)
    }

    fn jump_non_borrow(&mut self) -> bool {
        return self.jump_if(!self.borrow_flag)
    }

    fn shift_right(&mut self) -> bool {
        let mut result = self.read_register(1);
        self.carry_flag = ( result & 0x01u8 ) != 0;
        result >>= 1;

        self.write_register(1, result);
        self.set_flags(result);

        return true
    }

    fn shift_left(&mut self) -> bool {
        let mut result = self.read_register(1);
        self.carry_flag = ( result & 0x80u8 ) != 0;
        result <<= 1;

        self.write_register(1, result);
        self.set_flags(result);

        return true
    }

    fn rotate_right(&mut self) -> bool {
        let mut result = self.read_register(1);
        let tmp_carry = self.carry_flag;
        self.carry_flag = ( result & 0x01u8 ) != 0;
        result >>= 1;

        if tmp_carry {
            result &= 0x80u8;
        }

        self.write_register(1, result);
        self.set_flags(result);

        return true
    }

    fn rotate_left(&mut self) -> bool {
        let mut result = self.read_register(1);
        let tmp_carry = self.carry_flag;
        self.carry_flag = ( result & 0x80u8 ) != 0;
        result <<= 1;

        if tmp_carry {
            result &= 0x01u8;
        }
        
        self.write_register(1, result);
        self.set_flags(result);

        return true
    }
}
#[cfg(test)]
mod ahmes_test{
    use super::*;
    use rstest::*;

    #[rstest(acc, op, c, v,
        case(0, 128, false, false),
        case(128, 128, true, true),
        case(127, 1, false, true),
        case(255, 1, true, false)
    )]
    fn add_flags(acc: u8, op: u8, c: bool, v: bool) {
        let mut cpu = AhmesEmulator{..Default::default()};
        cpu.write_register(1, acc);
        cpu.mem._write(0, 1);
        cpu.mem._write(1, op);
        cpu.add();
        assert_eq!(cpu.carry_flag, c);
        assert_eq!(cpu.overflow_flag, v);
    }

}