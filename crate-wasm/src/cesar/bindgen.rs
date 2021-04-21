use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::cesar::core::CesarProcessor;

#[wasm_bindgen]
pub struct CesarJsInterface {
    cpu: CesarProcessor
}

#[derive(Serialize, Deserialize)]
pub struct ExportedCesar {
    pub rx: [u16; 8],
    pub pc: u16,
    pub ri: Vec<u8>,
    pub zf: bool,
    pub nf: bool,
    pub cf: bool,
    pub vf: bool,
    pub mem_access_counter: usize,
    pub instruction_counter: usize
}

#[wasm_bindgen]
impl CesarJsInterface {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CesarJsInterface {
        CesarJsInterface {
            cpu: CesarProcessor { ..Default::default() }
        }
    }
    pub fn get_memory(&self) -> *const u8 {
        return self.cpu.memory.bank.as_ptr();
    }

    pub fn get_state(&self) -> JsValue {
        let cpu = ExportedCesar {
            rx: self.cpu.rx,
            pc: self.cpu.rx[7],
            ri: self.cpu.decoder.ri.to_vec(),
            zf: self.cpu.flags.z,
            nf: self.cpu.flags.n,
            cf: self.cpu.flags.c,
            vf: self.cpu.flags.v,
            mem_access_counter: self.cpu.memory.access_count,
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

    pub fn set_pc(&mut self, new_pc: u16) {
        self.cpu.rx[7] = new_pc;
    }

    pub fn clear_counters(&mut self) {
        self.cpu.instruction_counter = 0;
        self.cpu.memory.access_count = 0;
    }

    pub fn set_register(&mut self, id: usize, new_value: u16) {
        self.cpu.rx[id] = new_value;
    }

    pub fn set_mem(&mut self, pos: usize, value: u8){
        self.cpu.memory.bank[pos] = value;
    }

    pub fn load_mem(&mut self, array:JsValue) {
        let elements: Vec<u8> = array.into_serde().unwrap();
        self.cpu.memory.bank = elements.try_into()
            .unwrap_or_else(
                |v: Vec<u8>|
                    panic!("Expecteted len 65536 but came {}", v.len()
                    )
            );
    }

    pub fn input_keyboard(&mut self, letter: u8) {
        self.cpu.keyboard_interrupt(letter);
    }

    pub fn get_visor(&self) -> String {
        return self.cpu.get_visor();
    }
}
