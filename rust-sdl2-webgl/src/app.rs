use glow::HasContext;  
use crate::gl_utils;

pub struct ApplicationState {
    pub bg_red: f32,
    pub simple_tri_shader: Option<gl_utils::gl_types::ShaderProgram>
}

// main init fn called once on start
pub fn init(gl : &glow::Context) -> ApplicationState
{
    let mut new_state = ApplicationState {
        bg_red: 0.0, 
        simple_tri_shader: None
    };

    let vertex_shader_src = r#"#version 300 es
        const vec2 verts[3] = vec2[3](
            vec2(0.5f, 1.0f),
            vec2(0.0f, 1.0f),
            vec2(0.0f, 0.0f)
        );
        out vec2 vert;
        void main() {
            vert = verts[gl_VertexID];
            gl_Position = vec4(vert - 0.5, 0.0, 1.0);
        }
        "#;
    
    let fragment_shader_src = r#"#version 300 es
        precision mediump float;
        in vec2 vert;
        out vec4 color;
        void main() {
            color = vec4(vert, 0.5, 1.0);
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
pub fn tick(state: &mut ApplicationState)
{
    state.bg_red = state.bg_red + 0.001;
    if state.bg_red > 1.0
    {
        state.bg_red = 0.0;
    }
}

// main update/drawing entry point
pub fn draw_gl(gl : &glow::Context, state: &ApplicationState,_viewport_width: u32, _viewport_height: u32)
{
    unsafe {
        gl.clear_color(state.bg_red, 0.0, 0.0, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        gl.use_program(state.simple_tri_shader);
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