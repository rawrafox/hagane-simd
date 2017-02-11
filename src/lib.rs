#![feature(repr_simd, platform_intrinsics)]

#![allow(non_camel_case_types)]

use std::ops::*;

#[path = "impls/impl_char2.rs"] mod impl_char2;
#[path = "impls/impl_char3.rs"] mod impl_char3;
#[path = "impls/impl_char4.rs"] mod impl_char4;

#[path = "impls/impl_uchar2.rs"] mod impl_uchar2;
#[path = "impls/impl_uchar3.rs"] mod impl_uchar3;
#[path = "impls/impl_uchar4.rs"] mod impl_uchar4;

#[path = "impls/impl_short2.rs"] mod impl_short2;
#[path = "impls/impl_short3.rs"] mod impl_short3;
#[path = "impls/impl_short4.rs"] mod impl_short4;

#[path = "impls/impl_ushort2.rs"] mod impl_ushort2;
#[path = "impls/impl_ushort3.rs"] mod impl_ushort3;
#[path = "impls/impl_ushort4.rs"] mod impl_ushort4;

#[path = "impls/impl_int2.rs"] mod impl_int2;
#[path = "impls/impl_int3.rs"] mod impl_int3;
#[path = "impls/impl_int4.rs"] mod impl_int4;

#[path = "impls/impl_uint2.rs"] mod impl_uint2;
#[path = "impls/impl_uint3.rs"] mod impl_uint3;
#[path = "impls/impl_uint4.rs"] mod impl_uint4;

#[path = "impls/impl_float2.rs"] mod impl_float2;
#[path = "impls/impl_float3.rs"] mod impl_float3;
#[path = "impls/impl_float4.rs"] mod impl_float4;

#[path = "impls/impl_long2.rs"] mod impl_long2;
#[path = "impls/impl_long3.rs"] mod impl_long3;
#[path = "impls/impl_long4.rs"] mod impl_long4;

#[path = "impls/impl_ulong2.rs"] mod impl_ulong2;
#[path = "impls/impl_ulong3.rs"] mod impl_ulong3;
#[path = "impls/impl_ulong4.rs"] mod impl_ulong4;

#[path = "impls/impl_double2.rs"] mod impl_double2;
#[path = "impls/impl_double3.rs"] mod impl_double3;
#[path = "impls/impl_double4.rs"] mod impl_double4;

#[path = "impls/impl_float2x2.rs"] mod impl_float2x2;
#[path = "impls/impl_float3x2.rs"] mod impl_float3x2;
#[path = "impls/impl_float4x2.rs"] mod impl_float4x2;
#[path = "impls/impl_float2x3.rs"] mod impl_float2x3;
#[path = "impls/impl_float3x3.rs"] mod impl_float3x3;
#[path = "impls/impl_float4x3.rs"] mod impl_float4x3;
#[path = "impls/impl_float2x4.rs"] mod impl_float2x4;
#[path = "impls/impl_float3x4.rs"] mod impl_float3x4;
#[path = "impls/impl_float4x4.rs"] mod impl_float4x4;

#[path = "impls/impl_double2x2.rs"] mod impl_double2x2;
#[path = "impls/impl_double3x2.rs"] mod impl_double3x2;
#[path = "impls/impl_double4x2.rs"] mod impl_double4x2;
#[path = "impls/impl_double2x3.rs"] mod impl_double2x3;
#[path = "impls/impl_double3x3.rs"] mod impl_double3x3;
#[path = "impls/impl_double4x3.rs"] mod impl_double4x3;
#[path = "impls/impl_double2x4.rs"] mod impl_double2x4;
#[path = "impls/impl_double3x4.rs"] mod impl_double3x4;
#[path = "impls/impl_double4x4.rs"] mod impl_double4x4;

pub mod objc;

use simd::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;
}

