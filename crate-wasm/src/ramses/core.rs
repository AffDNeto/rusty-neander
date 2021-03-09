use crate::common::alu_trait::ExtendedALU;
use crate::common::memory_trait::MemoryAccess;
use crate::common::{
    alu_trait::SimpleAlu, 
    memory_trait::Memory, 
    register_trait::RegisterBank, 
    runner_trait::Runner
};

#[derive(Debug)]
pub struct RamsesMachine{
    pub pc: u8,
    pub ri: u8,
    pub rem: u8,
    pub rdm: u8,
    pub ra: u8,
    pub rb: u8,
    pub rx: u8,
    pub memory: [u8; 256],
    pub memory_access: usize,
    pub instruction_counter: usize,
    pub zero_flag: bool,
    pub negative_flag: bool,
    pub carry_flag: bool
}

impl Default for RamsesMachine {
    fn default() -> Self {
        RamsesMachine {
            pc: 0,
            ri: 0,
            rem: 0,
            rdm: 0,
            ra: 0,
            rb: 0,
            rx: 0,
            memory: [0; 256],
            memory_access: 0,
            instruction_counter: 0,
            zero_flag: false,
            negative_flag: false,
            carry_flag: false
        }
    }
}
impl SimpleAlu for RamsesMachine {
    fn set_zero(&mut self, value:bool) {self.zero_flag = value;}
    fn set_negative(&mut self, value:bool) {self.negative_flag = value;}
    fn set_carry(&mut self, value:bool) {self.carry_flag = value;}
    fn set_borrow(&mut self, value:bool) {self.carry_flag = !value;}

    fn get_zero(&self) -> bool {return self.zero_flag}
    fn get_negative(&self) -> bool {return self.negative_flag}
    fn get_carry(&self) -> bool {return self.carry_flag}
    fn get_borrow(&self) -> bool {return !self.carry_flag}

    //Since ramses doesn't have these flags nothing will be implemented
    fn set_overflow(&mut self, _:bool) {}
    fn get_overflow(&self) -> bool {false}
}

impl Memory for RamsesMachine{
    #[inline]
    fn _read(&self, rem: usize) -> u8 {
        return self.memory[rem]
    }
    #[inline]
    fn _write(&mut self, rem: usize, value: u8) {
        self.memory[rem] = value
    }
    #[inline]
    fn get_access_count(&self) -> usize {
        return self.memory_access
    }
    #[inline]
    fn set_access_count(&mut self, value: usize) {
        self.memory_access = value
    }
    #[inline]
    fn get_rem(&self) -> u8 {
        return self.rem;
    }
    #[inline]
    fn get_rdm(&self) -> u8 {
        return self.rdm;
    }
    #[inline]
    fn set_rem(&mut self, value: u8) {
        self.rem = value;
    }
    #[inline]
    fn set_rdm(&mut self, value: u8) {
        self.rdm = value;
    }
}

impl RegisterBank for RamsesMachine {
    #[inline]
    fn get_pc(&self) -> u8 {
        return self.pc;
    }
    #[inline]
    fn get_ri(&self) -> u8 {
        return self.ri;
    }
    #[inline]
    fn get_register(&self, id: u8) -> u8 {
        match id {
            0 => return self.ra,
            1 => return self.rb,
            2 => return self.rx,
            _ => return 0,
        }
    }

    #[inline]
    fn set_pc(&mut self, value: u8) {
        self.pc = value;
    }
    #[inline]
    fn set_ri(&mut self, value: u8) {
        self.ri = value;
    }
    #[inline]
    fn set_register(&mut self, id: u8, value: u8) {
        match id {
            0 => self.ra = value ,
            1 => self.rb = value ,
            2 => self.rx = value ,
            _ => return
        }
    }
}
impl MemoryAccess for RamsesMachine {
    fn index_registrer_id(&self) -> u8 { return 2; }
}
impl Runner for RamsesMachine {
    fn read_with_mode(&mut self) {
        println!("Reading with mode {} from {},{:02X} ", self.decode_mode(), self.get_rem(), self.get_rem());
        match self.decode_mode() {
            0 => self.direct_read(),
            1 => self.indirect_read(),
            2 => self.imediate_read(),
            3 => self.indexed_read(),
            _ => panic!("Invalid read mode")
        }
    }
    
    fn read_address_with_mode(&mut self){
        println!("Getting address to jump with...");
        match self.decode_mode() {
            3 => { 
                // Indexed mode for jmp operations uses the memory position
                // as a target and not the memory value like the other modes
                let address = self.get_indexed_address();
                self.set_rdm(address);
            },
            _ => self.read_with_mode()
        }
        println!("Address to is {},{:02X}", self.rdm, self.rdm)
    }

