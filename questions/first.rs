#![allow(dead_code)]

fn first<'a>(_x: &str, y: &'a str) -> &'a str {
    y
}
