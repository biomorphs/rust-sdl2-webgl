use glow::HasContext;   // all implementations use glow gl context
mod gl_utils;   // use our gl utils module

// import platform contexts as modules
#[cfg(feature = "sdl2")]
mod sdl2_context;   

#[cfg(feature = "webgl")]
mod webgl_context;

// we need to store global state somehow for wasm (to avoid passing from rust -> js and back)
// we can use a global static protected by a mutex to achieve this
// see https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use std::sync::Mutex;
struct GlobalState {
    bg_red: f32
}
static GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(
    GlobalState { bg_red: 0.0 }
);

// main update/drawing entry point
fn draw_gl(gl : &glow::Context, _viewport_width: u32, _viewport_height: u32)
{
    let mut clear_red: f32 = 0.0;
    if let Ok(mut value) = GLOBAL_STATE.lock()  // use this syntax to get a mutable reference to the globals
    {
        value.bg_red = value.bg_red + 0.001;
        if value.bg_red > 1.0
        {
            value.bg_red = 0.0;
        }
        clear_red = value.bg_red;
    }
    
    unsafe {
        gl.clear_color(clear_red, 0.0, 0.0, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
fn cleanup_gl_resources(_gl : &glow::Context)
{
    //unsafe {
        // cleanup gl stuff
    //}
}

// window resize callback for desktop app
#[cfg(feature = "sdl2")]
fn on_canvas_size_changed(_gl : &glow::Context, new_width: u32, new_height: u32)
{
    // do gl stuff to handle resizes
    println!("Resize event! {}, {}", new_width, new_height);
}

// main entry point, does nothing in webgl build
fn main() {
    // Set up the gl context for SDL 2 and init the event loop
    #[cfg(feature = "sdl2")]
    {
        let context= sdl2_context::create_sdl2_window_and_context(1024, 768);
        sdl2_context::run_sdl2_event_loop(context);
    }
}
