use glow::HasContext;
use nalgebra::{Point3,Point4};
use crate::gl_utils;

struct ImmediateRenderVertex
{
    position: Point4<f32>,
    colour: Point4<f32>
}

struct ImmediateRenderDrawcall
{
    start_vertex: i32,
    vertex_count: i32
}

// immediate render owns a vertex buffer + vertex array
// vertex buffer is updated each frame with new geometry
pub struct ImmediateRender
{
    shader_program: Option<gl_utils::gl_types::ShaderProgram>,
    vertex_array: Option<gl_utils::gl_types::VertexArray>,
    current_vertices: Vec<ImmediateRenderVertex>,
    current_triangle_draws: Vec<ImmediateRenderDrawcall>
}

impl ImmediateRender {
    pub fn new(gl : &glow::Context) -> Self {
        let vertex_shader_src = r#"#version 300 es
            uniform mat4 view_projection_matrix;
            layout (location = 0) in vec4 vs_in_position; 
            layout (location = 1) in vec4 vs_in_colour;
            out vec4 vs_out_colour;
            void main() {
                gl_Position = view_projection_matrix * vs_in_position;
                vs_out_colour = vs_in_colour;
            }
        "#;
        let fragment_shader_src = r#"#version 300 es
            precision mediump float;
            in vec4 vs_out_colour;
            out vec4 fs_out_colour;
            void main() {
                fs_out_colour = vs_out_colour;
            }
        "#;
        let shader_program = match gl_utils::load_shader_program(gl, vertex_shader_src, fragment_shader_src) {
            Ok(shader_program) => Some(shader_program),
            Err(text) => {
                console_log!("Failed to load shaders - {text}");
                None
            }
        };
        let vertex_array: Option<gl_utils::gl_types::VertexArray>;
        unsafe {
            vertex_array = match gl.create_vertex_array() {
                Ok(vertex_array) => Some(vertex_array),
                Err(text) => {
                    console_log!("Failed to create vertex array - {text}");
                    None
                }
            }
        }
        Self {
            vertex_array: vertex_array,
            shader_program: shader_program,
            current_vertices: Vec::new(),
            current_triangle_draws: Vec::new()
        }
    }

    pub fn add_triangle(&mut self, v0: &Point3<f32>, c0: &Point4<f32>, v1: &Point3<f32>, c1: &Point4<f32>, v2: &Point3<f32>, c2: &Point4<f32>) {
        let v0 = ImmediateRenderVertex {
            position: Point4::new(v0.x, v0.y, v0.z, 1.0),
            colour: *c0
        };
        let v1 = ImmediateRenderVertex {
            position: Point4::new(v1.x, v1.y, v1.z, 1.0),
            colour: *c1
        };
        let v2 = ImmediateRenderVertex {
            position: Point4::new(v2.x, v2.y, v2.z, 1.0),
            colour: *c2
        };
        let draw = ImmediateRenderDrawcall {
            start_vertex: self.current_vertices.len() as i32,
            vertex_count: 3
        };
        self.current_vertices.push(v0);
        self.current_vertices.push(v1);
        self.current_vertices.push(v2);
        self.current_triangle_draws.push(draw);
    }

    pub fn clear(&mut self) {
        self.current_vertices.clear();
        self.current_triangle_draws.clear();
    }

    pub fn draw(&self, gl : &glow::Context, camera: &crate::render::camera::Camera) {
        unsafe {
            gl.use_program(self.shader_program);
            let view_proj_uniform_pos = gl.get_uniform_location(self.shader_program.unwrap(), "view_projection_matrix");
            gl.uniform_matrix_4_f32_slice(view_proj_uniform_pos.as_ref(), false, camera.get_view_projection_matrix().as_slice());
            
            gl.bind_vertex_array(self.vertex_array);
            for draw in &self.current_triangle_draws
            {
                gl.draw_arrays(glow::TRIANGLES, draw.start_vertex, draw.vertex_count);
            }
        }
    }
}

