use crate::common::alu_trait::ExtendedALU;
use crate::common::register_trait::Runner;
use crate::common::register_trait::RegisterBank;
use crate::common::alu_trait::SimpleAlu;
use crate::common::memory_trait::Memory as MemoryExp;
use crate::common::{AhmesInstructions, BasicALU, ExecuteCycle, Memory, Memory256, NeanderOperations};

#[derive(Debug)]
pub struct AhmesEmulator {
    pub mem: Memory256,
    pub registers: [u8; 2],
    pub ri:u8,
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
            ri: 0,
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
        let op = (op_code & 0b1111_0000) >> 4;
        self.ri = op_code;
        let jmp_op = ( op_code & 0b0000_1100) >> 2;
        let shift_op = op_code & 0b0000_0011;
        self.instruction_counter += 1;

        match op {
            0x0 => return self.no_operation(),
            0x1 => return self.store(),
            0x2 => return self.load(),
            0x3 => return self.add(),
            0x4 => return self.or(),
            0x5 => return self.and(),
            0x6 => return self.not(),
            0x7 => return self.sub(),
            0x8 => return self.jump(),
            0x9 => {
                match jmp_op {
                    0x0 => return self.jump_negative(),
                    0x1 => return self.jump_non_negative(),
                    0x2 => return self.jump_overflow(),
                    0x3 => return self.jump_non_overflow(),
                    _ => false
                }
            },
            0xA => {
                match jmp_op {
                    0x0 => return self.jump_zero(),
                    0x1 => return self.jump_non_zero(),
                    _ => false
                }
            },
            0xB => {
                match jmp_op {                    
                    0x0 => return self.jump_carry(),
                    0x1 => return self.jump_non_carry(),
                    0x2 => return self.jump_borrow(),
                    0x3 => return self.jump_non_borrow(),
                    _ => false
                }
            },
            0xE => {
                match shift_op {                    
                    0x0 => return self.shift_right(),
                    0x1 => return self.shift_left(),
                    0x2 => return self.rotate_right(),
                    0x3 => return self.rotate_left(),
                    _ => false
                }
            },
            0xF => return self.halt(),
            _ => {
                println!("Whoops! {} {}", op, op_code);
                return self.halt()
            }
        }
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
    fn sub(&mut self) -> bool {
        let pos = self.next_instruction();
        let value = self.mem.direct_read(pos);
        let acc = self.read_register(1);
        let (result, carry) = acc.overflowing_sub(value);
        let (_, overflow) = (acc as i8).overflowing_sub(value as i8);
        
        self.write_register(1, result);
        self.set_flags(result);
        self.overflow_flag = overflow;
        self.carry_flag = carry;
        return true
    }
    
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

#[derive(Debug)]
pub struct AhmesMachine{
    pub pc: u8,
    pub ri: u8,
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
            ri: 0,
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

        self.set_register(self.ri_reg(), result);
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