use neader::NeanderCPU;
use wasm_bindgen::prelude::*;
mod neader;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log("Hello finally!")
}

static mut NEANDER_CPU: NeanderCPU = NeanderCPU{..Default::default()};

#[wasm_bindgen]
pub fn get_accummulator() -> u8 {
    return NEANDER_CPU.accumulator;
}


