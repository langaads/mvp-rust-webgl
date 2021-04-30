use super::super::common_funcs as cf;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertive_array_length: usize,
    rect_vertice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::color_2d::SHADER,
            super::super::shaders::fragment::color_2d::SHADER,
        )
        .unwrap();

        // 2 triangles counterclockwise makea rectangle
        let vertices_rect: [f32; 12] = [
            0., 1., // x, y top left
            0., 0., // x, y bottom left
            1., 1., // x, y top right
            1., 1., // x, y top right
            0., 0., // x, y bottom left
            1., 0., // x, y bottom right
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );
        let buffer_rect = gl.create_buffer().ok_or("Failet to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            rect_vertive_array_length: vertices_rect.len(),
            rect_vertice_buffer: buffer_rect,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_heigth: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(
            Some(&self.u_color),
            0.,  // r
            0.5, // g
            0.5, // b
            1.0, // a
        );

        gl.uniform1f(Some(&self.u_opacity), 1.);

        let translate_matrix = cf::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_heigth - 1.,
            0.,
        );

        let scale_matrix = cf::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_heigth,
            0.,
        );

        let transform_matrix = cf::mult_matrix_4(scale_matrix, translate_matrix);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_matrix);

        gl.draw_arrays(
            GL::TRIANGLES,
            0,
            (self.rect_vertive_array_length / 2) as i32,
        );
    }
}
