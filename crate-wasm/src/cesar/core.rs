
use crate::cesar::{BranchType, OneOperandType, TwoOperandType};
use crate::cesar::AddressMode;
use crate::cesar::ConditionFlags;
use crate::cesar::Instruction;
use crate::cesar::decoder::CesarDecoder;
use crate::cesar::operations::*;
use log::trace;
use std::fmt;


#[derive(Debug)]
pub struct CesarProcessor {
    pub rx: [u16; 8],
    pub decoder: CesarDecoder,
    pub instruction_counter: usize,
    pub memory: MemoryBank,
    pub flags: ConditionFlags
}

impl Default for CesarProcessor {
    fn default() -> Self {
        CesarProcessor {
            rx: [0; 8],
            decoder: CesarDecoder{ ri: [0,0] },
            instruction_counter: 0,
            memory: MemoryBank{..Default::default()},
            flags: ConditionFlags{ n:false, z:false, v:false, c:false }
        }
    }
}

impl CesarProcessor {

    /// runs a complete cycle of the processor
    /// Return value is signalizes if the program can continue to run 
    /// i.e.: last instruction was not a halt and no error happened
    fn step_code(&mut self) -> bool {
        trace!("Running next cycle of processor: {:?}", self);
        self.instruction_counter += 1;
        let instruction = self.get_next_instruction();
        
        match instruction {
            Instruction::Nop => return true,
            Instruction::SetCondition(_) => return self.execute_scc(instruction),
            Instruction::ClearCondition(_) => return self.execute_ccc(instruction),
            Instruction::Jump{..} => return self.execute_jump(instruction),
            Instruction::Branch{..} => return self.execute_branch(instruction),
            Instruction::Loop{..} => return self.execute_loop(instruction),
            Instruction::BranchSubroutine{..} => return self.execute_jsr(instruction),
            Instruction::ReturnSubroutine{..} => return self.execute_rts(instruction),
            Instruction::OneOperand{..} => return self.execute_one_op(instruction),
            Instruction::TwoOperand{..} => return self.execute_two_op(instruction),
            Instruction::Halt => return false
        }
    }

    /// Reads byte from the address given and returns it
    fn read_byte(&mut self, address: u16) -> u8{
        self.memory.rem = address;
        self.memory.read();
        trace!("Read byte {a:#x},{a} in address {b:#x},{b}", a=self.memory.rdm, b=address);
        return self.memory.rdm;
    }
    
    /// Reads a word(16bits) from the address given and return it
    fn read_word(&mut self, address: u16) -> u16{
        let msb = self.read_byte(address) as u16;
        let lsb = self.read_byte(address+1) as u16;
        let word = lsb | (msb << 8);
        trace!("Read word {a:#x},{a} at {b:#x},{b}", a=word, b=address);
        return word;
    }

    /// Writes a word(16bit) on the address given, 
    /// split the word in bytes properly before writing it
    fn write_word(&mut self, address: u16, value: u16) {
        trace!("Writing word {a:#x},{a} in address {b:#x},{b}", a=value, b=address);
        let msb = ( value >> 8 ) as u8;
        let lsb = value as u8; // Most significant bit will just be truncated
        self.memory.rem = address;
        self.memory.rdm = msb;
        self.memory.write();
        
        self.memory.rem = address+1;
        self.memory.rdm = lsb;
        self.memory.write();
    }

