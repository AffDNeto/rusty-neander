use std::convert::TryInto;


pub trait ExecuteCycle<T> {
    fn execute_cycle(&mut self) -> bool {
        let op_code = self.read_pc();
        return self.run_instruction(op_code);
    }

    fn read_pc(&mut self) -> T;
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
