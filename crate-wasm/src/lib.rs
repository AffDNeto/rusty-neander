mod neander;
mod ahmes;
mod ramses;
mod cesar;
mod common;
use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {
            let _ = env_logger::builder().is_test(true).try_init();
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    init_log();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn greet() {
    log("Canary hello!")
}
