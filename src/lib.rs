mod neander_bindgen; 
mod neander;
use neander_bindgen::NeanderJS;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let p = NeanderJS::new();
    log(&format!("{}", p.get_acc()));
    log("Hello finally!")
}
