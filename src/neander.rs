pub struct NeanderCPU {
    pub mem: [u8; 256],
    pub accumulator: u8,
    pub program_counter: u8,
    pub ri: u8,
    pub negative_flag: bool,
    pub zero_flag: bool,
    pub mem_access_counter: usize,
    pub instruction_counter: usize
}

impl Default for NeanderCPU {
    fn default() -> NeanderCPU {
        NeanderCPU{
            mem : [ 0; 256],
            accumulator: 0,
            program_counter: 0,
            ri: 0,
            negative_flag: false,
            zero_flag: false,
            mem_access_counter: 0,
            instruction_counter: 0
        }
    }
}
impl NeanderCPU {
    pub fn execute_cycle(&mut self) -> bool{
        let op_code = self.read_pc();
        let instruction_result = self.do_instruction(op_code);
        
        self.instruction_counter += 1;
        return instruction_result;
    }

    pub fn do_instruction(&mut self, op_code: u8) -> bool {
        ///let op = ( op_code & 0xF0 ) >> 4;

        match op_code {
            0x00 => return self.no_operation(),
            0x10 => return self.store(),
            0x20 => return self.load(),
            0x30 => return self.add(),
            0x40 => return self.or(),
            0x50 => return self.and(),
            0x60 => return self.not(),
            0x80 => return self.jump(),
            0x90 => return self.jump_negative(),
            0xA0 => return self.jump_zero(),
            0xF0 => return self.halt(),
            _ => return self.halt()
        }
    }
    
    fn read_pc(&mut self) -> u8 {
        let value = self.read_byte(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        
        return value;
    }

    fn read_byte(&mut self, address: u8) -> u8 {
        self.mem_access_counter += 1;
        return self.mem[address as usize];
    }
    
    fn write_byte(&mut self, address: u8, data: u8) {
        self.mem_access_counter += 1;
        self.mem[address as usize] = data;
    }

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
        self.write_byte(position, self.accumulator);
        return true;
    }

    /// Read memory position to accumulator
    pub fn load(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.read_byte(position);
        self.load_accumulator(value);
        return true;
    }
    
    /// Add memory value with accumulator value and stores to it
    pub fn add(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.read_byte(position);

        self.load_accumulator(self.accumulator.wrapping_add(value));

        return true;
    }

    /// Bitwise OR operation
    pub fn or(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.read_byte(position);

        self.load_accumulator(self.accumulator | value);
        
        return true;
    }

    /// Bitwise AND operation
    pub fn and(&mut self) -> bool {
        let position = self.read_pc();
        let value = self.read_byte(position);

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
        assert_eq!(cpu.mem, [0;256]);   
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
        cpu.write_byte(0, 4);

        assert_eq!(cpu.mem[0], 4);
        assert_eq!(cpu.mem_access_counter, 1);
    }

    #[test]
    fn read_test(){
        let mut cpu = NeanderCPU{..Default::default()};
        cpu.mem[0] = 4;

        assert_eq!(cpu.read_byte(4), 4);
        assert_eq!(cpu.mem_access_counter, 1);
    }
}