macro_rules! declare_vector {
  ($name2:ident, $name3:ident, $name4:ident, $scalar:ident, $kind:ident) => (
    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name2(pub $scalar, pub $scalar);

    impl_vector!($name2, $scalar, $kind);

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name3(pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name3, $scalar, $kind);

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name4(pub $scalar, pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name4, $scalar, $kind);
  );
}

macro_rules! impl_trait {
  ($vector:ident, $scalar:ident, $intrinsic:ident, $trait_name:ident, $fn_name:ident) => {
    impl $trait_name<$vector> for $vector {
      type Output = Self;

      #[inline]
      fn $fn_name(self, other: Self) -> Self {
        return unsafe { $intrinsic(self, other) };
      }
    }

    impl $trait_name<$scalar> for $vector {
      type Output = Self;

      #[inline]
      fn $fn_name(self, other: $scalar) -> Self {
        return unsafe { $intrinsic(self, $vector::broadcast(other)) };
      }
    }

    impl $trait_name<$vector> for $scalar {
      type Output = $vector;

      #[inline]
      fn $fn_name(self, other: $vector) -> $vector {
        return unsafe { $intrinsic($vector::broadcast(self), other) };
      }
    }
  }
}

macro_rules! impl_vector {
  ($vector:ident, $scalar:ident, integer) => {
    impl_vector!($vector, $scalar, float);

    impl_trait!($vector, $scalar, simd_and, BitAnd, bitand);
    impl_trait!($vector, $scalar, simd_or, BitOr, bitor);
    impl_trait!($vector, $scalar, simd_xor, BitXor, bitxor);
    
    impl_trait!($vector, $scalar, simd_shl, Shl, shl);
    impl_trait!($vector, $scalar, simd_shr, Shr, shr);
    
  };
  ($vector:ident, $scalar:ident, signed) => {
    impl_vector!($vector, $scalar, integer);

    impl std::ops::Not for $vector {
      type Output = Self;

      #[inline]
      fn not(self) -> Self {
        return self ^ -1;
      }
    }
  };
  ($vector:ident, $scalar:ident, unsigned) => {
    impl_vector!($vector, $scalar, integer);

    impl std::ops::Not for $vector {
      type Output = Self;

      #[inline]
      fn not(self) -> Self {
        return self ^ std::$scalar::MAX;
      }
    }
  };
  ($vector:ident, $scalar:ident, float) => {
    impl_trait!($vector, $scalar, simd_add, Add, add);
    impl_trait!($vector, $scalar, simd_sub, Sub, sub);
    impl_trait!($vector, $scalar, simd_mul, Mul, mul);
    impl_trait!($vector, $scalar, simd_div, Div, div);

    impl PartialEq for $vector {
      #[inline]
      fn eq(&self, other: &Self) -> bool {
        return simd::eq(*self, *other).all();
      }

      #[inline]
      fn ne(&self, other: &Self) -> bool {
        return simd::ne(*self, *other).all();
      }
    }
  }
}

macro_rules! declare_matrix {
  ($name2:ident, $name3:ident, $name4:ident, $vector:ty) => (
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name2(pub $vector, pub $vector);

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name3(pub $vector, pub $vector, pub $vector);

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name4(pub $vector, pub $vector, pub $vector, pub $vector);
  );
}

declare_vector!(char2, char3, char4, i8, signed);
declare_vector!(short2, short3, short4, i16, signed);
declare_vector!(int2, int3, int4, i32, signed);
declare_vector!(long2, long3, long4, i64, signed);

declare_vector!(uchar2, uchar3, uchar4, u8, unsigned);
declare_vector!(ushort2, ushort3, ushort4, u16, unsigned);
declare_vector!(uint2, uint3, uint4, u32, unsigned);
declare_vector!(ulong2, ulong3, ulong4, u64, unsigned);

// TODO: declare_vector!(half2, half3, half4, f16, float);
declare_vector!(float2, float3, float4, f32, float);
declare_vector!(double2, double3, double4, f64, float);