    /// Decodes the address to be read/writen from memory 
    /// based on the register and mode, also updates register 
    /// values along the way also.
    fn get_address_with_mode(&mut self, register: u8, mode: AddressMode) -> u16 {
        trace!("Getting address of register {} with mode {:?}", register, mode);
        let rid = register as usize;
        let address: u16;
        match mode {
            AddressMode::Register => panic!("Tried to get an address, but the Address mode is Register"),
            AddressMode::PosInc => {
                address = self.rx[rid];
                self.rx[rid] = self.rx[rid].wrapping_add(2);
            },
            AddressMode::PreDec => {
                self.rx[rid] = self.rx[rid].wrapping_sub(2);
                address = self.rx[rid];
            },
            AddressMode::Index => {
                let index = self.read_word(self.rx[7]);
                self.rx[7] = self.rx[7].wrapping_add(2);
                address = self.rx[rid].wrapping_add(index);
            },
            AddressMode::Indirect => {address = self.rx[rid];},
            AddressMode::IndirectPosInc => {
                address = self.read_word(self.rx[rid]);
                self.rx[rid] = self.rx[rid].wrapping_add(2);

            },
            AddressMode::IndirectPreDec => {
                self.rx[rid] = self.rx[rid].wrapping_sub(2);
                address = self.read_word(self.rx[rid]);
            },
            AddressMode::IndirectIndex => {
                let index = self.read_word(self.rx[7]);
                self.rx[7] = self.rx[7].wrapping_add(2);
                trace!("Summing {} with {} = {}", self.rx[rid], index, self.rx[rid].wrapping_add(index));
                address = self.read_word(self.rx[rid].wrapping_add(index));
            }
        };
        trace!("Returning address {a:},{a:#x}", a=address);
        return address;
    }
    /// Reads a word(16bits) from memory pointed by the register and the mode
    /// Also updates the register based on the mode used
    /// Returns the word read and the address in this order, 
    /// in case the address needs to be used later
    fn read_word_with_mode(&mut self, register: u8, mode: AddressMode) -> (u16,Option<u16>) {
        trace!("Reading word from register {}, with mode {:?}", register, mode);
        let rx_id = register as usize;
        let word: u16;
        let address: u16;
        match mode {
            AddressMode::Register => return (self.rx[rx_id], None),
            _ => {
                address = self.get_address_with_mode(register, mode);
                word = self.read_word(address)
            },
        }

        trace!("Word: {a:#x}, {a}, read at address {b:#x}, {b}", a=word, b=address);
        return (word, Some(address));
    }
    
    /// Retrieves from memory the next instruction based on the register 7
    /// and increments it accordingly
    fn get_next_instruction(&mut self) -> Instruction {
        trace!("Searching next instruction: {a:#04x},{a}", a=self.rx[7]);
        self.decoder.ri[0] = self.read_byte(self.rx[7]);
        // Case the program pointer overflows, halt the program
        let new_pc = self.rx[7].overflowing_add(1);
        if new_pc.1 { return Instruction::Halt } else { self.rx[7] = new_pc.0 }
        if self.decoder.is_single_byte_instruction() {
            //Cleans second byte of the decoder
            self.decoder.ri[1] = 0;
        }else{
            self.decoder.ri[1] = self.read_byte(self.rx[7]);

            // Case the program pointer overflows, halt the program
            let new_pc = self.rx[7].overflowing_add(1);
            if new_pc.1 { return Instruction::Halt } else { self.rx[7] = new_pc.0 }
        }

        self.decoder.instruction()
    }

    /// Executes the instruction SCC and sets the instructed flags
    fn execute_scc(&mut self, instruction: Instruction) -> bool {
        trace!("Running set flags instruction: {:?}", instruction);
        if let Instruction::SetCondition(flags) = instruction {
            if flags.n{self.flags.n = true;}
            if flags.z{self.flags.z = true;}
            if flags.v{self.flags.v = true;}
            if flags.c{self.flags.c = true;}
            return true;
        }else{
            panic!("Tried to run SCC received {:?}", instruction);
        }
    }
    
    /// Executes the instruction CCC and clears the instructed flags
    fn execute_ccc(&mut self, instruction: Instruction) -> bool {
        trace!("Running clear flags instruction: {:?}", instruction);
        if let Instruction::ClearCondition(flags) = instruction {
            if flags.n{self.flags.n = false;}
            if flags.z{self.flags.z = false;}
            if flags.v{self.flags.v = false;}
            if flags.c{self.flags.c = false;}
            return true;
        }else{
            panic!("Tried to run CCC received {:?}", instruction);
        }
    }

