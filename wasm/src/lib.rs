mod audio;
mod loopr;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-audio!");
}

// This function is automatically invoked after the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    
    let tick = |val| {
        console_log(format!("{:?}",val));
    };
    loopr::sequencer::run(tick)
}

fn console_log(s:String){
    console::log_1(&s.into());
}
