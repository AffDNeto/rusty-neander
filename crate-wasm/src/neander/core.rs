use crate::common::{alu_trait::SimpleAlu, memory_trait::Memory, register_trait::{RegisterBank, Runner}};

#[derive(Debug)]
pub struct NeanderMachine{
    pub pc: u8,
    pub ri: u8,
    pub rem: u8,
    pub rdm: u8,
    pub acc: u8,
    pub memory: [u8; 256],
    pub memory_access: usize,
    pub instruction_counter: usize,
    pub zero_flag: bool,
    pub negative_flag: bool
}

impl Default for NeanderMachine {
    fn default() -> Self {
        NeanderMachine {
            pc: 0,
            ri: 0,
            rem: 0,
            rdm: 0,
            acc: 0,
            memory: [0; 256],
            memory_access: 0,
            instruction_counter: 0,
            zero_flag: false,
            negative_flag: false
        }
    }
}
impl SimpleAlu for NeanderMachine {
    fn set_zero(&mut self, value:bool) {
        self.zero_flag = value;
    }

    fn set_negative(&mut self, value:bool) {
        self.negative_flag = value;
    }

    fn get_zero(&self) -> bool {
        return self.zero_flag
    }

    fn get_negative(&self) -> bool {
        return self.negative_flag
    }

    //Since neander doesn't have these flags nothing will be implemented
    fn set_carry(&mut self, _:bool) {}
    fn set_overflow(&mut self, _:bool) {}
    fn set_borrow(&mut self, _:bool) {}
    fn get_carry(&self) -> bool {false}
    fn get_overflow(&self) -> bool {false}
    fn get_borrow(&self) -> bool {false}
}

impl Memory for NeanderMachine{
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

impl RegisterBank for NeanderMachine {
    #[inline]
    fn get_pc(&self) -> u8 {
        return self.pc;
    }
    #[inline]
    fn get_ri(&self) -> u8 {
        return self.ri;
    }
    #[inline]
    fn get_register(&self, _: u8) -> u8 {
        return self.acc;
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
    fn set_register(&mut self, _: u8, value: u8) {
        self.acc = value;
    }
}

impl Runner for NeanderMachine {
    #[inline]
    fn get_instruction_counter(&self) -> usize {
        return self.instruction_counter
    }

    #[inline]
    fn set_instruction_counter(&mut self, value: usize) {
        self.instruction_counter = value;
    }

    fn decode_and_execute(&mut self) -> bool {
        let operator = (self.get_ri() & 0b1111_0000) >> 4;

        match operator {
            0x1 => self.str(),//store
            0x2..=0x5 => self.ula_operation(),//ula operation
            0x6 => self.op_not(),// NOT
            0x8 => self._jmp_if(true),
            0x9 => self._jmp_if(self.get_negative()), //JN
            0xA => self._jmp_if(self.get_zero()), // JZ
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

impl NeanderMachine {
    fn ula_operation(&mut self) {
        self.pre_ula_operation();
        let operation = (self.get_ri() & 0b1111_0000) >> 4;
        let a = self.get_register(self.ri_reg());
        let b = self.get_rdm();
        let result: u8; 
        match operation {
            0x2 => {
                result = b;
                self.compute_flags(b); //LDA
            },
            0x3 => result = self.add(a, b), //ADD
            0x4 => result = self.or(a, b), //OR
            0x5 => result = self.and(a, b), //AND
            _ => result = 0
        }

        self.set_register(self.ri_reg(), result);
    }

    fn op_not(&mut self) {
        let a = self.get_register(self.ri_reg());
        let result = self.not(a);
        self.set_register(self.ri_reg(), result);
    }
}

#[cfg(test)]
mod functional_tests {
    use differ::{Differ, Tag};
    use std::convert::TryInto;
    use std::{fs::{self, File}, io::Read};
    use std::path::Path;    
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
                _ => ()
            }
        }
    }

    fn read<T: AsRef<Path>>(file_path: T) -> [u8; 256] {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let test_path = Path::new(crate_path)
            .join("tests")
            .join("neander")
            .join(file_path);
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
        case::simples("simples.mem"),
        case::add("add_test.mem"),
        case::and("and_test.mem"),
        case::jmp("jmp_test.mem"),
        case::lda("lda_test.mem"),
        case::not("not_test.mem"),
        case::or("or_test.mem"),
        case::sta("sta_test.mem")
    )]
    fn basic_neander_test(filename: impl AsRef<str>){
        let mut neander = NeanderMachine{..Default::default()};
        let mut result_file = "result.".to_owned();
        result_file.push_str(filename.as_ref());
        let start = read(filename.as_ref());
        let result = read(&result_file);
        neander.memory = start;

        for _ in 1..100 {
            if !neander.step_code() { break; }
            //println!("{:?}", neander);
        }

        compare_mem(&neander.memory, &result);
        println!("What changed");
        compare_mem(&neander.memory, &start);
        
        assert!(neander.memory==result);
    }
}