    /// Updates the PC(r7) register with the address instructed
    /// effectvely jumping on the program
    fn execute_jump(&mut self, instruction: Instruction) -> bool {
        trace!("Running jump instruction: {:?}", instruction);
        if let Instruction::Jump{rx, mode} = instruction {
            let new_pc = self.get_address_with_mode(rx, mode);
            self.rx[7] = new_pc;
            return true;
        }else{
            panic!("Tried to execute jump but received {:?}", instruction);
        }
    }
    
    fn execute_branch(&mut self, instruction: Instruction) -> bool {
        trace!("Running branch instruction: {:?}", instruction);
        trace!("Condition flags: {:?}", self.flags);
        if let Instruction::Branch{displacement, kind} = instruction {
            let do_branch = match kind {
                BranchType::Br => true,
                BranchType::Bne => !self.flags.z ,
                BranchType::Beq => self.flags.z,
                BranchType::Bpl => !self.flags.n,
                BranchType::Bmi => self.flags.n,
                BranchType::Bvc => !self.flags.v,
                BranchType::Bvs => self.flags.v,
                BranchType::Bcc => !self.flags.c,
                BranchType::Bcs => self.flags.c,
                BranchType::Bge => self.flags.n == self.flags.v,
                BranchType::Blt => self.flags.n != self.flags.v,
                BranchType::Bgt => self.flags.n == self.flags.v && !self.flags.z,
                BranchType::Ble => self.flags.n != self.flags.v || self.flags.z,
                BranchType::Bhi => !self.flags.c && !self.flags.z,
                BranchType::Bls => self.flags.c || self.flags.z,
                BranchType::Nop => false
            };

            if do_branch {
                // Multiple conversions needed to correctly calculate the new PC with two complement
                self.rx[7] = ((self.rx[7] as i16 ) + (displacement as i8) as i16) as u16;
            }
            return true
        }else{
            panic!("Tried to execute branch but received {:?}", instruction);
        }
    }
    
    fn execute_loop(&mut self, instruction: Instruction) -> bool {
        trace!("Running loop instruction: {:?}", instruction);
        if let Instruction::Loop{rx, displacement} = instruction {
            self.rx[rx as usize] -= 1;
            if self.rx[rx as usize] != 0 {
                // Multiple conversions needed to correctly calculate the new PC with two complement
                self.rx[7] = ((self.rx[7] as i16 ) + (displacement as i8) as i16) as u16;
            }
            return false;
        }else{
            panic!("Tried to execute _ but received {:?}", instruction)
        }
    }

    fn execute_jsr(&mut self, instruction: Instruction) -> bool {
        trace!("Running JSR instruction: {:?}", instruction);
        if let Instruction::BranchSubroutine{r1, r2, mode} = instruction {
            let (temp, address) = self.read_word_with_mode(r2, mode);
            
            // If the address mode is Register this instruction is ignored.
            if let None = address { return true }
            
            // Push value to stack of the program using r6 as the stack pointer
            let stack_top = self.get_address_with_mode(6, AddressMode::PreDec);
            self.write_word(stack_top, self.rx[r1 as usize]);
            self.rx[r1 as usize] = self.rx[7];
            self.rx[7] = temp;
            return true;
        }else{
            panic!("Tried to execute _ but received {:?}", instruction)
        }
    }

    fn execute_rts(&mut self, instruction: Instruction) -> bool {
        trace!("Running RTS instruction: {:?}", instruction);
        if let Instruction::ReturnSubroutine{rx} = instruction {
            self.rx[7] = self.rx[rx as usize];
            self.rx[rx as usize] = self.read_word_with_mode(6, AddressMode::PosInc).0;
            return true;
        }else{
            panic!("Tried to execute _ but received {:?}", instruction)
        }
    }

