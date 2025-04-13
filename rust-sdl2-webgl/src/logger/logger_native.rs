// just forward everything to println! for native code
macro_rules! console_log {
    ($($args: tt)*) => {
        println!($($args)*);
    }
}