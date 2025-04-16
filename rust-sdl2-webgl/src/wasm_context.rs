use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// since input state is captured among multiple JS callbacks, we need mutable global state
// use LazyLock to achieve this since it allows non-const initialisers

use std::sync::{LazyLock, Mutex};
static INPUT_STATE: LazyLock<Mutex<crate::input::InputState>> = LazyLock::new( || Mutex::new(crate::input::InputState::default()));

pub struct WasmContext
{
    pub gl : glow::Context,
    last_tick_time: f64
}

// Get the main browser window or panic
fn window() -> web_sys::Window 
{
    web_sys::window().expect("no global `window` exists")
}

// Request next animation frame with a closure to call, or panic
fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) 
{
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn register_input_events(canvas: &web_sys::HtmlCanvasElement)
{
    // register input events from canvas
    let on_mouse_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
        let mut mutable_input = INPUT_STATE.lock().unwrap();
        mutable_input.mouse_state.position_x = event.client_x();
        mutable_input.mouse_state.position_y = event.client_y();
    });
    canvas.set_onmousemove(Some(on_mouse_move.as_ref().unchecked_ref()));
    on_mouse_move.forget();

    let on_mouse_down = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
        let mut mutable_input = INPUT_STATE.lock().unwrap();
        match event.button() {
            0 => mutable_input.mouse_state.left_btn_down = true,
            1 => mutable_input.mouse_state.middle_btn_down = true,
            2 => mutable_input.mouse_state.right_btn_down = true,
            _ => ()
        }
    });
    canvas.set_onmousedown(Some(on_mouse_down.as_ref().unchecked_ref()));
    on_mouse_down.forget();

    let on_mouse_up = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
        let mut mutable_input = INPUT_STATE.lock().unwrap();
        match event.button() {
            0 => mutable_input.mouse_state.left_btn_down = false,
            1 => mutable_input.mouse_state.middle_btn_down = false,
            2 => mutable_input.mouse_state.right_btn_down = false,
            _ => ()
        }
    });
    canvas.set_onmouseup(Some(on_mouse_up.as_ref().unchecked_ref()));
    on_mouse_up.forget();

    // disable context menu on right-click
    let on_context_menu = Closure::<dyn FnMut() -> bool>::new(move || {
       return false;
    });
    canvas.set_oncontextmenu(Some(on_context_menu.as_ref().unchecked_ref()));
    on_context_menu.forget();
}

// Get the gl context from the canvas
pub fn create_context() -> WasmContext
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

    register_input_events(&canvas);

    WasmContext { gl: gl, last_tick_time: 0.0 }
}

// main loop implemented via websys request_animation_frame
pub fn wasm_main_loop(mut wasm_context : WasmContext, mut app_state: crate::app::ApplicationState) 
{
    let f = Rc::new(RefCell::new(None));    // so the lambda can register a copy of itself with register animation frame
    let g = f.clone();
    
    // get a lambda that can be called from JS
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        use wasm_bindgen::JsCast;
        let canvas = window()
           .document()
           .unwrap()
           .get_element_by_id("canvas")
           .unwrap()
           .dyn_into::<web_sys::HtmlCanvasElement>()
           .unwrap();

        // ensure the canvas size always fits the entire page
        let document_element = window().document().unwrap().document_element().unwrap();
        canvas.set_width(document_element.client_width() as u32);
        canvas.set_height(document_element.client_height() as u32);

        let tick_delta_ms = timestamp - wasm_context.last_tick_time;
        wasm_context.last_tick_time = timestamp;

        let title_text = format!("Update time: {:.2}ms", tick_delta_ms);
        window()
           .document()
           .unwrap()
           .set_title(&title_text);

        crate::app::tick(&mut app_state, &INPUT_STATE.lock().unwrap(), tick_delta_ms / 1000.0); 
        crate::app::draw_gl(&wasm_context.gl, &mut app_state, canvas.width(), canvas.height());    // call the shared render fn

        request_animation_frame(f.borrow().as_ref().unwrap());  // register next frame
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}