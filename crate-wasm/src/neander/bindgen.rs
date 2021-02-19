use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::{common::{memory_trait::Memory, register_trait::{RegisterBank, Runner}}};

use super::NeanderExp;

#[wasm_bindgen]
pub struct NeanderJS {
    cpu: NeanderExp
}

#[derive(Serialize, Deserialize)]
pub struct ExportedNeander{
    pub acc: u8, 
    pub pc: u8, 
    pub mem: Vec<u8>,
    pub zf: bool,
    pub nf: bool,
    pub mem_access_counter: usize,
    pub instruction_counter: usize
}

#[wasm_bindgen]
impl NeanderJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NeanderJS {
        NeanderJS{
            cpu: NeanderExp{..Default::default()}
        }
    }
    
    pub fn get_state(&self) -> JsValue {
        let cpu = ExportedNeander{
            acc: self.cpu.get_register(0),
            pc: self.cpu.get_pc(),
            mem: self.cpu.memory.to_vec(),
            zf: self.cpu.zero_flag,
            nf: self.cpu.negative_flag,
            mem_access_counter: self.cpu.memory_access,
            instruction_counter: self.cpu.instruction_counter
        };
        JsValue::from_serde(&cpu).unwrap()
    }
    
    pub fn execute(&mut self, cycles: usize){
        let mut result:bool = true;
        let mut cycle_count:usize = 0;

        while result && cycle_count < cycles {
            result = self.cpu.step_code();
            cycle_count += 1;
        }
    }

    pub fn set_pc(&mut self, new_pc: u8) {
        self.cpu.set_pc(new_pc);
    }

    pub fn set_acc(&mut self, new_acc: u8) {
        self.cpu.set_register(0, new_acc);
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
