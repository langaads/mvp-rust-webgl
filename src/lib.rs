extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);

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
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl LangasGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            program_color_2d: programs::Color2D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let state = app_state::get_current_state();
        self.program_color_2d.render(
            &self.gl,
            state.control_bottom,
            state.control_top,
            state.control_left,
            state.control_right,
            state.canvas_height,
            state.canvas_width,
        )
    }
}
