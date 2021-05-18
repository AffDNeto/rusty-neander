use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::common::{
    memory_trait::Memory,
    register_trait::RegisterBank,
    runner_trait::Runner
};

use crate::ramses::core::RamsesMachine;

#[wasm_bindgen]
pub struct RamsesJsInterface {
    cpu: RamsesMachine
}

#[derive(Serialize, Deserialize)]
pub struct ExportedRamses {
    pub ra: u8,
    pub rb: u8,
    pub rx: u8,
    pub pc: u8,
    pub mem: Vec<u8>,
    pub ri: Vec<u8>,
    pub zf: bool,
    pub nf: bool,
    pub cf: bool,
    pub mem_access_counter: usize,
    pub instruction_counter: usize
}

#[wasm_bindgen]
impl RamsesJsInterface {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RamsesJsInterface {
        RamsesJsInterface {
            cpu: RamsesMachine{..Default::default()}
        }
    }

    pub fn get_state(&self) -> JsValue {
        let cpu = ExportedRamses {
            ra: self.cpu.get_register(0),
            rb: self.cpu.get_register(1),
            rx: self.cpu.get_register(2),
            pc: self.cpu.get_pc(),
            mem: self.cpu.memory.to_vec(),
            ri: self.cpu.ri.to_vec(),
            zf: self.cpu.zero_flag,
            nf: self.cpu.negative_flag,
            cf: self.cpu.carry_flag,
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

    pub fn clear_counters(&mut self) {
        self.cpu.instruction_counter = 0;
        self.cpu.memory_access = 0;
        self.cpu.pc = 0;
    }

    pub fn set_register(&mut self, id: u8, new_value: u8) {
        self.cpu.set_register(id, new_value);
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
