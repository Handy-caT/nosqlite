#![feature(core_intrinsics)]
use std::intrinsics::size_of;

pub mod de;
pub mod descriptor;
mod integration;
pub mod ser;

pub const USIZE_SIZE: usize = size_of::<usize>();
