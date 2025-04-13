use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// Get the main browser window or panic
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

// Request next animation frame with a closure to call, or panic
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Get the gl context from the canvas
pub fn get_gl_context() -> glow::Context
{
    use wasm_bindgen::JsCast;
    let canvas = window()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);
    gl
}

// main loop implemented via websys request_animation_frame
pub fn wasm_main_loop(gl_context : glow::Context) 
{
    let f = Rc::new(RefCell::new(None));    // so the lambda can register a copy of itself with register animation frame
    let g = f.clone();
    
    // get a lambda that can be called from JS
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        use wasm_bindgen::JsCast;
        let canvas = window()
           .document()
           .unwrap()
           .get_element_by_id("canvas")
           .unwrap()
           .dyn_into::<web_sys::HtmlCanvasElement>()
           .unwrap();

        if let Ok(mut globals) = super::global_state::GLOBALS.lock()  // get a mutable reference to the globals
        {
            crate::app::tick(&mut globals);  // always tick before drawing
            crate::app::draw_gl(&gl_context, &mut globals, canvas.width(), canvas.height());    // call the shared render fn
        }

        request_animation_frame(f.borrow().as_ref().unwrap());  // register next frame
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}