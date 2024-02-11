#![feature(core_intrinsics)]
use std::intrinsics::size_of;

pub mod de;
pub mod descriptor;
pub mod ser;
mod integration;

pub const USIZE_SIZE: usize = size_of::<usize>();