    fn execute_one_op(&mut self, instruction: Instruction) -> bool {
        trace!("Running single operand instruction: {:?}", instruction);
        if let Instruction::OneOperand{rx, mode, kind} = instruction {
            let (value, address) = self.read_word_with_mode(rx, mode);
            let result: u16;
            match kind {
                // Test instruction doesn't write anything
                OneOperandType::Tst => { tst(value, &mut self.flags); return true }
                OneOperandType::Clr => { result = clr(&mut self.flags) }
                OneOperandType::Not => { result = not(value, &mut self.flags) }
                OneOperandType::Inc => { result = inc(value, &mut self.flags) }
                OneOperandType::Dec => { result = dec(value, &mut self.flags) }
                OneOperandType::Neg => { result = neg(value, &mut self.flags) }
                OneOperandType::Ror => { result = ror(value, &mut self.flags) }
                OneOperandType::Rol => { result = rol(value, &mut self.flags) }
                OneOperandType::Asr => { result = asr(value, &mut self.flags) }
                OneOperandType::Asl => { result = asl(value, &mut self.flags) }
                OneOperandType::Adc => { result = adc(value, &mut self.flags) }
                OneOperandType::Sbc => { result = sbc(value, &mut self.flags) }
                OneOperandType::Nop => { return true }
            }

            if let Some(a) = address {
                self.write_word(result, a);
            }else{
                self.rx[rx as usize] = result;
            }

            return true;
        }else{
            panic!("Tried to execute _ but received {:?}", instruction)
        }
    }

    fn execute_two_op(&mut self, instruction: Instruction) -> bool {
        trace!("Running two operand instruction: {:?}", instruction);
        if let Instruction::TwoOperand {r1, r2, mode1, mode2, kind} = instruction {
            let (src_value, _) = self.read_word_with_mode(r1, mode1);
            if let TwoOperandType::Mov = kind {
                mov(src_value, &mut self.flags); // update flags with value being moved
                if let AddressMode::Register = mode2 {
                    self.rx[r2 as usize] = src_value;
                }else{
                    let dst_address = self.get_address_with_mode(r2, mode2);
                    self.write_word(dst_address, src_value);
                }
                return true;
            }
            let (dst_value, dst_address) = self.read_word_with_mode(r2, mode2);
            let result;
            match kind {
                TwoOperandType::Add => {
                    result = add(src_value, dst_value, &mut self.flags)
                }
                TwoOperandType::Sub => {
                    result = sub(dst_value, src_value, &mut self.flags)
                }
                TwoOperandType::And => {
                    result = and(src_value, dst_value, &mut self.flags)
                }
                TwoOperandType::Or => {
                    result = or(src_value, dst_value, &mut self.flags)
                }
                TwoOperandType::Cmp => { cmp(src_value, dst_value, &mut self.flags); return true }
                _ => { return true }
            }

            if let Some(a) = dst_address {
                self.write_word(a, result);
            }else {
                self.rx[r2 as usize] = result;
            }
            return true;
        }else{
            panic!("Tried to execute _ but received {:?}", instruction)
        }
    }
}

pub struct MemoryBank {
    pub rem: u16,
    pub rdm: u8,
    pub access_count: usize,
    bank: [u8; 65536]
}

