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

pub mod objc;

pub mod simd {
  pub trait Vector : Sized {

  }

  pub trait Common : Vector {
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;

    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self  {
      return self.max(min).min(max)
    }
  }

  #[inline(always)]
  pub fn abs<T: Common>(x: T) -> T {
    return x.abs();
  }

  #[inline(always)]
  pub fn max<T: Common>(x: T, y: T) -> T {
    return x.max(y);
  }

  #[inline(always)]
  pub fn min<T: Common>(x: T, y: T) -> T {
    return x.min(y);
  }

  #[inline(always)]
  pub fn clamp<T: Common>(t: T, min: T, max: T) -> T {
    return t.clamp(min, max);
  }

  pub trait Cross : Vector {
    type Output;

    fn cross(self, rhs: Self) -> Self::Output;
  }

  pub trait Dot<RHS = Self> {
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
  }

  #[inline(always)]
  pub fn dot<RHS, T: Dot<RHS>>(x: T, y: RHS) -> T::Output {
    return x.dot(y);
  }

  pub trait Float : Vector {
    fn sign(self) -> Self;
    fn mix(self, a: Self, b: Self) -> Self;
    fn recip(self) -> Self;
    fn rsqrt(self) -> Self;
    fn fract(self) -> Self;
    fn step(self, a: Self) -> Self;
    fn smoothstep(self, a: Self, b: Self) -> Self;
  }

  #[inline(always)]
  pub fn sign<T: Float>(x: T) -> T {
    return x.sign();
  }

  #[inline(always)]
  pub fn mix<T: Float>(t: T, a: T, b: T) -> T {
    return t.mix(a, b);
  }

  #[inline(always)]
  pub fn recip<T: Float>(x: T) -> T {
    return x.recip();
  }

  #[inline(always)]
  pub fn rsqrt<T: Float>(x: T) -> T {
    return x.rsqrt();
  }

  #[inline(always)]
  pub fn fract<T: Float>(x: T) -> T {
    return x.fract();
  }

  #[inline(always)]
  pub fn step<T: Float>(t: T, a: T) -> T {
    return t.step(a);
  }

  #[inline(always)]
  pub fn smoothstep<T: Float>(t: T, a: T, b: T) -> T {
    return t.smoothstep(a, b);
  }

  pub trait Geometry : Vector {
    type Scalar;

    fn project(self, onto: Self) -> Self;
    fn length(self) -> Self::Scalar;
    fn length_squared(self) -> Self::Scalar;
    fn norm_one(self) -> Self::Scalar;
    fn norm_inf(self) -> Self::Scalar;
    fn distance(self, to: Self) -> Self::Scalar;
    fn distance_squared(self, to: Self) -> Self::Scalar;
    fn normalize(self) -> Self;
    fn reflect(self, n: Self) -> Self;
    fn refract(self, n: Self, eta: Self::Scalar) -> Self;
  }

  #[inline(always)]
  pub fn project<T: Geometry>(x: T, onto: T) -> T {
    return x.project(onto);
  }

  #[inline(always)]
  pub fn length<T: Geometry>(x: T) -> T::Scalar {
    return x.length();
  }

  #[inline(always)]
  pub fn length_squared<T: Geometry>(x: T) -> T::Scalar {
    return x.length_squared();
  }

  #[inline(always)]
  pub fn norm_one<T: Geometry>(x: T) -> T::Scalar {
    return x.norm_one();
  }

  #[inline(always)]
  pub fn norm_inf<T: Geometry>(x: T) -> T::Scalar {
    return x.norm_inf();
  }

  #[inline(always)]
  pub fn distance<T: Geometry>(x: T, y: T) -> T::Scalar {
    return x.distance(y);
  }

  #[inline(always)]
  pub fn distance_squared<T: Geometry>(x: T, y: T) -> T::Scalar {
    return x.distance_squared(y);
  }

  #[inline(always)]
  pub fn normalize<T: Geometry>(x: T) -> T {
    return x.normalize();
  }

  #[inline(always)]
  pub fn reflect<T: Geometry>(x: T, n: T) -> T {
    return x.reflect(n);
  }

  #[inline(always)]
  pub fn refract<T: Geometry>(x: T, n: T, eta: T::Scalar) -> T {
    return x.refract(n, eta);
  }

  pub trait Logic : Vector {
    fn all(self) -> bool;
    fn any(self) -> bool;
  }

  #[inline(always)]
  pub fn all<T: Logic>(x: T) -> bool {
    return x.all();
  }

  #[inline(always)]
  pub fn any<T: Logic>(x: T) -> bool {
    return x.any();
  }

  pub trait Reduction : Vector {
    type Output;

    fn reduce_add(self) -> Self::Output;
    fn reduce_max(self) -> Self::Output;
    fn reduce_min(self) -> Self::Output;
  }

  #[inline(always)]
  pub fn reduce_add<T: Reduction>(x: T) -> T::Output {
    return x.reduce_add();
  }

  #[inline(always)]
  pub fn reduce_max<T: Reduction>(x: T) -> T::Output {
    return x.reduce_max();
  }

  #[inline(always)]
  pub fn reduce_min<T: Reduction>(x: T) -> T::Output {
    return x.reduce_min();
  }

  pub trait Select<T> : Logic {
    fn select(self, a: T, b: T) -> T;
    fn bitselect(self, a: T, b: T) -> T;
  }

  #[inline(always)]
  pub fn select<T, B: Select<T>>(condition: B, a: T, b: T) -> T {
    return condition.select(a, b);
  }

  #[inline(always)]
  pub fn bitselect<T, B: Select<T>>(condition: B, a: T, b: T) -> T {
    return condition.bitselect(a, b);
  }
}
