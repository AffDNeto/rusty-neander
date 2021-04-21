use crate::common::alu_trait::ExtendedALU;
use crate::common::runner_trait::Runner;
use crate::common::register_trait::RegisterBank;
use crate::common::alu_trait::SimpleAlu;
use crate::common::memory_trait::Memory as MemoryExp;

#[derive(Debug)]
pub struct AhmesMachine{
    pub pc: u8,
    pub ri: [u8;2],
    pub rem: u8,
    pub rdm: u8,
    pub acc: u8,
    pub memory: [u8; 256],
    pub memory_access: usize,
    pub instruction_counter: usize,
    pub zero_flag: bool,
    pub negative_flag: bool,
    pub carry_flag: bool,
    pub borrow_flag: bool,
    pub overflow_flag: bool

}

impl Default for AhmesMachine {
    fn default() -> Self {
        AhmesMachine {
            pc: 0,
            ri: [0; 2],
            rem: 0,
            rdm: 0,
            acc: 0,
            memory: [0; 256],
            memory_access: 0,
            instruction_counter: 0,
            zero_flag: false,
            negative_flag: false,
            carry_flag: false,
            borrow_flag: false,
            overflow_flag: false
        }
    }
}

impl SimpleAlu for AhmesMachine {
    fn set_zero(&mut self, value:bool){self.zero_flag = value}
    fn set_negative(&mut self, value:bool){self.negative_flag = value}
    fn set_carry(&mut self, value:bool){self.carry_flag = value}
    fn set_borrow(&mut self, value:bool){self.borrow_flag = value}
    fn set_overflow(&mut self, value:bool){self.overflow_flag = value}
    fn get_zero(&self) -> bool {self.zero_flag}
    fn get_negative(&self) -> bool {self.negative_flag}
    fn get_carry(&self) -> bool {self.carry_flag}
    fn get_borrow(&self) -> bool {self.borrow_flag}
    fn get_overflow(&self) -> bool {self.overflow_flag}
}

impl MemoryExp for AhmesMachine{
    #[inline]
    fn _read(&self, rem: usize) -> u8{return self.memory[rem]}
    #[inline]
    fn _write(&mut self, rem: usize, value: u8){self.memory[rem] = value}
    #[inline]
    fn get_access_count(&self) -> usize{return self.memory_access}
    #[inline]
    fn set_access_count(&mut self, value: usize){self.memory_access = value}
    #[inline]
    fn get_rem(&self) -> u8{return self.rem;}
    #[inline]
    fn get_rdm(&self) -> u8{return self.rdm;}
    #[inline]
    fn set_rem(&mut self, value: u8){self.rem = value;}
    #[inline]
    fn set_rdm(&mut self, value: u8){self.rdm = value;}
}

impl Runner for AhmesMachine {
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
            0x2..=0x7 | 0xE
                => return self.ula_operation(),//ula operation
            0x8 => self._jmp_if(true), // JMP
            0x9..=0xB => return self.jump_operation(),
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

impl ExtendedALU for AhmesMachine {}

impl AhmesMachine {
    fn jump_operation(&mut self) -> bool {
        let jump_op = self.get_ri() & 0b1111_1100; // Ignore some bits
        match jump_op {
            0x90 => self._jmp_if(self.negative_flag),
            0x94 => self._jmp_if(!self.negative_flag),
            0x98 => self._jmp_if(self.overflow_flag),
            0x9C => self._jmp_if(!self.overflow_flag),
            0xA0 => self._jmp_if(self.zero_flag),
            0xA4 => self._jmp_if(!self.zero_flag),
            0xB0 => self._jmp_if(self.carry_flag),
            0xB4 => self._jmp_if(!self.carry_flag),
            0xB8 => self._jmp_if(self.borrow_flag),
            0xBC => self._jmp_if(!self.borrow_flag),
            _ => return false
        }
        return true
    }

