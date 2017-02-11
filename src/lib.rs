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
  extern "platform-intrinsic" {
    // fn simd_add<T>(x: T, y: T) -> T;
    // fn simd_sub<T>(x: T, y: T) -> T;
    // fn simd_mul<T>(x: T, y: T) -> T;
    // fn simd_div<T>(x: T, y: T) -> T;
    // 
    // fn simd_shl<T>(x: T, y: T) -> T;
    // fn simd_shr<T>(x: T, y: T) -> T;
    // 
    // fn simd_and<T>(x: T, y: T) -> T;
    // fn simd_or<T>(x: T, y: T) -> T;
    // fn simd_xor<T>(x: T, y: T) -> T;

    fn simd_eq<T, U>(x: T, y: T) -> U;
    fn simd_ne<T, U>(x: T, y: T) -> U;
    fn simd_lt<T, U>(x: T, y: T) -> U;
    fn simd_le<T, U>(x: T, y: T) -> U;
    fn simd_gt<T, U>(x: T, y: T) -> U;
    fn simd_ge<T, U>(x: T, y: T) -> U;

    // fn simd_cast<T, U>(x: T) -> U;

    fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
    fn simd_extract<T, E>(x: T, i: u32) -> E;
  }

  pub trait Vector : Sized {
    type Scalar;
    type Boolean;

    fn extract(self, i: u32) -> Self::Scalar {
      return unsafe { simd_extract(self, i) };
    }

    fn replace(self, i: u32, value: Self::Scalar) -> Self {
      return unsafe { simd_insert(self, i, value) };
    }

    fn eq(self, other: Self) -> Self::Boolean {
      return unsafe { simd_eq(self, other) };
    }

    fn ne(self, other: Self) -> Self::Boolean {
      return unsafe { simd_ne(self, other) };
    }

    fn lt(self, other: Self) -> Self::Boolean {
      return unsafe { simd_lt(self, other) };
    }

    fn le(self, other: Self) -> Self::Boolean {
      return unsafe { simd_le(self, other) };
    }

    fn gt(self, other: Self) -> Self::Boolean {
      return unsafe { simd_gt(self, other) };
    }

    fn ge(self, other: Self) -> Self::Boolean {
      return unsafe { simd_ge(self, other) };
    }

    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;

    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self  {
      return self.max(min).min(max)
    }
  }

  #[inline(always)]
  pub fn extract<T: Vector>(x: T, i: u32) -> T::Scalar {
    return x.extract(i);
  }

  #[inline(always)]
  pub fn replace<T: Vector>(x: T, i: u32, value: T::Scalar) -> T {
    return x.replace(i, value);
  }

  #[inline(always)]
  pub fn eq<T: Vector>(x: T, y: T) -> T::Boolean {
    return x.eq(y);
  }

  #[inline(always)]
  pub fn ne<T: Vector>(x: T, y: T) -> T::Boolean {
    return x.ne(y);
  }

  #[inline(always)]
  pub fn lt<T: Vector>(x: T, y: T) -> T::Boolean {
    return x.lt(y);
  }

  #[inline(always)]
  pub fn le<T: Vector>(x: T, y: T) -> T::Boolean {
    return x.le(y);
  }

  #[inline(always)]
  pub fn gt<T: Vector>(x: T, y: T) -> T::Boolean {
    return x.gt(y);
  }

  #[inline(always)]
  pub fn ge<T: Vector>(x: T, y: T) -> T::Boolean {
    return x.ge(y);
  }

  #[inline(always)]
  pub fn abs<T: Vector>(x: T) -> T {
    return x.abs();
  }

  #[inline(always)]
  pub fn max<T: Vector>(x: T, y: T) -> T {
    return x.max(y);
  }

  #[inline(always)]
  pub fn min<T: Vector>(x: T, y: T) -> T {
    return x.min(y);
  }

  #[inline(always)]
  pub fn clamp<T: Vector>(t: T, min: T, max: T) -> T {
    return t.clamp(min, max);
  }

  pub trait Cross : Vector {
    type Output;

    fn cross(self, rhs: Self) -> Self::Output;
  }

  #[inline(always)]
  pub fn cross<T: Cross>(x: T, y: T) -> T::Output {
    return x.cross(y);
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
    fn copysign(self, magnitude: Self) -> Self;
    fn sign(self) -> Self;

    fn sqrt(self) -> Self;

    fn recip(self) -> Self;
    fn rsqrt(self) -> Self;

    fn fract(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn trunc(self) -> Self;

    fn mix(self, a: Self, b: Self) -> Self;
    fn step(self, a: Self) -> Self;
    fn smoothstep(self, a: Self, b: Self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
  }

  #[inline(always)]
  pub fn copysign<T: Float>(sign: T, magnitude: T) -> T {
    return sign.copysign(magnitude);
  }

  #[inline(always)]
  pub fn sign<T: Float>(x: T) -> T {
    return x.sign();
  }

  #[inline(always)]
  pub fn sqrt<T: Float>(x: T) -> T {
    return x.sqrt();
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
  pub fn ceil<T: Float>(x: T) -> T {
    return x.ceil();
  }

  #[inline(always)]
  pub fn floor<T: Float>(x: T) -> T {
    return x.floor();
  }

  #[inline(always)]
  pub fn trunc<T: Float>(x: T) -> T {
    return x.trunc();
  }

  #[inline(always)]
  pub fn mix<T: Float>(t: T, a: T, b: T) -> T {
    return t.mix(a, b);
  }

  #[inline(always)]
  pub fn step<T: Float>(t: T, a: T) -> T {
    return t.step(a);
  }

  #[inline(always)]
  pub fn smoothstep<T: Float>(t: T, a: T, b: T) -> T {
    return t.smoothstep(a, b);
  }

  #[inline(always)]
  pub fn sin<T: Float>(x: T) -> T {
    return x.sin();
  }

  #[inline(always)]
  pub fn cos<T: Float>(x: T) -> T {
    return x.cos();
  }

  pub trait Geometry : Vector {
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

  pub trait Reduce : Vector {
    fn reduce_add(self) -> Self::Scalar;
    fn reduce_max(self) -> Self::Scalar;
    fn reduce_min(self) -> Self::Scalar;
  }

  #[inline(always)]
  pub fn reduce_add<T: Reduce>(x: T) -> T::Scalar {
    return x.reduce_add();
  }

  #[inline(always)]
  pub fn reduce_max<T: Reduce>(x: T) -> T::Scalar {
    return x.reduce_max();
  }

  #[inline(always)]
  pub fn reduce_min<T: Reduce>(x: T) -> T::Scalar {
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
