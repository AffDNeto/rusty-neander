use crate::common::{ExecuteCycle, Memory, Memory256};

pub struct NeanderCPU {
    pub mem: Memory256,
    pub accumulator: u8,
    pub program_counter: u8,
    pub ri: u8,
    pub negative_flag: bool,
    pub zero_flag: bool,
    pub instruction_counter: usize
}

impl Default for NeanderCPU {
    fn default() -> NeanderCPU {
        NeanderCPU{
            mem : Memory256 {..Default::default()},
            accumulator: 0,
            program_counter: 0,
            ri: 0,
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
    
    fn read_pc(&mut self) -> u8 {
        let value = self.mem.direct_read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        
        return value;
    }
}

impl NeanderCPU{
    /// Loads accumulator with a value and set the proper flags.
    pub fn load_accumulator(&mut self, value: u8) {
        self.accumulator = value;
        self.zero_flag = value == 0;
        self.negative_flag = value >= i8::MAX as u8;
    }

    ///
    /// Instructions
    ///
    
    /// Do no operation at all
    pub fn no_operation(&mut self) -> bool {
        return true
    }

    /// Stops the cpu from running
    pub fn halt(&mut self) -> bool {
        self.program_counter = self.program_counter.wrapping_sub(1);
        return false
    }

    /// Writes accumulator value to memory
    pub fn store(&mut self) -> bool {
        let position = self.read_pc();
        self.mem.direct_write(position, self.accumulator);
        return true;
    }

    /// Read memory position to accumulator
    pub fn load(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.mem.direct_read(position);
        self.load_accumulator(value);
        return true;
    }
    
    /// Add memory value with accumulator value and stores to it
    pub fn add(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.mem.direct_read(position);

        self.load_accumulator(self.accumulator.wrapping_add(value));

        return true;
    }

    /// Bitwise OR operation
    pub fn or(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.mem.direct_read(position);

        self.load_accumulator(self.accumulator | value);
        
        return true;
    }

    /// Bitwise AND operation
    pub fn and(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.mem.direct_read(position);

        self.load_accumulator(self.accumulator & value);
        
        return true;
    }

    /// Bitwise NOT operation on the accumulator
    pub fn not(&mut self) -> bool {
        self.load_accumulator(!self.accumulator);
        
        return true;
    }

    /// Jumps (program counter) to memory position unconditionaly
    pub fn jump(&mut self) -> bool {
        let position = self.read_pc();
        self.program_counter = position;
        return true;
    }

    /// Jumps only if negative flag is on
    pub fn jump_negative(&mut self) -> bool {
        let position  = self.read_pc();

        if self.negative_flag {
            self.program_counter = position
        }
        return true;
    }

    /// Jumps only if zero flag is ON
    pub fn jump_zero(&mut self) -> bool {
        let position  = self.read_pc();

        if self.zero_flag {
            self.program_counter = position
        }
        return true;
    
    }
    
}

#[cfg(test)]
mod neander_tests{

    use super::*;

    #[test]
    fn create_cpu() {
        let cpu = NeanderCPU{..Default::default()};
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.accumulator, 0);
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
