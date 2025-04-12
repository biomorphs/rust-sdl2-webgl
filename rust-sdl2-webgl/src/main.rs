pub mod gl_utils;   // use our gl utils module + make them public to the crate
pub mod global_state;   // access our global state
use glow::HasContext;   // all implementations use glow gl context

// main init fn called once on start
fn init(gl : &glow::Context, state: &mut global_state::GlobalState)
{
    println!("Init now");
    let vertex_shader_src = r#"
        #version 300 es
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
    
    let fragment_shader_src = r#"
        #version 300 es
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
            println!("Failed to load shaders - {text}");
            None
        }
    }
}

// main tick/update entry point
fn tick(gl : &glow::Context)
{
    if let Ok(mut globals) = global_state::GLOBALS.lock()  // use this syntax to get a mutable reference to the globals
    {
        if !globals.initialised
        {
            init(gl, &mut globals);
            globals.initialised = true;
        }

        globals.bg_red = globals.bg_red + 0.001;
        if globals.bg_red > 1.0
        {
            globals.bg_red = 0.0;
        }
    }
}

// main update/drawing entry point
fn draw_gl(gl : &glow::Context, _viewport_width: u32, _viewport_height: u32)
{
    if let Ok(globals) = global_state::GLOBALS.lock()
    {
        let clear_red: f32 = globals.bg_red;  // read bg_red from globals
        unsafe {
            gl.clear_color(clear_red, 0.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.use_program(globals.simple_tri_shader);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
fn cleanup_gl_resources(gl : &glow::Context)
{
    if let Ok(mut globals) = global_state::GLOBALS.lock()  // use this syntax to get a mutable reference to the globals
    {
        // cleanup gl stuff
        gl_utils::unload_shader_program(gl, &globals.simple_tri_shader.unwrap());
        globals.simple_tri_shader = None;
    }
}

// window resize callback for desktop app
#[cfg(feature = "sdl2")]
fn on_canvas_size_changed(_gl : &glow::Context, _new_width: u32, _new_height: u32)
{
    // do gl stuff to handle resizes
}

// import platform contexts as modules
#[cfg(feature = "sdl2")]
mod sdl2_context;   

#[cfg(feature = "webgl")]
mod webgl_context;

// main entry point, does nothing in webgl build
fn main() {
    // Set up the gl context for SDL 2 and init the event loop
    #[cfg(feature = "sdl2")]
    {
        let context= sdl2_context::create_sdl2_window_and_context(1024, 768);
        sdl2_context::run_sdl2_event_loop(context);
    }
}
