
use wasm_bindgen::prelude::*;
use crate::neander::NeanderCPU;

#[wasm_bindgen]
pub struct NeanderJS {
    cpu: NeanderCPU
}

#[wasm_bindgen]
impl NeanderJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NeanderJS {
        NeanderJS{
            cpu: NeanderCPU{..Default::default()}
        }
    }
    
    pub fn get_acc(&self) -> u8 {
        return self.cpu.accumulator;
    }
    
}