    fn write_with_mode(&mut self) {
        println!("Writing with mode {} to {},{:02X}: {} ", self.decode_mode(), self.rem, self.get_rem(), self.rdm);
        match self.decode_mode() {
            0 => self.direct_write(),
            1 => self.indirect_write(),
            2 => self.imediate_write(),
            3 => self.indexed_write(),
            _ => panic!("Invalid write mode")
        }
    }
    #[inline]
    fn decode_register(&self) -> u8 {
        (self.get_ri() & 0b0000_1100) >> 2
    }
    #[inline]
    fn decode_mode(&self) -> u8 {
        self.get_ri() & 0b0000_0011
    }
    #[inline]
    fn get_instruction_counter(&self) -> usize {
        return self.instruction_counter
    }

    #[inline]
    fn set_instruction_counter(&mut self, value: usize) {
        self.instruction_counter = value;
    }

    fn decode_and_execute(&mut self) -> bool {
        let operator = (self.decode_instruction()) >> 4;
        println!("Instruction op {},{:#x} from {:02X}", self.get_ri(), self.get_ri(), self.get_pc());
        match operator {
            0x1 => self.ramses_str(),//store
            0x2..=0x7 | 0xE
                => return self.ula_operation(),//ula operation
            0x8..=0xB => return self.jump_operation(),
            0xC => return self.jsr(),
            0x0 => return true,// NOP
            0xF => return false,// HLT
            _ => {
                println!("Unexpected instruction found {}", self.get_ri());
                return false
            }

        }
        // If the code reaches here some operation was done
        return true

    }
}

impl ExtendedALU for RamsesMachine {}

impl RamsesMachine {
    fn read(&mut self){
        let value = self._read(self.get_rem() as usize);
        self.set_rdm(value);
        self._increment_access_count();
    }
    fn write(&mut self){
        self._write(
            self.get_rem() as usize,
            self.get_rdm()
        );
        self._increment_access_count();
    }

    fn ramses_str(&mut self) {
        let store_value = self.get_register(self.decode_register());
        match self.decode_mode() {
            0 => self.str(),
            1 => {
                self.set_rem(self.get_pc());
                self.read();
                self._increment_pc();
                self.set_rem(self.get_rdm());
                self.read();
                self.set_rem(self.get_rdm());
                self.set_rdm(store_value);
                self.write();
            },
            2 => {
                self.set_rem(self.get_pc());
                self._increment_pc();
                self.set_rdm(store_value);
                self.write();
            },
            3 => {
                self.set_rem(self.get_pc());
                self._increment_pc();
                self.read();
                let index = self.get_register(self.index_registrer_id());
                self.set_rem(self.get_rdm().wrapping_add(index));
                self.set_rdm(store_value);
                self.write();
            },
            _ => return
        }
    }

    fn jsr(&mut self) -> bool {
        self.get_operator_from_memory();
        self.set_rem(self.get_rdm());
        self.set_pc(self.get_rdm());
        self._increment_pc();
        self.set_rdm(self.get_pc());
        self.write(); // Write return address in the subroutine header
        return true
    }

    fn jump_operation(&mut self) -> bool {
        // Don't jump if the instruction mode is imediate
        if self.decode_mode() == 2 {
            self._jmp_if(false);
            return true
        }
        let jump_op = self.decode_instruction();
        match jump_op {
            0x80 => self._jmp_if(true),
            0x90 => self._jmp_if(self.negative_flag),
            0xA0 => self._jmp_if(self.zero_flag),
            0xB0 => self._jmp_if(self.carry_flag),
            _ => return false
        }
        return true
    }
    fn ula_operation(&mut self) -> bool {
        let operation = self.decode_instruction();
        // Retrieve second operator from memory if necessary
        if let 0x20..=0x50 | 0x70 = self.get_ri() {
            self.get_operator_from_memory();    
        }
        let a = self.get_register(self.decode_register());
        let b = self.get_rdm();
        let result: u8; 
        match operation {
            0x20 => {
                result = b;
                self.compute_flags(b); //LDR
            },
            0x30 => result = self.add(a, b), //ADD
            0x40 => result = self.or(a, b), //OR
            0x50 => result = self.and(a, b), //AND
            0x60 => result = self.not(a), //NOT
            0x70 => result = self.sub(a, b), //SUB
            0xD0 => result = self.neg(a), //NEG
            0xE0 => result = self.shr(a), //SHR
            _ => return false
        }

        self.set_register(self.decode_register(), result);
        return true
    }
}

#[cfg(test)]
mod ramses_tests {
    use differ::{Differ, Tag};
    use std::convert::TryInto;
    use std::{fs::{self, File}, io::Read};
    use std::path::{Path, PathBuf};    
    use super::*;
    use rstest::*;

