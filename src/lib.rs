use js_sys::{WebAssembly, Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement,
    WebGlProgram,
    WebGlRenderingContext,
    WebGlShader,
};

#[wasm_bindgen]
pub struct Scene {
    ctx: WebGlRenderingContext,
}

#[wasm_bindgen]
impl Scene {
    pub fn new() -> Result<Scene, JsValue> {
        let document = web_sys::window()
            .ok_or_else(|| Error::new("Can't get window"))?
            .document()
            .ok_or_else(|| Error::new("Can't get document"))?;
        let canvas = document
            .get_element_by_id("canvas")
            .ok_or_else(|| Error::new("Can't get canvas element"))?;
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| Error::new("Can't cast element to HtmlCanvasElement"))?;

        let ctx = canvas
            .get_context("webgl")?
            .ok_or_else(|| Error::new("Can't get webgl context"))?
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| Error::new("Can't cast context to WebGlRenderingContext"))?;

        Ok(Scene { ctx })
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let vertex_shader = compile_shader(
            &self.ctx,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 position;
            void main() {
                gl_Position = position;
            }
            "#,
        )?;
        let fragment_shader = compile_shader(
            &self.ctx,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            void main() {
                gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
            "#,
        )?;

        #[rustfmt::skip]
        let program = link_program(
            &self.ctx,
            vertex_shader,
            fragment_shader,
        )?;
        self.ctx.use_program(Some(&program));

        #[rustfmt::skip]
        let vertices: [f32; 9] = [
            -0.7, -0.7, 0.0,
            0.7, -0.7, 0.0,
            0.0, 0.7, 0.0,
        ];
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

        let buffer = self.ctx.create_buffer().ok_or("failed to create buffer")?;
        self.ctx
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        self.ctx.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        self.ctx
            .vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.ctx.enable_vertex_attrib_array(0);

        self.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        self.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.ctx.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );
        Ok(())
    }
}

fn compile_shader(
    ctx: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    if ctx
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(ctx
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

fn link_program(
    ctx: &WebGlRenderingContext,
    vertex_shader: WebGlShader,
    fragment_shader: WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = ctx
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    ctx.attach_shader(&program, &vertex_shader);
    ctx.attach_shader(&program, &fragment_shader);
    ctx.link_program(&program);

    if ctx
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(ctx
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}
