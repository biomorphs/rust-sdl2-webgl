use glow::HasContext;   // all implementations use glow gl context

// import platform contexts as modules

#[cfg(feature = "sdl2")]
mod sdl2_context;   

#[cfg(feature = "webgl")]
mod webgl_context;

// main drawing entry point
fn draw_gl(gl : &glow::Context)
{
    static mut BG_RED: f32 = 0.5; 
    unsafe {
        BG_RED = BG_RED + 0.001;
        if BG_RED > 1.0
        {
            BG_RED = 0.0;
        }

        gl.clear_color(BG_RED, 0.0, 0.0, 1.0);
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

// window resize function for desktop app
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
