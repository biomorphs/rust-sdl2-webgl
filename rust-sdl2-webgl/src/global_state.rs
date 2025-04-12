// we need to store global state somehow for wasm (to avoid passing from rust -> js and back)
// we can use a global static singleton protected by a mutex to achieve this
// see https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use std::sync::Mutex;

pub struct GlobalState {
    pub bg_red: f32
}

pub static GLOBALS: Mutex<GlobalState> = Mutex::new(
    GlobalState { bg_red: 0.0 }
);