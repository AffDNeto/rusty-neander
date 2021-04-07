use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::common::{BasicALU, ExecuteCycle, Memory};

use super::core::AhmesEmulator;

#[derive(Serialize, Deserialize)]
pub struct ExportedAhmes{
    pub acc: u8, 
    pub pc: u8, 
    pub mem: Vec<u8>,
    pub zf: bool,
    pub nf: bool,
    pub vf: bool,
    pub cf: bool,
    pub bf: bool,
    pub mem_access_counter: usize,
    pub instruction_counter: usize
}

#[wasm_bindgen]
pub struct AhmesJS {
    cpu: AhmesEmulator
}

#[wasm_bindgen]
impl AhmesJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AhmesJS {
        AhmesJS{
            cpu: AhmesEmulator{..Default::default()}
        }
    }

    pub fn get_state(&self) -> JsValue {
        let cpu = ExportedAhmes{
            acc: self.cpu.read_register(1),
            pc: self.cpu.read_pc(),
            mem: self.cpu.mem.dump(),
            zf: self.cpu.zero_flag,
            nf: self.cpu.negative_flag,
            vf: self.cpu.overflow_flag,
            cf: self.cpu.carry_flag,
            bf: self.cpu.borrow_flag,
            mem_access_counter: self.cpu.mem.access_counter,
            instruction_counter: self.cpu.instruction_counter
        };
        JsValue::from_serde(&cpu).unwrap()
    }

    pub fn execute(&mut self, cycles: usize) -> bool{
        let mut result:bool = true;
        let mut cycle_count:usize = 0;

        while result && cycle_count < cycles {
            result = self.cpu.execute_cycle();
            cycle_count += 1;
        }

        return result;
    }

    pub fn set_pc(&mut self, new_pc: u8) {
        self.cpu.set_pc(new_pc);
    }

    pub fn set_acc(&mut self, new_acc: u8) {
        self.cpu.write_register(1, new_acc);
    }

    pub fn clear_counters(&mut self) {
        self.cpu.instruction_counter = 0;
        self.cpu.mem.access_counter = 0;
    }

    pub fn set_mem(&mut self, pos: u8, value: u8){
        self.cpu.mem._write(pos, value);
    }

    pub fn load_mem(&mut self, array:JsValue) {
        let elements: Vec<u8> = array.into_serde().unwrap();
        self.cpu.mem.load(elements)
    }

}