    fn ula_operation(&mut self) -> bool {
        // Retrieve second operator from memory if necessary
        if let 0x20..=0x50 | 0x70 = self.get_ri() {
            self.get_operator_from_memory();    
        }
        let operation = (self.get_ri() & 0b1111_0000) >> 4;
        let a = self.get_register(self.decode_register());
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
            0x6 => result = self.not(a), //NOT
            0x7 => result = self.sub(a, b), //SUB
            0xE => { //Rotate or Shift
                println!("Shift {:#x}", self.get_ri());
                match self.get_ri() {
                    0xE0 => result = self.shr(a), 
                    0xE1 => result = self.shl(a), 
                    0xE2 => result = self.ror(a), 
                    0xE3 => result = self.rol(a),
                    _ => return false // Unknow operation
                }
                println!("{:#x} {:#x}", a, result);
            },
            _ => return false // Unknow operation
        }

        self.set_register(self.decode_register(), result);
        return true
    }
}

impl RegisterBank for AhmesMachine {
    #[inline]
    fn get_pc(&self) -> u8 {
        return self.pc;
    }
    #[inline]
    fn get_ri(&self) -> u8 {
        return self.ri[1];
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
    fn set_ri(&mut self, pos:u8, value: u8) {
        self.ri[0] = pos;
        self.ri[1] = value;
    }
    #[inline]
    fn set_register(&mut self, _: u8, value: u8) {
        self.acc = value;
    }
}


#[cfg(test)]
mod functional_tests {
    use differ::{Differ, Tag};
    use std::convert::TryInto;
    use std::{fs::{self, File}, io::Read};
    use std::path::{Path, PathBuf};    
    use super::*;
    use rstest::*;
    const CARGO_ROOT: &str = env!("CARGO_MANIFEST_DIR");

    fn ahmes_test_path() -> PathBuf {
        Path::new(CARGO_ROOT)
        .join("tests")
        .join("ahmes")
    }

    fn neander_test_path() -> PathBuf {
        Path::new(CARGO_ROOT)
        .join("tests")
        .join("neander")
    }
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
            .filter(|&(i, _) | i%2 == 0) // .mem format for 8bit memory  has 0 on every odd position
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
        println!("Reading file:");
        println!("{:?}", file_path.as_ref());
        let mem = array_to_256mem( mem_to_array(&file_path));

        mem.try_into()
        .unwrap_or_else(
            |v: Vec<u8>| 
            panic!("Expecteted len 256 but came {}", v.len()
            )
        )
    }
    
    #[rstest(filename,
        case::add("add_test.mem"),
        case::and("and_test.mem"),
        case::jmp("jmp_test.mem"),
        case::jb("jb_test.mem"),
        case::jv("jv_test.mem"),
        case::jc("jc_test.mem"),
        case::jz("jz_test.mem"),
        case::jn("jn_test.mem"),
        case::lda("lda_test.mem"),
        case::not("not_test.mem"),
        case::or("or_test.mem"),
        case::ror("ror_test.mem"),
        case::rol("rol_test.mem"),
        case::shr("shr_test.mem"),
        case::shl("shl_test.mem"),
        case::sta("sta_test.mem"),
        case::sub("sub_test.mem")
    )]
    fn ahmes_test(filename: impl AsRef<str>){
        let mut ahmes = AhmesMachine{..Default::default()};
        let tests_path = ahmes_test_path();
        let mut result_file = "result.".to_owned();
        result_file.push_str(filename.as_ref());
        
        let start = read(&tests_path.join(filename.as_ref()));
        let result = read(&tests_path.join(&result_file));
        ahmes.memory  = start;

        for _ in 1..200 {
            println!("{:?}", ahmes);
            if !ahmes.step_code() { break; }
        }
        println!("{:?}", ahmes);

        compare_mem(&ahmes.memory , &result);
        println!("What changed");
        compare_mem(&ahmes.memory , &start);
        
        assert!(ahmes.memory ==result);
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
    fn basic_neander_test(filename: impl AsRef<str>){
        let mut ahmes = AhmesMachine{..Default::default()};
        let tests_path = neander_test_path();
        let mut result_file = "result.".to_owned();
        result_file.push_str(filename.as_ref());
        
        let start = read(&tests_path.join(filename.as_ref()));
        let result = read(&tests_path.join(&result_file));
        ahmes.memory  = start;

        for _ in 1..100 {
            if !ahmes.step_code() { break; }
            //println!("{:?}", ahmes);
        }

        compare_mem(&ahmes.memory , &result);
        println!("What changed");
        compare_mem(&ahmes.memory , &start);
        
        assert!(ahmes.memory ==result);
    }
}