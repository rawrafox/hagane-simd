use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct int3(pub i32, pub i32, pub i32);

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

impl std::ops::Index<u32> for int3 {
  type Output = i32;

  #[inline]
  fn index(&self, index: u32) -> &i32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for int3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int3::broadcast(other)) };
  }
}

impl std::ops::Add<int3> for i32 {
  type Output = int3;

  #[inline]
  fn add(self, other: int3) -> int3 {
    return unsafe { simd_add(int3::broadcast(self), other) };
  }
}

impl std::ops::Sub for int3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int3::broadcast(other)) };
  }
}

impl std::ops::Sub<int3> for i32 {
  type Output = int3;

  #[inline]
  fn sub(self, other: int3) -> int3 {
    return unsafe { simd_sub(int3::broadcast(self), other) };
  }
}

impl std::ops::Mul for int3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int3::broadcast(other)) };
  }
}

impl std::ops::Mul<int3> for i32 {
  type Output = int3;

  #[inline]
  fn mul(self, other: int3) -> int3 {
    return unsafe { simd_mul(int3::broadcast(self), other) };
  }
}

impl std::ops::Div for int3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int3::broadcast(other)) };
  }
}

impl std::ops::Div<int3> for i32 {
  type Output = int3;

  #[inline]
  fn div(self, other: int3) -> int3 {
    return unsafe { simd_div(int3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for int3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i32> for int3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i32) -> Self {
    return unsafe { simd_and(self, int3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<int3> for i32 {
  type Output = int3;

  #[inline]
  fn bitand(self, other: int3) -> int3 {
    return unsafe { simd_and(int3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for int3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i32> for int3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i32) -> Self {
    return unsafe { simd_or(self, int3::broadcast(other)) };
  }
}

impl std::ops::BitOr<int3> for i32 {
  type Output = int3;

  #[inline]
  fn bitor(self, other: int3) -> int3 {
    return unsafe { simd_or(int3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for int3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i32> for int3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i32) -> Self {
    return unsafe { simd_xor(self, int3::broadcast(other)) };
  }
}

impl std::ops::BitXor<int3> for i32 {
  type Output = int3;

  #[inline]
  fn bitxor(self, other: int3) -> int3 {
    return unsafe { simd_xor(int3::broadcast(self), other) };
  }
}

impl std::ops::Shl<int3> for int3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i32> for int3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i32) -> Self {
    return unsafe { simd_shl(self, int3::broadcast(other)) };
  }
}

impl std::ops::Shl<int3> for i32 {
  type Output = int3;

  #[inline]
  fn shl(self, other: int3) -> int3 {
    return unsafe { simd_shl(int3::broadcast(self), other) };
  }
}

impl std::ops::Shr<int3> for int3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i32> for int3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i32) -> Self {
    return unsafe { simd_shr(self, int3::broadcast(other)) };
  }
}

impl std::ops::Shr<int3> for i32 {
  type Output = int3;

  #[inline]
  fn shr(self, other: int3) -> int3 {
    return unsafe { simd_shr(int3::broadcast(self), other) };
  }
}

impl std::ops::Not for int3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for int3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for int3 {
  type Scalar = i32;
  type Boolean = int3;

  #[inline(always)]
  fn extract(self, i: u32) -> Self::Scalar {
    return unsafe { simd_extract(self, i) };
  }

  #[inline(always)]
  fn replace(self, i: u32, x: Self::Scalar) -> Self {
    return unsafe { simd_insert(self, i, x) };
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

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 31;
    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return simd::bitselect(simd::gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return simd::bitselect(simd::lt(other, self), self, other);
  }
}

impl simd::Dot for int3 {
  type Output = i32;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for int3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & std::i32::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & std::i32::MIN != 0;
  }
}

impl simd::Reduce for int3 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(simd::reduce_min(self.lo()), self.2);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(simd::reduce_max(self.lo()), self.2);
  }
}

impl simd::Select<int3> for int3 {
  #[inline(always)]
  fn select(self, a: int3, b: int3) -> int3 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int3, b: int3) -> int3 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uint3> for int3 {
  #[inline(always)]
  fn select(self, a: uint3, b: uint3) -> uint3 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint3, b: uint3) -> uint3 {
    return uint3::bitcast(self.bitselect(int3::bitcast(a), int3::bitcast(b)));
  }
}

impl simd::Select<float3> for int3 {
  #[inline(always)]
  fn select(self, a: float3, b: float3) -> float3 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float3, b: float3) -> float3 {
    return float3::bitcast(self.bitselect(int3::bitcast(a), int3::bitcast(b)));
  }
}

impl int3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i32) -> Self {
    return int3(x, x, x);
  }

  #[inline]
  pub fn madd(x: int3, y: int3, z: int3) -> int3 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: int3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: int3) -> char3 {
    return int3::to_char(simd::clamp(x, int3::broadcast(std::i8::MIN as i32), int3::broadcast(std::i8::MAX as i32)));
  }

  #[inline]
  pub fn to_uchar(x: int3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: int3) -> uchar3 {
    return int3::to_uchar(simd::clamp(x, int3::broadcast(std::u8::MIN as i32), int3::broadcast(std::u8::MAX as i32)));
  }

  #[inline]
  pub fn to_short(x: int3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: int3) -> short3 {
    return int3::to_short(simd::clamp(x, int3::broadcast(std::i16::MIN as i32), int3::broadcast(std::i16::MAX as i32)));
  }

  #[inline]
  pub fn to_ushort(x: int3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: int3) -> ushort3 {
    return int3::to_ushort(simd::clamp(x, int3::broadcast(std::u16::MIN as i32), int3::broadcast(std::u16::MAX as i32)));
  }

  #[inline]
  pub fn to_int(x: int3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: int3) -> int3 {
    return x;
  }

  #[inline]
  pub fn to_uint(x: int3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: int3) -> uint3 {
    return int3::to_uint(simd::max(x, int3::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: int3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: int3) -> long3 {
    return long3(x.0 as i64, x.1 as i64, x.2 as i64);
  }

  #[inline]
  pub fn to_long_sat(x: int3) -> long3 {
    return int3::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: int3) -> ulong3 {
    return ulong3(x.0 as u64, x.1 as u64, x.2 as u64);
  }

  #[inline]
  pub fn to_ulong_sat(x: int3) -> ulong3 {
    return int3::to_ulong(simd::max(x, int3::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: int3) -> double3 {
    return double3(x.0 as f64, x.1 as f64, x.2 as f64);
  }

  #[inline]
  pub fn lo(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> int2 {
    return int2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> int2 {
    return int2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> int2 {
    return int2(self.0, self.2);
  }
}
