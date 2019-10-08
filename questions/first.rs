// Will you join in the game of life...times or death!!
//
// Rust's borrow checker (BC) can be your best friend and your worst
// enemy in the same five minuets. We will start out with a simple
// problem, lets help the BC figure out which two &str's need
// to have the same lifetime.

fn return_y(_x: &str, y: &str) -> &str {
    y
}

fn main() {}
