use glow::HasContext;  
use crate::gl_utils;
use crate::render::camera::Camera;
use crate::render::immediate_render::ImmediateRender;
use nalgebra::{Point3, Point4, Vector3};

pub struct ApplicationState {
    pub im_render: ImmediateRender,
    pub bg: f32,
    pub time_elapsed: f64,
    pub simple_tri_shader: Option<gl_utils::gl_types::ShaderProgram>
}

// main init fn called once on start
pub fn init(gl : &glow::Context) -> ApplicationState
{
    let mut new_state = ApplicationState {
        im_render: ImmediateRender::new(gl),
        bg: 0.0, 
        time_elapsed: 0.0,
        simple_tri_shader: None
    };

    let vertex_shader_src = r#"#version 300 es
        const vec2 verts[3] = vec2[3](
            vec2(0.0f, 1.0f),
            vec2(1.0f, -1.0f),
            vec2(-1.0f, -1.0f)
        );
        out vec2 vert;
        uniform mat4 view_projection_matrix;
        void main() {
            vert = verts[gl_VertexID];
            gl_Position = view_projection_matrix * vec4(vert, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"#version 300 es
        precision mediump float;
        in vec2 vert;
        out vec4 color;
        uniform vec3 colourMultiplier;
        void main() {
            color = vec4(colourMultiplier * vec3(vert, 0.5), 1.0);
        }
    "#;
    new_state.simple_tri_shader = match gl_utils::load_shader_program(gl, vertex_shader_src, fragment_shader_src) {
        Ok(shader_program) => Some(shader_program),
        Err(text) => {
            console_log!("Failed to load shaders - {text}");
            None
        }
    };
    new_state
}

// main tick/update entry point
pub fn tick(state: &mut ApplicationState, delta_time: f64)
{
    state.im_render.clear();
    state.im_render.add_triangle(
        &Point3::new(0.0, 1.0, 0.0), &Point4::new(1.0, 0.0, 0.0, 1.0),
        &Point3::new(1.0, -1.0, 0.0), &Point4::new(1.0, 0.0, 0.0, 1.0),
        &Point3::new(-1.0, -1.0, 0.0), &Point4::new(1.0, 0.0, 0.0, 1.0)
    );

    state.time_elapsed += delta_time;
    state.bg = 0.5 + (state.time_elapsed.sin() as f32) * 0.5;
}

// main update/drawing entry point
pub fn draw_gl(gl : &glow::Context, state: &ApplicationState,viewport_width: u32, viewport_height: u32)
{
    let aspect: f32 = viewport_width as f32 / viewport_height as f32;
    let mut render_camera = Camera::make_projection(0.1, 100.0, aspect, 90.0 );
    render_camera.look_at(Point3::new(2.0,1.0,5.0), Point3::new(0.0,0.0,0.0), Vector3::y());

    unsafe {
        gl.clear_color(state.bg, state.bg, state.bg, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        state.im_render.draw(gl, &render_camera);

        gl.use_program(state.simple_tri_shader);
        let view_proj_uniform_pos = gl.get_uniform_location(state.simple_tri_shader.unwrap(), "view_projection_matrix");
        gl.uniform_matrix_4_f32_slice(view_proj_uniform_pos.as_ref(), false, render_camera.get_view_projection_matrix().as_slice());
        let colour_mul_uniform_pos = gl.get_uniform_location(state.simple_tri_shader.unwrap(), "colourMultiplier");
        gl.uniform_3_f32(colour_mul_uniform_pos.as_ref(), 1.0 - state.bg, state.bg, 1.0 - state.bg);
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
pub fn cleanup_gl_resources(gl : &glow::Context, state: &mut ApplicationState)
{
    // cleanup gl stuff
    gl_utils::unload_shader_program(gl, &state.simple_tri_shader.unwrap());
    state.simple_tri_shader = None;
}