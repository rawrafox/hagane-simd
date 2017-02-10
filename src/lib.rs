#![feature(repr_simd, platform_intrinsics)]

#![allow(non_camel_case_types)]

#[path = "types/type_char1.rs"] mod type_char1;
#[path = "types/type_char2.rs"] mod type_char2;
#[path = "types/type_char3.rs"] mod type_char3;
#[path = "types/type_char4.rs"] mod type_char4;

#[path = "types/type_uchar1.rs"] mod type_uchar1;
#[path = "types/type_uchar2.rs"] mod type_uchar2;
#[path = "types/type_uchar3.rs"] mod type_uchar3;
#[path = "types/type_uchar4.rs"] mod type_uchar4;

#[path = "types/type_short1.rs"] mod type_short1;
#[path = "types/type_short2.rs"] mod type_short2;
#[path = "types/type_short3.rs"] mod type_short3;
#[path = "types/type_short4.rs"] mod type_short4;

#[path = "types/type_ushort1.rs"] mod type_ushort1;
#[path = "types/type_ushort2.rs"] mod type_ushort2;
#[path = "types/type_ushort3.rs"] mod type_ushort3;
#[path = "types/type_ushort4.rs"] mod type_ushort4;

#[path = "types/type_int1.rs"] mod type_int1;
#[path = "types/type_int2.rs"] mod type_int2;
#[path = "types/type_int3.rs"] mod type_int3;
#[path = "types/type_int4.rs"] mod type_int4;

#[path = "types/type_uint1.rs"] mod type_uint1;
#[path = "types/type_uint2.rs"] mod type_uint2;
#[path = "types/type_uint3.rs"] mod type_uint3;
#[path = "types/type_uint4.rs"] mod type_uint4;

#[path = "types/type_float1.rs"] mod type_float1;
#[path = "types/type_float2.rs"] mod type_float2;
#[path = "types/type_float3.rs"] mod type_float3;
#[path = "types/type_float4.rs"] mod type_float4;

#[path = "types/type_long1.rs"] mod type_long1;
#[path = "types/type_long2.rs"] mod type_long2;
#[path = "types/type_long3.rs"] mod type_long3;
#[path = "types/type_long4.rs"] mod type_long4;

#[path = "types/type_ulong1.rs"] mod type_ulong1;
#[path = "types/type_ulong2.rs"] mod type_ulong2;
#[path = "types/type_ulong3.rs"] mod type_ulong3;
#[path = "types/type_ulong4.rs"] mod type_ulong4;

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

pub use type_char1::*;
pub use type_char2::*;
pub use type_char3::*;
pub use type_char4::*;

pub use type_uchar1::*;
pub use type_uchar2::*;
pub use type_uchar3::*;
pub use type_uchar4::*;

pub use type_short1::*;
pub use type_short2::*;
pub use type_short3::*;
pub use type_short4::*;

pub use type_ushort1::*;
pub use type_ushort2::*;
pub use type_ushort3::*;
pub use type_ushort4::*;

pub use type_int1::*;
pub use type_int2::*;
pub use type_int3::*;
pub use type_int4::*;

pub use type_uint1::*;
pub use type_uint2::*;
pub use type_uint3::*;
pub use type_uint4::*;

pub use type_float1::*;
pub use type_float2::*;
pub use type_float3::*;
pub use type_float4::*;

pub use type_long1::*;
pub use type_long2::*;
pub use type_long3::*;
pub use type_long4::*;

pub use type_ulong1::*;
pub use type_ulong2::*;
pub use type_ulong3::*;
pub use type_ulong4::*;

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
