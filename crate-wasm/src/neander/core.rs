use crate::common::BasicOperations;
use crate::common::BasicALU;
use crate::common::{ExecuteCycle, Memory, Memory256};

pub struct NeanderCPU {
    pub mem: Memory256,
    pub registers: [u8; 2],
    pub negative_flag: bool,
    pub zero_flag: bool,
    pub instruction_counter: usize
}

impl Default for NeanderCPU {
    fn default() -> NeanderCPU {
        NeanderCPU{
            mem : Memory256 {..Default::default()},
            registers: [0; 2],
            negative_flag: false,
            zero_flag: true,
            instruction_counter: 0
        }
    }
}
impl ExecuteCycle<u8> for NeanderCPU {
    
    fn run_instruction(&mut self, op_code: u8) -> bool {
        let op = ( op_code & 0xF0 ) >> 4;
        self.instruction_counter += 1;
        match op {
            0x0 => return self.no_operation(),
            0x1 => return self.store(),
            0x2 => return self.load(),
            0x3 => return self.add(),
            0x4 => return self.or(),
            0x5 => return self.and(),
            0x6 => return self.not(),
            0x8 => return self.jump(),
            0x9 => return self.jump_negative(),
            0xA => return self.jump_zero(),
            0xF => return self.halt(),
            _ => return self.halt()
        }
    }
    
    fn next_instruction(&mut self) -> u8 {
        let value = self.mem.direct_read(self.read_pc());
        self.increment_pc();
        return value;
    }
}

impl BasicALU for NeanderCPU{
    fn read_register(&self, id: usize) -> u8 {
        self.registers[id]
    }

    fn write_register(&mut self, id: usize, value: u8) {
        self.registers[id] = value;
    }
}

impl BasicOperations for NeanderCPU {
    /// Writes accumulator value to memory
    fn store(&mut self) -> bool {
        let position = self.next_instruction();
        self.mem.direct_write(position, self.read_register(1));
        return true;
    }

    /// Read memory position to accumulator
    fn load(&mut self) -> bool {
        let position = self.next_instruction();
        let value = self.mem.direct_read(position);
        self.set_accumulator(value);
        return true;
    }
    
    /// Add memory value with accumulator value and stores to it
    fn add(&mut self) -> bool {
        let position = self.next_instruction();
        let value = self.mem.direct_read(position);

        self.set_accumulator(self.read_register(1).wrapping_add(value));

        return true;
    }

    /// Bitwise OR operation
    fn or(&mut self) -> bool {
        let position = self.next_instruction();
        let value = self.mem.direct_read(position);

        self.set_accumulator(self.read_register(1) | value);
        
        return true;
    }

    /// Bitwise AND operation
    fn and(&mut self) -> bool {
        let position = self.next_instruction();
        let value = self.mem.direct_read(position);

        self.set_accumulator(self.read_register(1) & value);
        
        return true;
    }

    /// Bitwise NOT operation on the accumulator
    fn not(&mut self) -> bool {
        self.set_accumulator(!self.read_register(1));
        
        return true;
    }

    /// Jumps (program counter) to memory position unconditionaly
    fn jump(&mut self) -> bool {
        let position = self.next_instruction();
        self.set_pc(position);
        return true;
    }

    /// Jumps only if negative flag is on
    fn jump_negative(&mut self) -> bool {
        let position  = self.next_instruction();

        if self.negative_flag {
            self.set_pc(position);
        }
        return true;
    }

    /// Jumps only if zero flag is ON
    fn jump_zero(&mut self) -> bool {
        let position  = self.next_instruction();

        if self.zero_flag {
            self.set_pc(position);
        }
        return true;
    
    }
}
impl NeanderCPU{
    /// Loads accumulator with a value and set the proper flags.
    pub fn set_accumulator(&mut self, value: u8) {
        self.write_register(1, value);
        self.zero_flag = value == 0;
        self.negative_flag = value.leading_ones() > 0;
    } 
}

#[cfg(test)]
mod neander_tests{

    use super::*;

    #[test]
    fn create_cpu() {
        let cpu = NeanderCPU{..Default::default()};
        assert_eq!(cpu.read_pc(), 0);
        assert_eq!(cpu.read_register(1), 0);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.negative_flag, false);
        assert_eq!(cpu.mem.dump(), [0;256].to_vec());   
    }

    #[test]
    fn cycle_test(){
        let mut cpu = NeanderCPU{..Default::default()};
        let cycle_result = cpu.execute_cycle();
        
        assert_eq!(cycle_result, true);
        assert_eq!(cpu.instruction_counter, 1)
    }

    #[test]
    fn write_test(){
        let mut cpu = NeanderCPU{..Default::default()};
        cpu.mem.direct_write(0, 4);

        assert_eq!(cpu.mem.mem[0], 4);
        assert_eq!(cpu.mem.access_counter, 1);
    }

    #[test]
    fn read_test(){
        let mut cpu = NeanderCPU{..Default::default()};
        cpu.mem.mem[0] = 4;

        assert_eq!(cpu.mem.direct_read(4), 4);
        assert_eq!(cpu.mem.access_counter, 1);
    }
}