    /// Reads a bynary file to an u8 vector
    fn mem_to_array<T: AsRef<Path>>(filename: T) -> Vec<u8> {
        let mut f = File::open(&filename).expect("No file found");
        let metadata = fs::metadata(&filename).expect("Unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];

        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }

    fn array_to_256mem(array: Vec<u8>) -> Vec<u8> {
        array.into_iter()
            .skip(4) // First 4 bytes is the file header
            .enumerate()
            .filter(|&(i, _) | i%2 == 0) // .mem format has 0 on every odd position
            .map(|(_, v)| v) // removes the index added by the enumerate
            .collect()
    }

    fn compare_mem(a: &[u8], b: &[u8]){
        let diff =  Differ::new(&a, &b);
        for span in diff.spans() {
            match span.tag {
                Tag::Replace => {
                    println!("Difference found from {} to {}", span.b_start, span.b_end);
                    println!("Want:{:?}", &b[span.b_start..span.b_end]);
                    println!("Got :{:?}", &a[span.b_start..span.b_end]);
                },
                Tag::Delete => {
                    println!("Deleted found from {} to {}", span.b_start, span.b_end);
                    println!("Want:{:?}", &b[span.b_start..span.b_end]);
                    println!("Got :{:?}", &a[span.b_start..span.b_end]);
                },
                Tag::Insert => {
                    println!("Insert found from {} to {}", span.b_start, span.b_end);
                    println!("Want:{:?}", &b[span.b_start..span.b_end]);
                    println!("Got :{:?}", &a[span.b_start..span.b_end]);
                }
                _ => ()
            }
        }
    }
    fn ramses_path<T: AsRef<Path>>(filename: T) -> PathBuf {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        Path::new(crate_path)
            .join("tests")
            .join("ramses")
            .join(filename)
    }
    fn neander_path<T: AsRef<Path>>(filename: T) -> PathBuf {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        Path::new(crate_path)
            .join("tests")
            .join("neander")
            .join(filename)
    }

    fn read<T: AsRef<Path>+std::fmt::Debug>(test_path: T) -> [u8; 256] {
        println!("Reading file:");
        println!("{:?}", test_path);
        let mem = array_to_256mem( mem_to_array(&test_path));

        mem.try_into()
        .unwrap_or_else(
            |v: Vec<u8>| 
            panic!("Expecteted len 256 but came {}", v.len()
            )
        )
    }

    #[rstest(filename,
        case::ldr("ldr_test.mem"),
        case::str("str_test.mem"),
        case::not("not_test.mem"),
        case::shr("shr_test.mem"),
        case::or("or_test.mem"),
        case::neg("neg_test.mem"),
        case::add("add_test.mem"),
        case::sub("sub_test.mem"),
        case::and("and_test.mem"),
        case::jmp("jmp_test.mem"),
        case::jn("jn_test.mem"),
        case::jz("jz_test.mem"),
        case::jc("jc_test.mem"),
        case::jsr("jsr_test.mem")
    )]
    fn ramses(filename: impl AsRef<str>){
        let mut ramses = RamsesMachine{..Default::default()};
        let mut result_file = "result.".to_owned();
        result_file.push_str(filename.as_ref());
        let start = read(ramses_path(filename.as_ref()));
        let result = read(ramses_path(&result_file));
        ramses.memory = start;
        println!("[{}]", start.iter().fold(
            String::new(),
            |acc, &num| acc + &format!("{:02X}",&num)+", "
        ));
        for _ in 1..40 {
            println!("{:?}", ramses);
            if !ramses.step_code() { break; }
        }
        println!("{:?}", ramses);

        compare_mem(&ramses.memory, &result);
        //println!("What changed");
        //compare_mem(&ramses.memory, &start);
        
        assert!(ramses.memory==result);
    }

    #[rstest(filename,
        case::add("add_test.mem"),
        case::and("and_test.mem"),
        case::jmp("jmp_test.mem"),
        case::lda("lda_test.mem"),
        case::not("not_test.mem"),
        case::or("or_test.mem"),
        case::sta("sta_test.mem")
    )]
    fn neander(filename: impl AsRef<str>){
        let mut ramses = RamsesMachine{..Default::default()};
        let mut result_file = "result.".to_owned();
        result_file.push_str(filename.as_ref());
        
        let start = read(neander_path(filename.as_ref()));
        let result = read(neander_path(&result_file));
        ramses.memory  = start;

        for _ in 1..100 {
            println!("{:?}", ramses);
            if !ramses.step_code() { break; }
        }
        println!("{:?}", ramses);

        compare_mem(&ramses.memory , &result);
        println!("What changed");
        compare_mem(&ramses.memory , &start);
        
        assert!(ramses.memory ==result);
    }
}