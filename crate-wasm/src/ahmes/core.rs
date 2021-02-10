use crate::common::Memory256;

pub struct AhmesEmulator {
    pub mem: Memory256,
    pub registers: [u8; 2],
    pub negative_flag: bool,
    pub zero_flag: bool,
    pub carry_flag: bool,
    pub borrow_flag: bool,
    pub overflow_flag: bool,
    pub instruction_counter: u8
}