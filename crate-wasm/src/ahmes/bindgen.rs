use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::ahmes::core::AhmesMachine;
use crate::common::{
    memory_trait::Memory,
    register_trait::RegisterBank,
    runner_trait::Runner
};
use std::convert::TryInto;

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
    cpu: AhmesMachine
}

#[wasm_bindgen]
impl AhmesJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AhmesJS {
        AhmesJS{
            cpu: AhmesMachine{..Default::default()}
        }
    }

    pub fn get_state(&self) -> JsValue {
        let cpu = ExportedAhmes{
            acc: self.cpu.get_register(0),
            pc: self.cpu.get_pc(),
            mem: self.cpu.memory.to_vec(),
            zf: self.cpu.zero_flag,
            nf: self.cpu.negative_flag,
            vf: self.cpu.overflow_flag,
            cf: self.cpu.carry_flag,
            bf: self.cpu.borrow_flag,
            mem_access_counter: self.cpu.memory_access,
            instruction_counter: self.cpu.instruction_counter
        };
        JsValue::from_serde(&cpu).unwrap()
    }

    pub fn execute(&mut self, cycles: usize) -> bool{
        let mut result:bool = true;
        let mut cycle_count:usize = 0;

        while result && cycle_count < cycles {
            result = self.cpu.step_code();
            cycle_count += 1;
        }

        return result;
    }

    pub fn set_pc(&mut self, new_pc: u8) {
        self.cpu.set_pc(new_pc);
    }

    pub fn set_acc(&mut self, new_acc: u8) {
        self.cpu.set_register(0, new_acc);
    }

    pub fn clear_counters(&mut self) {
        self.cpu.instruction_counter = 0;
        self.cpu.memory_access = 0;
    }

    pub fn set_mem(&mut self, pos: usize, value: u8){
        self.cpu._write(pos, value);
    }

    pub fn load_mem(&mut self, array:JsValue) {
        let elements: Vec<u8> = array.into_serde().unwrap();
        self.cpu.memory = elements.try_into()
            .unwrap_or_else(
                |v: Vec<u8>|
                    panic!("Expecteted len 256 but came {}", v.len()
                    )
            );
    }

}

