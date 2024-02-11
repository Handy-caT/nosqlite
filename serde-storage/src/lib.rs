#![feature(core_intrinsics)]
use std::intrinsics::size_of;

pub mod error;
pub mod ser;
mod de;
pub mod descriptor;

pub const USIZE_SIZE: usize = size_of::<usize>();
