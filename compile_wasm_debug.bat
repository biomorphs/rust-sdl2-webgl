cd rust-sdl2-webgl
cargo build --target wasm32-unknown-unknown --no-default-features --features "webgl"
wasm-bindgen target\wasm32-unknown-unknown\debug\rust-sdl2-webgl.wasm --out-dir ../wasm-output --no-modules --no-typescript
pause