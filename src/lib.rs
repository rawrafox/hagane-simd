#![feature(repr_simd, platform_intrinsics, associated_consts)]

#![allow(non_camel_case_types)]

#[macro_use] mod macros;

pub mod common;
pub mod matrix;
pub mod objc;
pub mod vector;
pub mod scalar;

pub use common::*;
pub use matrix::*;
pub use vector::*;
