#[macro_use]    // export macros from logger
pub mod logger;
pub mod gl_utils;       // make the gl utils public to the crate
pub mod global_state;   // make the global state public to the crate
pub mod app;            // make application callbacks public to the crate

// import platform contexts as modules
#[cfg(feature = "sdl2")]
mod sdl2_context;   

#[cfg(feature = "webgl")]
mod webgl_context;

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
            app::init(&gl_context, &mut globals);
        }

        // main loop is now handled via requestAnimationFrame
        webgl_context::wasm_main_loop(gl_context);
    }

    #[cfg(feature = "sdl2")]
    {
        let context = sdl2_context::create_sdl2_window_and_context(1024, 768);

        // initialise the app 
        if let Ok(mut globals) = global_state::GLOBALS.lock()  // get a mutable reference to the globals
        {
            app::init(&context.gl, &mut globals);
        }

        // run the main loop
        sdl2_context::run_sdl2_event_loop(context);
    }
    
}
