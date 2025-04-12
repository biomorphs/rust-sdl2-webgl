use wasm_bindgen::prelude::wasm_bindgen;

// for webgl builds, export function hooks for drawing, window resize, etc
// call them from dom events/requestAnimation
#[wasm_bindgen]
pub fn draw_webgl() {
    super::tick();  // always tick before drawing

    // get the glow context from a WebGL2 context on wasm32 targets
    use wasm_bindgen::JsCast;
    let canvas = web_sys::window()
        .unwrap()
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
    super::draw_gl(&gl, canvas.width(), canvas.height());    // call the shared render fn
}