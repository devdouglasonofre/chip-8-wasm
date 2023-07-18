use wasm_bindgen::prelude::*;
#[path = "structs/cpu.rs"]
mod cpu;
use cpu::*;

use js_sys::{Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::*;

const VRAM_START_ADDRESS: i32 = 0x6A0;
const WIDTH: i32 = 64;
const HEIGHT: i32 = 32;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    
    let mut current_cpu_registers = Registers {
        general: [0; 0xF],
        i: 0,
    };

    let mut current_cpu_timers = Timers { delay: 0, sound: 0 };

    let mut current_cpu = CPU {
        memory: [0; 0xFFFF],
        instrucion_pointer: 0,
        stack: [0; 0xC],
        registers: current_cpu_registers,
        timers: current_cpu_timers,
    };

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

    let move_rom_to_ram = Closure::wrap(Box::new(move |data: JsValue| {
        let file_view = Uint8Array::new(&data);
        let file_view_vec = file_view.to_vec();

        let mut i: u32 = 0;

        while (i < file_view.length()) {
            current_cpu.memory[0x200 + i as usize] = file_view_vec[i as usize];

            i = i + 1;
        }
    }) as Box<dyn FnMut(_)>);

    let  load_rom = Closure::wrap(Box::new(move |event: Event| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let filelist = element.files().unwrap();

        let _file = filelist.get(0).expect("should have a file handle.");

        let _file_buffer = _file.array_buffer().then(&move_rom_to_ram);

        
    }) as Box<dyn FnMut(_)>);

    document
        .get_element_by_id("input_file")
        .expect("To have #input_file")
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .set_oninput(Some(load_rom.as_ref().unchecked_ref()));

    load_rom.forget();

    // TODO: Implement File Loading DONE
    // TODO: Implement CPU Instructions
    // TODO: Implement Draw to Canvas
    // TODO: Implement Control
    // TODO: Implement Sound

    Ok(())
}
