use glow::{HasContext, COLOR_BUFFER_BIT};

// SDL 2 context stuff
#[cfg(feature = "sdl2")]
struct SDL2Context
{
    _sdl: sdl2::Sdl,
    event_loop: sdl2::EventPump,
    window: sdl2::video::Window,
    gl: glow::Context
}

// sdl 2 window + context creation
#[cfg(feature = "sdl2")]
fn create_sdl2_window_and_context() -> SDL2Context
{
    let new_context: SDL2Context;
    unsafe {
        #[cfg(feature = "sdl2")]
        let (_shader_version, _context) = {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
            let window = video
                .window("SDL 2 Window!", 512, 512)
                .opengl()
                .resizable()
                .build()
                .unwrap();
            let gl_context = window.gl_create_context().unwrap();
            let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
            new_context = SDL2Context { gl: gl, window: window, event_loop: sdl.event_pump().unwrap(), _sdl: sdl};
            ("#version 130", gl_context)
        };
    }
    new_context
}

// sdl 2 event pump
#[cfg(feature = "sdl2")]
fn run_sdl2_event_loop(mut context: SDL2Context)
{
    let mut running = true;
    while running {
        {
            for event in context.event_loop.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => running = false,
                    _ => {}
                }
            }
        }

        draw_gl(&context.gl);
        context.window.gl_swap_window();
        
        if !running {
            cleanup_gl_resources(&context.gl);
        }
    }
}

// main drawing entry point
fn draw_gl(gl : &glow::Context)
{
    unsafe {
        gl.clear_color(1.0, 0.0, 0.0, 1.0);
        gl.clear(COLOR_BUFFER_BIT);
    }
}

fn cleanup_gl_resources(_gl : &glow::Context)
{
    //unsafe {
        // cleanup gl stuff
    //}
}


fn main() {
    println!("Hello, world!");

    // Set up the gl context for SDL 2 and init the event loop
    #[cfg(feature = "sdl2")]
    let context= create_sdl2_window_and_context();

    #[cfg(feature = "sdl2")]
    run_sdl2_event_loop(context);
}
