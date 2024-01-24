#![feature(core_intrinsics)]
use std::intrinsics::size_of;

mod en;
mod error;
mod ser;

pub const USIZE_SIZE: usize = size_of::<usize>();

fn main() {
    println!("Hello, world!");
}
