use wasm_bindgen::prelude::*;
#[path = "structs/cpu.rs"]
mod cpu;
use cpu::*;

const VRAM_START_ADDRESS: i32 = 0x6A0;
const WIDTH: i32 = 64;
const HEIGHT: i32 = 32;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("for Window to exist");
    let document = window.document().expect("To have a document");

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let current_cpu_registers = Registers {
        general: [0; 0xF],
        i: 0
    };

    let current_cpu_timers = Timers {
        delay: 0,
        sound: 0
    };

    let current_cpu = CPU {
        memory: [0; 0xFFFF],
        instrucion_pointer: 0,
        stack: [0; 0xC],
        registers: current_cpu_registers,
        timers: current_cpu_timers
    };

    // TODO: Implement File Loading
    // TODO: Implement CPU Instructions
    // TODO: Implement Draw to Canvas
    // TODO: Implement Control
    // TODO: Implement Sound



    Ok(())
}
