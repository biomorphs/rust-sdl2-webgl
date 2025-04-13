#[macro_use]    // export macros from logger
pub mod logger;

// import platform contexts as modules
#[cfg(feature = "sdl2")]
mod sdl2_context;   

#[cfg(feature = "webgl")]
mod webgl_context;

pub mod gl_utils;   // use our gl utils module + make them public to the crate
pub mod global_state;   // access our global state
use glow::HasContext;   // all implementations use glow gl context

// main init fn called once on start
fn init(gl : &glow::Context, state: &mut global_state::GlobalState)
{
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

    state.simple_tri_shader = match gl_utils::load_shader_program(gl, vertex_shader_src, fragment_shader_src) {
        Ok(shader_program) => Some(shader_program),
        Err(text) => {
            console_log!("Failed to load shaders - {text}");
            None
        }
    };

    unsafe {
        state.vertex_array = match gl.create_vertex_array() {
            Ok(array) => Some(array),
            Err(text) => {
                console_log!("Failed to create vertex array - {text}");
                None
            }
        }
    }
}

// main tick/update entry point
fn tick(state: &mut global_state::GlobalState)
{
    state.bg_red = state.bg_red + 0.001;
    if state.bg_red > 1.0
    {
        state.bg_red = 0.0;
    }
}

// main update/drawing entry point
fn draw_gl(gl : &glow::Context, state: &global_state::GlobalState,_viewport_width: u32, _viewport_height: u32)
{
    unsafe {
        gl.clear_color(state.bg_red, 0.0, 0.0, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        gl.use_program(state.simple_tri_shader);
        gl.bind_vertex_array(state.vertex_array);
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
fn cleanup_gl_resources(gl : &glow::Context, state: &mut global_state::GlobalState)
{
    // cleanup gl stuff
    gl_utils::unload_shader_program(gl, &state.simple_tri_shader.unwrap());
    state.simple_tri_shader = None;
    unsafe{
        gl.delete_vertex_array(state.vertex_array.unwrap());
    }
    state.vertex_array = None;
}

// window resize callback for desktop app
#[cfg(feature = "sdl2")]
fn on_canvas_size_changed(_new_width: u32, _new_height: u32)
{
    // handle window resize
}

// main entry point
fn main() {
    #[cfg(feature = "webgl")]
    {
        // set up console panic hook for wasm so we see panic messages in the browser logs
        console_error_panic_hook::set_once();

        // get the gl context from the canvas once
        // Awkward, but glow will not return the same context for a canvas over multiple calls
        // See https://github.com/grovesNL/glow/issues/285
        let gl_context = webgl_context::get_gl_context();

        // initialise the app
        if let Ok(mut globals) = global_state::GLOBALS.lock()  // get a mutable reference to the globals
        {
            globals.gl = Some(gl_context);    // cache the context
            init(&mut globals);
        }
    }

    #[cfg(feature = "sdl2")]
    {
        let context = sdl2_context::create_sdl2_window_and_context(1024, 768);

        // initialise the app 
        if let Ok(mut globals) = global_state::GLOBALS.lock()  // get a mutable reference to the globals
        {
            init(&context.gl, &mut globals);
        }

        // run the main loop
        sdl2_context::run_sdl2_event_loop(context);
    }
    
}
