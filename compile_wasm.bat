cd rust-sdl2-webgl
cargo build --release --target wasm32-unknown-unknown --no-default-features --features "webgl"
wasm-bindgen target\wasm32-unknown-unknown\release\rust-sdl2-webgl.wasm --out-dir ../wasm-output --no-modules --no-typescript
pause