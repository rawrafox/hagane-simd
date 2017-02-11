use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ulong2(pub u64, pub u64);

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

impl std::ops::Index<u32> for ulong2 {
  type Output = u64;

  #[inline]
  fn index(&self, index: u32) -> &u64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ulong2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u64) -> Self {
    return unsafe { simd_add(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Add<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn add(self, other: ulong2) -> ulong2 {
    return unsafe { simd_add(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Sub for ulong2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u64) -> Self {
    return unsafe { simd_sub(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Sub<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn sub(self, other: ulong2) -> ulong2 {
    return unsafe { simd_sub(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Mul for ulong2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u64) -> Self {
    return unsafe { simd_mul(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Mul<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn mul(self, other: ulong2) -> ulong2 {
    return unsafe { simd_mul(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Div for ulong2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u64) -> Self {
    return unsafe { simd_div(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Div<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn div(self, other: ulong2) -> ulong2 {
    return unsafe { simd_div(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ulong2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u64) -> Self {
    return unsafe { simd_and(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitand(self, other: ulong2) -> ulong2 {
    return unsafe { simd_and(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ulong2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u64) -> Self {
    return unsafe { simd_or(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitOr<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitor(self, other: ulong2) -> ulong2 {
    return unsafe { simd_or(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ulong2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u64) -> Self {
    return unsafe { simd_xor(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitXor<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitxor(self, other: ulong2) -> ulong2 {
    return unsafe { simd_xor(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Shl<ulong2> for ulong2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u64) -> Self {
    return unsafe { simd_shl(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Shl<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn shl(self, other: ulong2) -> ulong2 {
    return unsafe { simd_shl(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Shr<ulong2> for ulong2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u64) -> Self {
    return unsafe { simd_shr(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Shr<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn shr(self, other: ulong2) -> ulong2 {
    return unsafe { simd_shr(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Not for ulong2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u64::MAX;
  }
}

impl PartialEq for ulong2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for ulong2 {
  type Scalar = u64;
  type Boolean = long2;

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
    return self;
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

impl simd::Dot for ulong2 {
  type Output = u64;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for ulong2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & 0x8000000000000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & 0x8000000000000000 != 0;
  }
}

impl simd::Reduce for ulong2 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(self.0, self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(self.0, self.1);
  }
}

impl ulong2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u64) -> Self {
    return ulong2(x, x);
  }

  #[inline]
  pub fn madd(x: ulong2, y: ulong2, z: ulong2) -> ulong2 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: ulong2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: ulong2) -> char2 {
    return ulong2::to_char(simd::min(x, ulong2::broadcast(std::i8::MAX as u64)));
  }

  #[inline]
  pub fn to_uchar(x: ulong2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: ulong2) -> uchar2 {
    return ulong2::to_uchar(simd::min(x, ulong2::broadcast(std::u8::MAX as u64)));
  }

  #[inline]
  pub fn to_short(x: ulong2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: ulong2) -> short2 {
    return ulong2::to_short(simd::min(x, ulong2::broadcast(std::i16::MAX as u64)));
  }

  #[inline]
  pub fn to_ushort(x: ulong2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: ulong2) -> ushort2 {
    return ulong2::to_ushort(simd::min(x, ulong2::broadcast(std::u16::MAX as u64)));
  }

  #[inline]
  pub fn to_int(x: ulong2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: ulong2) -> int2 {
    return ulong2::to_int(simd::min(x, ulong2::broadcast(std::i32::MAX as u64)));
  }

  #[inline]
  pub fn to_uint(x: ulong2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: ulong2) -> uint2 {
    return ulong2::to_uint(simd::min(x, ulong2::broadcast(std::u32::MAX as u64)));
  }

  #[inline]
  pub fn to_float(x: ulong2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: ulong2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: ulong2) -> long2 {
    return ulong2::to_long(simd::min(x, ulong2::broadcast(std::i64::MAX as u64)));
  }

  #[inline]
  pub fn to_ulong(x: ulong2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: ulong2) -> ulong2 {
    return x;
  }

  #[inline]
  pub fn to_double(x: ulong2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> u64 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u64 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u64 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u64 {
    return self.0;
  }
}
