// logging macro for desktop/wasm

#[macro_use]    // import macros from the next module
#[cfg(feature = "sdl2")]
mod logger_native;

#[macro_use]    // import macros from the next module
#[cfg(feature = "webgl")]
mod logger_wasm;