use wasm_bindgen::prelude::*;
use neander;

#[wasm_bindgen]
struct NeanderJS {
    cpu: NeanderCPU
}

#[wasm_bindgen]
impl NeanderJS {
    pub fn new() -> NeanderJS {
        NeanderJS{
            cpu: NeanderJS{..Default::default()}
        }
    }

    pub fn get_acc(&self) -> u8 {
        self.cpu.accumulator;
    }

}
