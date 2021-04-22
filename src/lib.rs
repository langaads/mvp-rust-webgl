extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(greeting: &str) {
    // alert(&format!("{} World 2, FROM RUST!", greeting));
    log(&format!("{} World 2, FROM RUST!", greeting));
}

#[wasm_bindgen]
pub struct LangasGame {

}

#[wasm_bindgen]
impl LangasGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
 
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {

        Ok(())
    }

    pub fn render(&self) {
        log("rendered")
    }
}