use super::{alu_trait::SimpleAlu, memory_trait::Memory};

pub trait RegisterBank {
    fn get_pc(&self) -> u8; // Program Counter
    fn get_ri(&self) -> u8; // Instruction Register
    fn get_register(&self, id: u8) -> u8;

    fn set_pc(&mut self, value: u8);
    fn set_ri(&mut self, value: u8);
    fn set_register(&mut self, id: u8, value: u8);

    fn _increment_pc(&mut self) -> u8 {
        let n_pc = self.get_pc().wrapping_add(1);
        self.set_pc(n_pc);
        return n_pc;
    }
}

pub trait Runner: SimpleAlu + RegisterBank + Memory {
    fn get_instruction_counter(&self) -> usize;
    fn set_instruction_counter(&mut self, value: usize);
    //decodes isntruction on RI and executes it
    fn decode_and_execute(&mut self) -> bool;
    
    fn reset_instruction_counter(&mut self){
        self.set_instruction_counter(0);
    }
    fn increment_instruction_counter(&mut self) {
        let inc = self.get_instruction_counter()+1;
        self.set_instruction_counter(inc);
    }
    
    /// Runs the next instruction and returns a value if the machines, 
    /// stoped running in case it run a halt instruction
    fn step_code(&mut self) -> bool {
        self.search_instruction();
        self.increment_instruction_counter();
        return self.decode_and_execute()
    }

    /// Searches for the next instruction on the memory and 
    /// sets RI
    fn search_instruction(&mut self) {
        self.read_from_pc();
        self.set_ri(self.get_rdm());
    }

    /// Reads from memory using the pc as an address and increments it
    fn read_from_pc(&mut self) {
        self.set_rem(self.get_pc());
        self._increment_pc();
        self.direct_read();
    }

    /// Reads from memory based on the addressing mode set in RI
    fn read_with_mode(&mut self) {
        // the simplest implementation is direct read
        self.direct_read();
    }

    /// Reads the address used by intruction with mode, used by jump functions
    /// By default is the same as the read_with_mode method
     fn read_address_with_mode(&mut self){
         self.read_with_mode();
    }

    /// Writes to memory what is on th addressing mode set in RI
    fn write_with_mode(&mut self) {
        self.direct_write();
    }

    //Jumps only if condition is true
    fn _jmp_if(&mut self, condition: bool) {
        // do the same behaviour of the machine, 
        //searching the destiny address even if no jump occurs
        self.set_rem(self.get_pc());
        self.read_address_with_mode();
        if condition {
            self.set_pc(self.get_rdm());
        }else{
            self._increment_pc();
        }
    }

    /// returns the index of the register signalized on RI 
    #[inline]
    fn decode_register(&self) -> u8 { 
        // By default will return the first register
        return 0
    }
    
    /// Returns the memory mode of the instruction, by default it is always 0
    #[inline]
    fn decode_mode(&self) -> u8 {
        return 0
    }

    //// Returns the current instruction identifier, by default it is the first 4 bits
    #[inline]
    fn decode_instruction(&self) -> u8 {
        self.get_ri() & 0b1111_0000
    }
    /// Stores from register to memmory
    fn str(&mut self) {
        self.read_from_pc();
        let pos = self.get_rdm();
        let value = self.get_register(self.decode_register());
        self.set_rem(pos);
        self.set_rdm(value);
        self.write_with_mode();
    }

    // Prepares the registers for an ALU operation based on the RI
    fn get_operator_from_memory(&mut self) {
        self.read_from_pc();
        self.set_rem(self.get_rdm());
        self.read_with_mode();
    }
    
}