use wasm_bindgen::prelude::*;

use js_sys::Uint8Array;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::*;

pub struct CPU {
    pub memory: [u8; 0xFFFF],
    pub registers: Registers,
    pub stack: [u8; 0xC],
    pub program_counter: i16,
    pub timers: Timers,
}

pub struct Registers {
    pub general: [u8; 0xF],
    pub i: u8,
}

pub struct Timers {
    pub delay: u8,
    pub sound: u8,
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut(),
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = unsafe { setInterval(&closure, millis) };

        Interval { closure, token }
    }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        unsafe { cancelInterval(self.token) };
    }
}

pub fn main(current_cpu: CPU) {
    Interval::new(1, move || run(&current_cpu, current_cpu.program_counter));

    //     Interval::new(1000 / 60, || {
    //         if current_cpu.timers.delay > 0x0 {
    //             current_cpu.timers.delay = current_cpu.timers.delay - 1;
    //         }
    //         if current_cpu.timers.sound > 0x0 {
    //             current_cpu.timers.sound = current_cpu.timers.sound - 1;
    //         }

    //         // drawToContext();
    //     });
}

pub fn run(current_cpu: &CPU, current_address: i16) {
    let mut instruction_binary = String::new();

    for byte in &current_cpu.memory[current_address as usize..current_address as usize + 2] {
        console::log_1(&(*byte).into());

    }

    current_cpu.program_counter += 0x2;
}
