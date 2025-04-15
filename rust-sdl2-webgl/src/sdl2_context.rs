pub struct SDL2Context
{
    sdl: sdl2::Sdl,
    _gl_context: sdl2::video::GLContext,
    event_loop: sdl2::EventPump,
    window: sdl2::video::Window,
    window_width: u32,
    window_height: u32,
    pub gl: glow::Context
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
        gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);  // we want a gles compatible context
        gl_attr.set_context_version(3, 0);              // v3+ please
        let window = video          // create opengl window
            .window("SDL 2 Window!", window_width, window_height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();   // get the gl context from the window
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);     // get the gl function pointers from the context
        new_context = SDL2Context {
            gl: gl,
            window: window, 
            event_loop: sdl.event_pump().unwrap(), 
            sdl: sdl, 
            _gl_context: gl_context,
            window_width: window_width,
            window_height: window_height
         };
         new_context
    }
}

// sdl 2 event pump
pub fn run_sdl2_event_loop(mut context: SDL2Context, mut app_state: crate::app::ApplicationState)
{
    let mut running = true;
    let sdl_timer = context.sdl.timer().unwrap();
    let perf_timer_frequency = sdl_timer.performance_frequency();
    let mut perf_timer_last_count = sdl_timer.performance_counter();
    while running {
        {
            for event in context.event_loop.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => running = false,
                    sdl2::event::Event::Window { timestamp: _, window_id: _, win_event } => {
                        if let sdl2::event::WindowEvent::Resized(w, h) = win_event      // detect window resize
                        {
                            context.window_width = w as u32;
                            context.window_height = h as u32;
                        }
                    }
                    _ => {}
                }
            }
        }

        let perf_timer_this_count = sdl_timer.performance_counter();
        let tick_delta : f64 = (perf_timer_this_count - perf_timer_last_count) as f64 / perf_timer_frequency as f64;
        perf_timer_last_count = perf_timer_this_count;

        crate::app::tick(&mut app_state, tick_delta);
        crate::app::draw_gl(&context.gl, &app_state, context.window_width, context.window_height);
        
        context.window.gl_swap_window();
        
        if !running {
            crate::app::cleanup_gl_resources(&context.gl, &mut app_state);
        }
    }
}