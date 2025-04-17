use glow::HasContext;
use nalgebra::{Point3,Point4};
use crate::gl_utils;

#[allow(dead_code)]     // Stop compiler warning that we never read these
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
    vertex_buffer: Option<gl_utils::gl_types::Buffer>,
    current_vertices: Vec<ImmediateRenderVertex>,
    current_triangle_draws: Vec<ImmediateRenderDrawcall>,
    current_line_draws: Vec<ImmediateRenderDrawcall>
}

impl ImmediateRender {
    pub fn cleanup(&mut self, gl : &glow::Context)
    {
        unsafe{
            gl.delete_buffer(self.vertex_buffer.unwrap());
            gl.delete_vertex_array(self.vertex_array.unwrap());
            gl_utils::unload_shader_program(gl, &self.shader_program.unwrap());
        }
        self.vertex_buffer = None;
        self.vertex_array = None;
        self.shader_program = None;
    }

    pub fn new(gl : &glow::Context, max_vertex_count: u32) -> Self {
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
            precision highp float;
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
        let vertex_buffer: Option<gl_utils::gl_types::Buffer>;
        let vertex_size = size_of::<ImmediateRenderVertex>();
        unsafe {
            vertex_buffer = match gl.create_buffer() {
                Ok(buffer) => Some(buffer),
                Err(text) => {
                    console_log!("Failed to create vertex buffer - {text}");
                    None
                }
            };
            let vertex_data_size_bytes = vertex_size * max_vertex_count as usize;
            gl.bind_buffer(glow::ARRAY_BUFFER, vertex_buffer);
            gl.buffer_data_size(glow::ARRAY_BUFFER, vertex_data_size_bytes as i32, glow::DYNAMIC_DRAW);

            vertex_array = match gl.create_vertex_array() {
                Ok(vertex_array) => Some(vertex_array),
                Err(text) => {
                    console_log!("Failed to create vertex array - {text}");
                    None
                }
            };

            let position_attrib_location = gl.get_attrib_location(shader_program.unwrap(), "vs_in_position");
            let colour_attrib_location = gl.get_attrib_location(shader_program.unwrap(), "vs_in_colour");
            gl.bind_vertex_array(vertex_array);
            gl.enable_vertex_attrib_array(position_attrib_location.unwrap());
            gl.vertex_attrib_pointer_f32(position_attrib_location.unwrap(), 4, glow::FLOAT, false, vertex_size as i32, 0);
            let colour_data_offset = size_of::<Point4<f32>>() as i32;
            gl.enable_vertex_attrib_array(colour_attrib_location.unwrap());
            gl.vertex_attrib_pointer_f32(colour_attrib_location.unwrap(), 4, glow::FLOAT, false, vertex_size as i32, colour_data_offset );

            // reset bound vao/buffer
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }
        Self {
            vertex_array: vertex_array,
            vertex_buffer: vertex_buffer,
            shader_program: shader_program,
            current_vertices: Vec::new(),
            current_triangle_draws: Vec::new(),
            current_line_draws: Vec::new()
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

    pub fn add_line(&mut self, v0: &Point3<f32>, c0: &Point4<f32>, v1: &Point3<f32>, c1: &Point4<f32>) {
        let v0 = ImmediateRenderVertex {
            position: Point4::new(v0.x, v0.y, v0.z, 1.0),
            colour: *c0
        };
        let v1 = ImmediateRenderVertex {
            position: Point4::new(v1.x, v1.y, v1.z, 1.0),
            colour: *c1
        };
        let draw = ImmediateRenderDrawcall {
            start_vertex: self.current_vertices.len() as i32,
            vertex_count: 2
        };
        self.current_vertices.push(v0);
        self.current_vertices.push(v1);
        self.current_line_draws.push(draw);
    }

    pub fn clear(&mut self) {
        self.current_vertices.clear();
        self.current_triangle_draws.clear();
        self.current_line_draws.clear();
    }

    fn draw_compacted(gl: &glow::Context, primitive_type: u32, draws: &Vec<ImmediateRenderDrawcall>)
    {
        let mut last_vertex_index = -1;
        let mut current_vertex_count = 0;
        for draw in draws
        {
            if draw.start_vertex == (last_vertex_index + current_vertex_count)
            {
                current_vertex_count += draw.vertex_count;
            }
            else 
            {
                if current_vertex_count > 0
                {
                    unsafe {
                        gl.draw_arrays(primitive_type, last_vertex_index, current_vertex_count);
                    }
                }
                last_vertex_index = draw.start_vertex;
                current_vertex_count = draw.vertex_count;
            }
        }
        if last_vertex_index != -1 && current_vertex_count > 0
        {
            unsafe {
                gl.draw_arrays(primitive_type, last_vertex_index, current_vertex_count);
            }
        }
    }

    pub fn draw(&self, gl : &glow::Context, camera: &crate::render::camera::Camera) {
        unsafe {
             // copy vertex data to buffer
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vertex_buffer);
            gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, self.current_vertices.align_to::<u8>().1);

            gl.use_program(self.shader_program);
            let view_proj_uniform_pos = gl.get_uniform_location(self.shader_program.unwrap(), "view_projection_matrix");
            gl.uniform_matrix_4_f32_slice(view_proj_uniform_pos.as_ref(), false, camera.get_view_projection_matrix().as_slice());
            gl.bind_vertex_array(self.vertex_array);
        }

        Self::draw_compacted(gl, glow::TRIANGLES, &self.current_triangle_draws);
        Self::draw_compacted(gl, glow::LINES, &self.current_line_draws);
    }
}

