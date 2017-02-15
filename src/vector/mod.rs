use std;
use std::ops::*;

use ::*;
use scalar::{Scalar, FloatScalar, IntegerScalar};

mod vector_char2;
mod vector_char3;
mod vector_char4;
mod vector_char8;
mod vector_char16;

mod vector_uchar2;
mod vector_uchar3;
mod vector_uchar4;
mod vector_uchar8;
mod vector_uchar16;

mod vector_short2;
mod vector_short3;
mod vector_short4;
mod vector_short8;
mod vector_short16;

mod vector_ushort2;
mod vector_ushort3;
mod vector_ushort4;
mod vector_ushort8;
mod vector_ushort16;

mod vector_int2;
mod vector_int3;
mod vector_int4;
mod vector_int8;
mod vector_int16;

mod vector_uint2;
mod vector_uint3;
mod vector_uint4;
mod vector_uint8;
mod vector_uint16;

mod vector_float2;
mod vector_float3;
mod vector_float4;
mod vector_float8;
mod vector_float16;

mod vector_long2;
mod vector_long3;
mod vector_long4;
mod vector_long8;
mod vector_long16;

mod vector_ulong2;
mod vector_ulong3;
mod vector_ulong4;
mod vector_ulong8;
mod vector_ulong16;

mod vector_double2;
mod vector_double3;
mod vector_double4;
mod vector_double8;
mod vector_double16;

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

declare_vector!(char2, char3, char4, char8, char16, i8, signed);
declare_vector!(short2, short3, short4, short8, short16, i16, signed);
declare_vector!(int2, int3, int4, int8, int16, i32, signed);
declare_vector!(long2, long3, long4, long8, long16, i64, signed);

declare_vector!(uchar2, uchar3, uchar4, uchar8, uchar16, u8, unsigned);
declare_vector!(ushort2, ushort3, ushort4, ushort8, ushort16, u16, unsigned);
declare_vector!(uint2, uint3, uint4, uint8, uint16, u32, unsigned);
declare_vector!(ulong2, ulong3, ulong4, ulong8, ulong16, u64, unsigned);

// TODO: declare_vector!(half2, half3, half4, f16, float);
declare_vector!(float2, float3, float4, float8, float16, f32, float);
declare_vector!(double2, double3, double4, double8, double16, f64, float);

pub trait Vector : Sized + Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + Dot<Self, DotProduct=<Self as Vector>::Scalar> + From<isize> {
  type Scalar: scalar::Scalar + Into<Self>;
  type Boolean: Select<Self> + Vector;

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

  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self;
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self;

  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar;

  #[inline(always)]
  fn broadcast(x: Self::Scalar) -> Self {
    return x.into();
  }

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

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return self.map_binary(other, &Self::Scalar::max);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return self.map_binary(other, &Self::Scalar::min);
  }

  #[inline(always)]
  fn clamp(self, min: Self, max: Self) -> Self  {
    return self.max(min).min(max)
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.reduce(&Self::Scalar::add);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return self.reduce(&Self::Scalar::max);
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return self.reduce(&Self::Scalar::min);
  }

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
pub fn broadcast<S: Into<T>, T>(x: S) -> T {
  return x.into();
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

pub trait Float : Vector<Scalar=<Self as Float>::FloatScalar> {
  type FloatScalar: scalar::FloatScalar + Into<Self>;

  const SIGN_MASK: <<Self as Vector>::Boolean as Vector>::Scalar;

  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return Self::Boolean::broadcast(Self::SIGN_MASK).bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    return (self.eq(Self::from(0)) | self.ne(self)).bitselect(Self::from(1).copysign(self), Self::from(0));
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return self.map_unary(&Self::Scalar::sqrt);
  }

  #[inline(always)]
  fn recip(self) -> Self {
    return Self::from(1) / self;
  }

  #[inline(always)]
  fn rsqrt(self) -> Self {
    return self.sqrt().recip();
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return self.map_unary(&Self::Scalar::fract);
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return self.map_unary(&Self::Scalar::ceil);
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return self.map_unary(&Self::Scalar::floor);
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return self.map_unary(&Self::Scalar::trunc);
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a)
  }

  #[inline(always)]
  fn step(self, a: Self) -> Self {
    return self.lt(a).bitselect(Self::from(1), Self::from(0));
  }

  #[inline(always)]
  fn smoothstep(self, a: Self, b: Self) -> Self {
    let t = ((self - a) / (b - a)).clamp(Self::from(0), Self::from(1));

    return t * t * (Self::from(3) - Self::from(2) * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return self.map_unary(&Self::Scalar::sin);
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return self.map_unary(&Self::Scalar::cos);
  }
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

pub trait Geometry : Float {
  #[inline(always)]
  fn project(self, onto: Self) -> Self {
    return Self::broadcast(self.dot(onto) / onto.dot(onto)) * onto
  }

  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return Self::Scalar::sqrt(self.length_squared());
  }

  #[inline(always)]
  fn length_squared(self) -> Self::Scalar {
    return self.dot(self);
  }

  #[inline(always)]
  fn norm_one(self) -> Self::Scalar {
    return self.abs().reduce_add();
  }

  #[inline(always)]
  fn norm_inf(self) -> Self::Scalar {
    return self.abs().reduce_max();
  }

  #[inline(always)]
  fn distance(self, to: Self) -> Self::Scalar {
    return (self - to).length();
  }

  #[inline(always)]
  fn distance_squared(self, to: Self) -> Self::Scalar {
    return (self - to).length_squared();
  }

  #[inline(always)]
  fn normalize(self) -> Self {
    return self * Self::broadcast(self.length_squared());
  }

  #[inline(always)]
  fn reflect(self, n: Self) -> Self {
    return self - Self::from(2) * Self::broadcast(self.dot(n)) * n;
  }

  #[inline(always)]
  fn refract(self, n: Self, eta: Self::Scalar) -> Self {
    let dp = Self::broadcast(self.dot(n));
    let k = Self::from(1) - Self::broadcast(eta * eta) * (Self::from(1) - dp * dp);

    return k.ge(Self::from(0)).bitselect(Self::broadcast(eta) * self - Self::broadcast(eta) * dp + k.sqrt(), Self::from(0));
  }
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

pub trait Integer : Vector<Scalar=<Self as Integer>::IntegerScalar> + BitAnd<Output=Self> + BitOr<Output=Self> + BitXor<Output=Self> {
  type IntegerScalar: scalar::IntegerScalar + Into<Self>;

  const SIGN_MASK: Self::Scalar;

  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.reduce(&Self::Scalar::bitand);
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.reduce(&Self::Scalar::bitor);
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.reduce(&Self::Scalar::bitxor);
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & Self::SIGN_MASK != Self::Scalar::ZERO;
  }
  
  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & Self::SIGN_MASK != Self::Scalar::ZERO;
  }
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
