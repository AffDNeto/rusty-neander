pub mod alu_trait;
pub mod memory_trait;
pub mod register_trait;
pub mod runner_trait;
use std::convert::TryInto;

pub trait BasicALU {
    fn read_register(&self, id: usize) -> u8;
    fn write_register(&mut self, id: usize, value: u8);

    //by convention the program counter will always be the first
    fn read_pc(&self) -> u8 {
        self.read_register(0)
    }

    fn set_pc(&mut self, value: u8) {
        self.write_register(0, value)
    }
    fn increment_pc(&mut self) {
        self.write_register(0,self.read_pc().wrapping_add(1))
    }
}

pub trait NeanderOperations: BasicALU {
    /// Stops the cpu from running
    fn halt(&self) -> bool {
        return false
    }
    
    /// Do no operation at all
    fn no_operation(&self) -> bool {
        return true
    }

    fn store(&mut self) -> bool;
    fn load(&mut self) -> bool;
    fn add(&mut self) -> bool;
    fn or(&mut self) -> bool;
    fn and(&mut self) -> bool;
    fn not(&mut self) -> bool;
    fn jump(&mut self) -> bool;
    fn jump_negative(&mut self) -> bool;
    fn jump_zero(&mut self) -> bool;
}

pub trait AhmesInstructions {
    fn sub(&mut self) -> bool;
    fn jump_non_zero(&mut self) -> bool;
    fn jump_non_negative(&mut self) -> bool;
    fn jump_overflow(&mut self) -> bool;
    fn jump_non_overflow(&mut self) -> bool;
    fn jump_carry(&mut self) -> bool;
    fn jump_non_carry(&mut self) -> bool;
    fn jump_borrow(&mut self) -> bool;
    fn jump_non_borrow(&mut self) -> bool;
    fn shift_right(&mut self) -> bool;
    fn shift_left(&mut self) -> bool;
    fn rotate_right(&mut self) -> bool;
    fn rotate_left(&mut self) -> bool;
}

pub trait ExecuteCycle<T>{
    fn execute_cycle(&mut self) -> bool {
        let op_code = self.next_instruction();
        return self.run_instruction(op_code);
    }

    fn next_instruction(&mut self) -> T;
    fn run_instruction(&mut self, op_code: T) -> bool;
}

pub trait Memory<T> {
    fn _read(&self, address: T) -> T;
    fn _write(&mut self, address: T, value: T);
    fn _access_inc(&mut self);
    fn load(&mut self, new_data: Vec<T>);
    fn dump(&self) -> Vec<T>;

    fn direct_read(&mut self, address: T) -> T {
        self._access_inc();
        return self._read(address);
    }
    fn direct_write(&mut self, address: T, value: T){
        self._access_inc();
        self._write(address, value);
    }
}

#[derive(Debug)]
pub struct Memory256 {
    pub mem: [u8; 256],
    pub access_counter: usize
}

impl Default for Memory256 {
    fn default() -> Self { Memory256 { mem: [0;256], access_counter: 0} }
}

impl Memory<u8> for Memory256 {
    fn _read(&self, address: u8) -> u8 {
        return self.mem[address as usize];
    }

    fn _write(&mut self, address: u8, value: u8) {
        self.mem[address as usize] = value;
    }

    fn _access_inc(&mut self) {
        self.access_counter += 1;
    }

    fn load(&mut self, new_data: Vec<u8>) {
        self.mem = new_data.try_into()
        .unwrap_or_else(
            |v: Vec<u8>| 
            panic!("Expecteted len {} but came {}", self.mem.len(), v.len()
            )
        );
    }

    fn dump(&self) -> Vec<u8> {
        return self.mem.to_vec();
    }
}