impl fmt::Debug for MemoryBank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryBank")
            .field("rem", &self.rem)
            .field("rdm", &self.rdm)
            .field("access_count", &self.access_count)
            .finish()
    }
}
impl Default for MemoryBank{
    fn default() -> Self { 
        MemoryBank {
            rem: 0,
            rdm: 0,
            access_count: 0,
            bank: [0; 65536]
        }
     }
}
impl MemoryBank{
    fn inc_access_count(&mut self){ self.access_count += 1;}
    fn read(&mut self){
        self.inc_access_count();
        self.rdm = self.bank[self.rem as usize];
    }
    fn write(&mut self){
        self.inc_access_count();
        self.bank[self.rem as usize] = self.rdm;
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

    fn cesar_test_path() -> PathBuf {
        Path::new(CARGO_ROOT)
        .join("tests")
        .join("cesar")
    }

    /// Reads a bynary file to an u8 vector
    fn mem_to_array<T: AsRef<Path>>(filename: T) -> Vec<u8> {
        let mut f = File::open(&filename).expect("No file found");
        let metadata = fs::metadata(&filename).expect("Unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];

        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }

    fn array_to_65kmem(array: Vec<u8>) -> Vec<u8> {
        array.into_iter()
            .skip(4) // First 4 bytes is the file header
            .collect()
    }

    fn compare_mem(a: &[u8], b: &[u8]){
        let diff =  Differ::new(&a, &b);
        for span in diff.spans() {
            match span.tag {
                Tag::Equal => (),
                _ => {
                    let end = if (span.b_end-span.b_start) > 100 {
                        span.b_start + 100
                    }else if span.b_end == span.b_start { span.b_start + 1 } else { span.b_end };
                    println!("{} found from {} to {}", span.tag, span.b_start, span.b_end);
                    println!("Want:{:?}", &b[span.b_start..end]);
                    println!("Got :{:?}", &a[span.b_start..end]);
                }
            }
        }
    }

    fn read<T: AsRef<Path>>(file_path: T) -> [u8; 65536] {
        println!("Reading file:");
        println!("{:?}", file_path.as_ref());
        let mem = array_to_65kmem( mem_to_array(&file_path));

        mem.try_into()
        .unwrap_or_else(
            |v: Vec<u8>| 
            panic!("Expecteted len 65536 but came {}", v.len()
            )
        )
    }
    
    #[rstest(filename,
        case::adc("adc_test.mem"),
        case::add("add_test.mem"),
        case::and("and_test.mem"),
        case::asl("asl_test.mem"),
        case::asr("asr_test.mem"),
        case::bcc("bcc_test.mem"),
        case::bcs("bcs_test.mem"),
        case::beq("beq_test.mem"),
        case::bge("bge_test.mem"),
        case::bgt("bgt_test.mem"),
        case::bhi("bhi_test.mem"),
        case::ble("ble_test.mem"),
        case::bls("bls_test.mem"),
        case::blt("blt_test.mem"),
        case::bmi("bmi_test.mem"),
        case::bne("bne_test.mem"),
        case::bpl("bpl_test.mem"),
        case::br("br_test.mem"),
        case::bvc("bvc_test.mem"),
        case::bvs("bvs_test.mem"),
        case::ccc("ccc_test.mem"),
        case::clr("clr_test.mem"),
        case::cmp("cmp_test.mem"),
        case::dec("dec_test.mem"),
        case::inc("inc_test.mem"),
        case::jmp("jmp_test.mem"),
        case::jsr_rts("jsr_rts_test.mem"),
        case::mov("mov_test.mem"),
        case::neg("neg_test.mem"),
        case::nophlt("nophlt_test.mem"),
        case::not("not_test.mem"),
        case::or("or_test.mem"),
        case::rol("rol_test.mem"),
        case::ror("ror_test.mem"),
        case::sbc("sbc_test.mem"),
        case::scc("scc_test.mem"),
        case::sob("sob_test.mem"),
        case::sub("sub_test.mem"),
        case::tst("tst_test.mem")
    )]
    fn cesar_test(filename: impl AsRef<str>){
        init_logger();
        let _ = env_logger::builder().is_test(true).try_init();
        let mut processor = CesarProcessor{..Default::default()};
        let tests_path = cesar_test_path();
        let mut result_file = "result.".to_owned();
        result_file.push_str(filename.as_ref());
        
        let start = read(&tests_path.join(filename.as_ref()));
        let result = read(&tests_path.join(&result_file));
        processor.memory.bank  = start;

        for _ in 1..200 {
            //println!("{:?}", processor);
            if !processor.step_code() { break; }
        }

        println!("{:?}", processor);
        compare_mem(&processor.memory.bank , &result);
        // println!("What changed");
        //compare_mem(&processor.memory.bank , &start);
        assert!(processor.memory.bank == result);
    }

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }
}

