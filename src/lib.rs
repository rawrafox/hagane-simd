#![feature(associated_consts, cfg_target_feature, link_llvm_intrinsics, platform_intrinsics, repr_simd, simd_ffi)]

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
