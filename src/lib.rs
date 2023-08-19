use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

mod types;

use types::triangle::Triangle;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let init_points = [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)];
    let init_color = (0, 255, 0);
    let depth = 5;

    let triangle = Triangle {
        points: init_points,
        color: init_color,
    };
    triangle.sierpinski(&context, depth);

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
