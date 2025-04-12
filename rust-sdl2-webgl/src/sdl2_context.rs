pub struct SDL2Context
{
    _sdl: sdl2::Sdl,
    _gl_context: sdl2::video::GLContext,
    event_loop: sdl2::EventPump,
    window: sdl2::video::Window,
    window_width: u32,
    window_height: u32,
    gl: glow::Context
}

// sdl 2 window + context creation
pub fn create_sdl2_window_and_context(window_width: u32, window_height: u32) -> SDL2Context
{
    let new_context: SDL2Context;
    unsafe {
        #[cfg(feature = "sdl2")]
        let sdl = sdl2::init().unwrap();        // init sdl
        let video = sdl.video().unwrap();   // init sdl video
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
        let window = video          // create opengl window
            .window("SDL 2 Window!", window_width, window_height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();   // get the gl context from the window
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);     // get the gl function pointers from the context
        new_context = SDL2Context { gl: gl, 
            window: window, 
            event_loop: sdl.event_pump().unwrap(), 
            _sdl: sdl, 
            _gl_context: gl_context,
            window_width: window_width,
            window_height: window_height
         };
    }
    new_context
}

// sdl 2 event pump
pub fn run_sdl2_event_loop(mut context: SDL2Context)
{
    let mut running = true;
    while running {
        {
            for event in context.event_loop.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => running = false,

                    // note that capturing enum values as '_' is used to ignore them
                    sdl2::event::Event::Window { timestamp: _, window_id: _, win_event } => {
                        if let sdl2::event::WindowEvent::Resized(w, h) = win_event      // detect window resize
                        {
                            context.window_width = w as u32;
                            context.window_height = h as u32;
                            super::on_canvas_size_changed(&context.gl, context.window_width, context.window_height);
                        }
                    }
                    _ => {}
                }
            }
        }

        super::tick();
        super::draw_gl(&context.gl, context.window_width, context.window_height);
        context.window.gl_swap_window();
        
        if !running {
            super::cleanup_gl_resources(&context.gl);
        }
    }
}