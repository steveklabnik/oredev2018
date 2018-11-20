use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn greet() {
    let window = window().expect("couldn't find the window");

    window.alert_with_message("hello wasm").expect("couldn't alert");
}
