// webgl and native gl backends use different types for shaders, vertex arrays, textures, etc
// we define our own types and use them instead

#[cfg(feature = "sdl2")]
mod gl_types_native;

#[cfg(feature = "sdl2")]
pub use gl_types_native::*;

#[cfg(feature = "webgl")]
mod gl_types_wasm;

#[cfg(feature = "webgl")]
pub use gl_types_wasm::*;