use crate::loopr::web;
use wasm_bindgen::prelude::*;
use crate::audio::{FmOsc};

pub struct Sequencer {
    i: i64,
    steps: Vec<u8>,
    // voice: FmOsc,
    n: i64,
}

pub fn run(callback: fn(i32)) -> Result<(), JsValue> {
    
    let mut s = Sequencer{
        i: 0,
        steps: vec!{60, 62, 67, 69},
        n: 4,
        // voice: FmOsc{

        // }
    };

    let tick = move |val| {
        if s.i == s.n {
            s.i=0
        };

    };
    
    web::run(tick);
    Ok(())
}