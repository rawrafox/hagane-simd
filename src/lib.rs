#![feature(repr_simd, platform_intrinsics)]

#![allow(non_camel_case_types)]

#[path = "types/type_int1.rs"] mod type_int1;
#[path = "types/type_int2.rs"] mod type_int2;
#[path = "types/type_int3.rs"] mod type_int3;
#[path = "types/type_int4.rs"] mod type_int4;

#[path = "types/type_float1.rs"] mod type_float1;
#[path = "types/type_float2.rs"] mod type_float2;
#[path = "types/type_float3.rs"] mod type_float3;
#[path = "types/type_float4.rs"] mod type_float4;

#[path = "types/type_long1.rs"] mod type_long1;
#[path = "types/type_long2.rs"] mod type_long2;
#[path = "types/type_long3.rs"] mod type_long3;
#[path = "types/type_long4.rs"] mod type_long4;

#[path = "types/type_double1.rs"] mod type_double1;
#[path = "types/type_double2.rs"] mod type_double2;
#[path = "types/type_double3.rs"] mod type_double3;
#[path = "types/type_double4.rs"] mod type_double4;

#[path = "types/type_float2x2.rs"] mod type_float2x2;
#[path = "types/type_float3x2.rs"] mod type_float3x2;
#[path = "types/type_float4x2.rs"] mod type_float4x2;
#[path = "types/type_float2x3.rs"] mod type_float2x3;
#[path = "types/type_float3x3.rs"] mod type_float3x3;
#[path = "types/type_float4x3.rs"] mod type_float4x3;
#[path = "types/type_float2x4.rs"] mod type_float2x4;
#[path = "types/type_float3x4.rs"] mod type_float3x4;
#[path = "types/type_float4x4.rs"] mod type_float4x4;

#[path = "types/type_double2x2.rs"] mod type_double2x2;
#[path = "types/type_double3x2.rs"] mod type_double3x2;
#[path = "types/type_double4x2.rs"] mod type_double4x2;
#[path = "types/type_double2x3.rs"] mod type_double2x3;
#[path = "types/type_double3x3.rs"] mod type_double3x3;
#[path = "types/type_double4x3.rs"] mod type_double4x3;
#[path = "types/type_double2x4.rs"] mod type_double2x4;
#[path = "types/type_double3x4.rs"] mod type_double3x4;
#[path = "types/type_double4x4.rs"] mod type_double4x4;

pub use type_int1::*;
pub use type_int2::*;
pub use type_int3::*;
pub use type_int4::*;

pub use type_float1::*;
pub use type_float2::*;
pub use type_float3::*;
pub use type_float4::*;

pub use type_long1::*;
pub use type_long2::*;
pub use type_long3::*;
pub use type_long4::*;

pub use type_double1::*;
pub use type_double2::*;
pub use type_double3::*;
pub use type_double4::*;

pub use type_float2x2::*;
pub use type_float3x2::*;
pub use type_float4x2::*;
pub use type_float2x3::*;
pub use type_float3x3::*;
pub use type_float4x3::*;
pub use type_float2x4::*;
pub use type_float3x4::*;
pub use type_float4x4::*;

pub use type_double2x2::*;
pub use type_double3x2::*;
pub use type_double4x2::*;
pub use type_double2x3::*;
pub use type_double3x3::*;
pub use type_double4x3::*;
pub use type_double2x4::*;
pub use type_double3x4::*;
pub use type_double4x4::*;

pub trait Dot<RHS = Self> {
  type Output;

  fn dot(self, rhs: RHS) -> Self::Output;
}
