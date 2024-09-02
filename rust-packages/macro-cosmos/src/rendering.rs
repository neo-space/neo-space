use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement, WebGlProgram, WebGlBuffer};
use js_sys::Float32Array;

#[derive(Clone, Copy)]
struct Shape {
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    is_circle: bool,
}

#[wasm_bindgen]
pub struct Renderer {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    program: WebGlProgram,
    shapes: Vec<Shape>,
    vertex_buffer: WebGlBuffer,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Renderer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
        
        let context_options = js_sys::Object::new();
        js_sys::Reflect::set(&context_options, &"alpha".into(), &true.into())?;
        
        let context = canvas
            .get_context_with_context_options("webgl2", &context_options)?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
            in vec2 a_position;
            in vec2 a_size;
            in vec4 a_color;
            in float a_isCircle;
            uniform vec2 u_resolution;
            uniform float u_flipY;
            out vec4 v_color;
            out float v_isCircle;
            void main() {
                vec2 pixelSpace = a_position.xy;
                if (a_isCircle < 0.5) {
                    pixelSpace += a_size * gl_VertexID / 2.0;  // Adjust for square vertices
                }
                vec2 clipSpace = (pixelSpace / u_resolution) * 2.0 - 1.0;
                gl_Position = vec4(clipSpace * vec2(1, u_flipY), 0, 1);
                gl_PointSize = max(a_size.x, a_size.y);
                v_color = a_color;
                v_isCircle = a_isCircle;
            }
            "##,
        )?;

        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
            precision mediump float;
            in vec4 v_color;
            in float v_isCircle;
            out vec4 outColor;
            void main() {
                if (v_isCircle > 0.5) {
                    vec2 coord = gl_PointCoord - vec2(0.5);
                    if (length(coord) > 0.5) {
                        discard;
                    }
                }
                outColor = v_color;
            }
            "##,
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let vertex_buffer = context.create_buffer().ok_or("Failed to create buffer")?;

        Ok(Renderer { 
            context, 
            canvas,
            program,
            shapes: Vec::new(),
            vertex_buffer,
        })
    }

    pub fn resize_canvas(&mut self) {
        let window = web_sys::window().unwrap();
        self.canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        self.canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
        self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);
    }

    pub fn clear(&self) {
        self.context.clear_color(0.0, 0.0, 0.0, 0.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    #[wasm_bindgen]
    pub fn add_circle(&mut self, x: f32, y: f32, size: f32, r: f32, g: f32, b: f32, a: f32) {
        let shape = Shape {
            position: [x, y],
            size: [size, size],
            color: [r, g, b, a],
            is_circle: true,
        };
        self.shapes.push(shape);
    }

    #[wasm_bindgen]
    pub fn add_square(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
        let shape = Shape {
            position: [x, y], // Top-left corner of the square
            size: [width.abs(), height.abs()], // Use absolute values for size
            color: [r, g, b, a],
            is_circle: false,
        };
        self.shapes.push(shape);
    }

    pub fn draw_all_shapes(&mut self) {
        self.clear();
    
        let mut vertex_data = Vec::new();
        for shape in &self.shapes {
            let (x, y) = (shape.position[0], shape.position[1]);
            let (width, height) = (shape.size[0], shape.size[1]);
            
            // For squares, we need to create 4 vertices
            if !shape.is_circle {
                vertex_data.extend_from_slice(&[
                    x, y,
                    width, height,
                    shape.color[0], shape.color[1], shape.color[2], shape.color[3],
                    0.0,
                    x + width, y,
                    width, height,
                    shape.color[0], shape.color[1], shape.color[2], shape.color[3],
                    0.0,
                    x, y + height,
                    width, height,
                    shape.color[0], shape.color[1], shape.color[2], shape.color[3],
                    0.0,
                    x + width, y + height,
                    width, height,
                    shape.color[0], shape.color[1], shape.color[2], shape.color[3],
                    0.0,
                ]);
            } else {
                // For circles, we can keep the existing point-based approach
                vertex_data.extend_from_slice(&[
                    x, y,
                    width, height,
                    shape.color[0], shape.color[1], shape.color[2], shape.color[3],
                    1.0,
                ]);
            }
        }
    
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));
        unsafe {
            let vert_array = Float32Array::view(&vertex_data);
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    
        let position_attrib_location = self.context.get_attrib_location(&self.program, "a_position") as u32;
        let size_attrib_location = self.context.get_attrib_location(&self.program, "a_size") as u32;
        let color_attrib_location = self.context.get_attrib_location(&self.program, "a_color") as u32;
        let is_circle_attrib_location = self.context.get_attrib_location(&self.program, "a_isCircle") as u32;
    
        self.context.vertex_attrib_pointer_with_i32(position_attrib_location, 2, WebGl2RenderingContext::FLOAT, false, 36, 0);
        self.context.vertex_attrib_pointer_with_i32(size_attrib_location, 2, WebGl2RenderingContext::FLOAT, false, 36, 8);
        self.context.vertex_attrib_pointer_with_i32(color_attrib_location, 4, WebGl2RenderingContext::FLOAT, false, 36, 16);
        self.context.vertex_attrib_pointer_with_i32(is_circle_attrib_location, 1, WebGl2RenderingContext::FLOAT, false, 36, 32);
    
        self.context.enable_vertex_attrib_array(position_attrib_location);
        self.context.enable_vertex_attrib_array(size_attrib_location);
        self.context.enable_vertex_attrib_array(color_attrib_location);
        self.context.enable_vertex_attrib_array(is_circle_attrib_location);
    
        let resolution_location = self.context.get_uniform_location(&self.program, "u_resolution");
        self.context.uniform2f(
            resolution_location.as_ref(),
            self.canvas.width() as f32,
            self.canvas.height() as f32,
        );
    
        let flip_y_location = self.context.get_uniform_location(&self.program, "u_flipY");
        self.context.uniform1f(flip_y_location.as_ref(), -1.0);
    
        if !self.shapes.is_empty() {
            self.context.draw_arrays(
                WebGl2RenderingContext::TRIANGLE_STRIP,
                0,
                (vertex_data.len() / 9) as i32,
            );
        }
    }
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<web_sys::WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &web_sys::WebGlShader,
    frag_shader: &web_sys::WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}