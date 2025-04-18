#[macro_use]    // export macros from logger
pub mod logger;
pub mod gl_utils;       // make the gl utils public to the crate
pub mod app;            // make application callbacks public to the crate
pub mod render;         // make render stuff public
pub mod input;
pub mod top_down_camera;

// import platform contexts as modules
#[cfg(feature = "sdl2")]
mod sdl2_context;   

#[cfg(feature = "webgl")]
mod wasm_context;

// main entry point
fn main() {
    #[cfg(feature = "webgl")]
    {
        // set up console panic hook for wasm so we see panic messages in the browser logs
        console_error_panic_hook::set_once();

        // get the gl context from the canvas
        let context = wasm_context::create_context();

        // initialise the app
        let app_state = app::init(&context.gl);

        // main loop is now handled via requestAnimationFrame
        wasm_context::wasm_main_loop(context, app_state);
    }

    #[cfg(feature = "sdl2")]
    {
        let context = sdl2_context::create_sdl2_window_and_context(1024, 768);

        // initialise the app 
        let app_state = app::init(&context.gl);

        // run the main loop
        sdl2_context::run_sdl2_event_loop(context, app_state);
    }
    
}