declare_matrix!(float2x2, float3x2, float4x2, float2);
declare_matrix!(float2x3, float3x3, float4x3, float3);
declare_matrix!(float2x4, float3x4, float4x4, float4);

declare_matrix!(double2x2, double3x2, double4x2, double2);
declare_matrix!(double2x3, double3x3, double4x3, double3);
declare_matrix!(double2x4, double3x4, double4x4, double4);

pub mod simd {
  use std::ops::*;

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

    fn simd_cast<T, U>(x: T) -> U;

    fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
    fn simd_extract<T, E>(x: T, i: u32) -> E;
  }

  pub trait Vector : Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {
    type Scalar;
    type Boolean;

    type CharVector;
    type ShortVector;
    type IntVector;
    type LongVector;

    type UCharVector;
    type UShortVector;
    type UIntVector;
    type ULongVector;

    type FloatVector;
    type DoubleVector;

    #[inline(always)]
    fn extract(self, i: u32) -> Self::Scalar {
      return unsafe { simd_extract(self, i) };
    }

    #[inline(always)]
    fn replace(self, i: u32, value: Self::Scalar) -> Self {
      return unsafe { simd_insert(self, i, value) };
    }

    #[inline(always)]
    fn add_mul(self, a: Self, b: Self) -> Self {
      return a * b + self;
    }

    #[inline(always)]
    fn eq(self, other: Self) -> Self::Boolean {
      return unsafe { simd_eq(self, other) };
    }

    #[inline(always)]
    fn ne(self, other: Self) -> Self::Boolean {
      return unsafe { simd_ne(self, other) };
    }

    #[inline(always)]
    fn lt(self, other: Self) -> Self::Boolean {
      return unsafe { simd_lt(self, other) };
    }

    #[inline(always)]
    fn le(self, other: Self) -> Self::Boolean {
      return unsafe { simd_le(self, other) };
    }

    #[inline(always)]
    fn gt(self, other: Self) -> Self::Boolean {
      return unsafe { simd_gt(self, other) };
    }

    #[inline(always)]
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

    fn reduce_add(self) -> Self::Scalar;
    fn reduce_max(self) -> Self::Scalar;
    fn reduce_min(self) -> Self::Scalar;

    #[inline(always)]
    fn to_char(self) -> Self::CharVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_short(self) -> Self::ShortVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_int(self) -> Self::IntVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_long(self) -> Self::LongVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_uchar(self) -> Self::UCharVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_ushort(self) -> Self::UShortVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_uint(self) -> Self::UIntVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_ulong(self) -> Self::ULongVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_float(self) -> Self::FloatVector {
      return unsafe { simd_cast(self) };
    }

    #[inline(always)]
    fn to_double(self) -> Self::DoubleVector {
      return unsafe { simd_cast(self) };
    }

    fn to_char_sat(self) -> Self::CharVector;
    fn to_short_sat(self) -> Self::ShortVector;
    fn to_int_sat(self) -> Self::IntVector;
    fn to_long_sat(self) -> Self::LongVector;

    fn to_uchar_sat(self) -> Self::UCharVector;
    fn to_ushort_sat(self) -> Self::UShortVector;
    fn to_uint_sat(self) -> Self::UIntVector;
    fn to_ulong_sat(self) -> Self::ULongVector;
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

  #[inline(always)]
  pub fn reduce_add<T: Vector>(x: T) -> T::Scalar {
    return x.reduce_add();
  }

  #[inline(always)]
  pub fn reduce_max<T: Vector>(x: T) -> T::Scalar {
    return x.reduce_max();
  }

  #[inline(always)]
  pub fn reduce_min<T: Vector>(x: T) -> T::Scalar {
    return x.reduce_min();
  }

  #[inline(always)]
  pub fn to_char<T: Vector>(x: T) -> T::CharVector {
    return x.to_char();
  }

  #[inline(always)]
  pub fn to_short<T: Vector>(x: T) -> T::ShortVector {
    return x.to_short();
  }

  #[inline(always)]
  pub fn to_int<T: Vector>(x: T) -> T::IntVector {
    return x.to_int();
  }

  #[inline(always)]
  pub fn to_long<T: Vector>(x: T) -> T::LongVector {
    return x.to_long();
  }

  #[inline(always)]
  pub fn to_uchar<T: Vector>(x: T) -> T::UCharVector {
    return x.to_uchar();
  }

  #[inline(always)]
  pub fn to_ushort<T: Vector>(x: T) -> T::UShortVector {
    return x.to_ushort();
  }

  #[inline(always)]
  pub fn to_uint<T: Vector>(x: T) -> T::UIntVector {
    return x.to_uint();
  }

  #[inline(always)]
  pub fn to_ulong<T: Vector>(x: T) -> T::ULongVector {
    return x.to_ulong();
  }

  #[inline(always)]
  pub fn to_float<T: Vector>(x: T) -> T::FloatVector {
    return x.to_float();
  }

  #[inline(always)]
  pub fn to_double<T: Vector>(x: T) -> T::DoubleVector {
    return x.to_double();
  }

  #[inline(always)]
  pub fn to_char_sat<T: Vector>(x: T) -> T::CharVector {
    return x.to_char_sat();
  }

  #[inline(always)]
  pub fn to_short_sat<T: Vector>(x: T) -> T::ShortVector {
    return x.to_short_sat();
  }

  #[inline(always)]
  pub fn to_int_sat<T: Vector>(x: T) -> T::IntVector {
    return x.to_int_sat();
  }

  #[inline(always)]
  pub fn to_long_sat<T: Vector>(x: T) -> T::LongVector {
    return x.to_long_sat();
  }

  #[inline(always)]
  pub fn to_uchar_sat<T: Vector>(x: T) -> T::UCharVector {
    return x.to_uchar_sat();
  }

  #[inline(always)]
  pub fn to_ushort_sat<T: Vector>(x: T) -> T::UShortVector {
    return x.to_ushort_sat();
  }

  #[inline(always)]
  pub fn to_uint_sat<T: Vector>(x: T) -> T::UIntVector {
    return x.to_uint_sat();
  }

  #[inline(always)]
  pub fn to_ulong_sat<T: Vector>(x: T) -> T::ULongVector {
    return x.to_ulong_sat();
  }

  pub trait Cross : Vector {
    type CrossProduct;

    fn cross(self, rhs: Self) -> Self::CrossProduct;
  }

  #[inline(always)]
  pub fn cross<T: Cross>(x: T, y: T) -> T::CrossProduct {
    return x.cross(y);
  }

  pub trait Dot<RHS = Self> {
    type DotProduct;

    fn dot(self, rhs: RHS) -> Self::DotProduct;
  }

  #[inline(always)]
  pub fn dot<RHS, T: Dot<RHS>>(x: T, y: RHS) -> T::DotProduct {
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

  pub trait Integer : Vector + BitAnd<Output=Self> + BitOr<Output=Self> + BitXor<Output=Self> {
    fn reduce_and(self) -> Self::Scalar;
    fn reduce_or(self) -> Self::Scalar;
    fn reduce_xor(self) -> Self::Scalar;

    fn all(self) -> bool;
    fn any(self) -> bool;
  }

  #[inline(always)]
  pub fn reduce_and<T: Integer>(x: T) -> T::Scalar {
    return x.reduce_and();
  }

  #[inline(always)]
  pub fn reduce_or<T: Integer>(x: T) -> T::Scalar {
    return x.reduce_or();
  }

  #[inline(always)]
  pub fn reduce_xor<T: Integer>(x: T) -> T::Scalar {
    return x.reduce_xor();
  }

  #[inline(always)]
  pub fn all<T: Integer>(x: T) -> bool {
    return x.all();
  }

  #[inline(always)]
  pub fn any<T: Integer>(x: T) -> bool {
    return x.any();
  }

  pub trait Select<T> : Integer {
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
