#![allow(clippy::infallible_destructuring_match)]
#![allow(dead_code)]
#![feature(test)]
#![allow(unused_must_use)]

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!("Hello, world!");
}
